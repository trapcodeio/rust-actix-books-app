mod env;
mod db;
mod controllers;
mod types;
mod models;

use actix_web::{App, HttpServer};
use actix_web::middleware::{Condition, Logger};
use actix_web::web::Data;

use crate::db::{connect_to_db};
use crate::env::{load_env};
use crate::controllers::{books_controller};
use crate::types::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!();
    // load env
    let env = load_env();
    let port = env.app_port;

    // connect to db
    let database = match connect_to_db(env.clone()).await {
        Ok(client) => client,
        Err(err) => panic!("{:?}", err)
    };

    println!("Server: [{}]", env.app_name);
    println!("Environment: [{}]", env.env);
    println!("Url: [http://localhost:{}]", port.clone());
    println!();


    if env.is_development() {
        // access logs are printed with the INFO level so ensure it is enabled by default
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    }


    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                database: database.clone(),
            }))
            .wrap(Condition::new(env.is_development(), Logger::new("%s - %r - [%D]")))
            .service(books_controller::index)
            .service(books_controller::all)
            .service(books_controller::create)
            .service(books_controller::view)
            .service(books_controller::update)
            .service(books_controller::delete)
            .service(books_controller::delete_all)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

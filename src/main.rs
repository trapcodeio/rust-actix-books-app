mod env;
mod db;
mod controllers;
mod types;
mod models;
mod extractors;

use actix_cors::Cors;
use actix_web::{App, error, HttpResponse, HttpServer};
use actix_web::middleware::{Condition, Logger};
use actix_web::web::{Data, JsonConfig};
use serde_json::json;

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
            .app_data(json_config_error_handler())
            .app_data(Data::new(AppState {
                database: database.clone(),
            }))
            .wrap(Condition::new(env.is_development(), Logger::new("%s - %r - [%D]")))
            .wrap(
                Cors::default()
                    .allow_any_origin() // You can customise this to only allow specific origins
                    .allow_any_method() // Allows all methods (GET, POST, DELETE, etc.)
                    .allow_any_header() // Allows all headers
                    .max_age(3600),     // Sets the maximum age for CORS preflight requests
            )
            .service(books_controller::index)
            .service(books_controller::all)
            .service(books_controller::create)
            .service(books_controller::view)
            .service(books_controller::update)
            .service(books_controller::delete)
            .service(books_controller::delete_all)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}


/// Json config error handler
fn json_config_error_handler() -> JsonConfig {
    JsonConfig::default()
        .error_handler(|err, _req| {
            // try to get the field name
            // the field name is surrounded by a backtick e.g `name`
            // using regex, we can get the field name
            let err_str = err.to_string();
            println!("{}", err_str);
            let re = regex::Regex::new(r"`(.*)`").unwrap();
            let field_name = match re.captures(err_str.as_str()) {
                Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
                None => "",
            };

            // create custom error response
            let response = HttpResponse::BadRequest().json(json!({
                "code": "bad_body",
                "message": format!("Missing field: `{}`", field_name),
                "field": field_name.to_string(),
            }));

            error::InternalError::from_response(err, response)
                .into()
        })
}
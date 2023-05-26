use std::error::Error;
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use crate::env::Env;

/// Connect to mongodb
/// This function returns a mongodb database
/// The database configured in the .env file is used
pub async fn connect_to_db(env: Env) -> Result<Database, Box<dyn Error>> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(env.mongodb_server).await?;

    // Manually set an option.
    client_options.app_name = Some(env.app_name);

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // print db name
    println!("Connected to Database: [{}]", env.mongodb_database);

    // return db
    Ok(client.database(env.mongodb_database.as_str()))
}
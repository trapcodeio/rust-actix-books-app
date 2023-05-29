use mongodb::Database;
use serde::{Deserialize, Serialize};

pub struct AppState {
   pub database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooksQuery {
   pub title: Option<String>,
}
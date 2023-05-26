use actix_web::{get, web, put, delete, post, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::models::book_model::{ApiBook, DbBook, convert_db_book_to_api_book, books_collection};
use crate::types::AppState;
use mongodb::bson::{doc};
use futures::stream::TryStreamExt;
// use mongodb::options::FindOptions;


#[get("/")]
pub async fn index() -> impl Responder {
    let response = serde_json::json!({
        "message": "Welcome to Actix Book App",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}

#[get("/books")]
pub async fn all(data: web::Data<AppState>) -> impl Responder {
    let collection = books_collection(&data.database);
    let mut cursor = collection.find(doc! {}, None).await.unwrap();

    let mut results: Vec<ApiBook> = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        results.push(convert_db_book_to_api_book(result));
    }

    web::Json(results)
}

#[post("/books")]
pub async fn create() -> impl Responder {
    let response = serde_json::json!({
        "message": "Create Book",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}

#[get("/books/{id}")]
pub async fn view() -> impl Responder {
    let response = serde_json::json!({
        "message": "Book by ID",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}

#[put("/books/{id}")]
pub async fn update() -> impl Responder {
    let response = serde_json::json!({
        "message": "Update Book by ID",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}

#[delete("/books/{id}")]
pub async fn delete() -> impl Responder {
    let response = serde_json::json!({
        "message": "Delete Book by ID",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}

#[delete("/books")]
pub async fn delete_all() -> impl Responder {
    let response = serde_json::json!({
        "message": "Delete All Books",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}



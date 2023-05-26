use actix_web::{get, web, put, delete, post, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::models::book_model::{ApiBook, convert_db_book_to_api_book, books_collection, BookForm, validate_book_form, DbBook};
use crate::types::AppState;
use mongodb::bson::{DateTime, doc};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use crate::extractors::BookIdExists;
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

    HttpResponse::Ok().json(results)
}

#[post("/books")]
pub async fn create(body: web::Json<BookForm>, data: web::Data<AppState>) -> impl Responder {
    // validate data
    match validate_book_form(&body) {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::BadRequest().json(json!({
                "message": err,
            }));
        }
    }

    let collection = books_collection(&data.database);

    // create book
    let book = DbBook {
        _id: ObjectId::new(),
        title: body.title.clone(),
        description: body.description.clone(),
        available: body.available.clone(),
        createdAt: DateTime::now(),
        updatedAt: DateTime::now(),
    };

    // insert book
    collection.insert_one(book, None).await.unwrap();

    HttpResponse::Ok().json(json!({"message": "Book created successfully"}))
}

#[get("/books/{id}")]
pub async fn view(book_id: BookIdExists, data: web::Data<AppState>) -> impl Responder {
    if !book_id.exists {
        return HttpResponse::NotFound().json(json!({
            "message": "Book not found",
        }));
    }

    let collection = books_collection(&data.database);
    let book = collection.find_one(doc! {"_id": book_id.id.unwrap()}, None).await.unwrap();

    // check if book exists
    if book.is_none() {
        return HttpResponse::NotFound().json(json!({
            "message": "Book not found",
        }));
    }

    HttpResponse::Ok()
        .json(convert_db_book_to_api_book(book.unwrap()))
}

#[put("/books/{id}")]
pub async fn update() -> impl Responder {
    let response = json!({
        "message": "Update Book by ID",
    });

    HttpResponse::Ok()
        .json(response)
}

#[delete("/books/{id}")]
pub async fn delete() -> impl Responder {
    let response = json!({
        "message": "Delete Book by ID",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}

#[delete("/books")]
pub async fn delete_all() -> impl Responder {
    let response = json!({
        "message": "Delete All Books",
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response.to_string())
}



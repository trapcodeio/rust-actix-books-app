use actix_web::{get, web, put, delete, post, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::models::book_model::{ApiBook, convert_db_book_to_api_book, books_collection, BookForm, validate_book_form, DbBook};
use crate::types::AppState;
use mongodb::bson::{DateTime, doc};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use crate::extractors::BookIdExists;


fn err_book_not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .json(json!({
            "error": "Book not found",
        }).to_string())
}


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
                "error": err,
            }));
        }
    }

    let collection = books_collection(&data.database);

    // create book
    let book = DbBook {
        _id: ObjectId::new(),
        title: body.title.clone(),
        description: body.description.clone(),
        available: body.available,
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
        return err_book_not_found();
    }

    let collection = books_collection(&data.database);
    let book = collection.find_one(doc! {"_id": book_id.id.unwrap()}, None).await.unwrap();

    // check if book exists
    if book.is_none() {
        return err_book_not_found();
    }

    HttpResponse::Ok()
        .json(convert_db_book_to_api_book(book.unwrap()))
}

#[put("/books/{id}")]
pub async fn update(book_id: BookIdExists, body: web::Json<BookForm>, data: web::Data<AppState>) -> impl Responder {
    if !book_id.exists {
        return err_book_not_found();
    }

    // validate data
    match validate_book_form(&body) {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::BadRequest().json(json!({
                "error": err,
            }));
        }
    }

    let collection = books_collection(&data.database);

    // update book
    let update = collection.update_one(
        doc! {"_id": book_id.id.unwrap()},
        doc! {"$set": {
            "title": body.title.clone(),
            "description": body.description.clone(),
            "available": body.available,
            "updatedAt": DateTime::now(),
        }},
        None,
    ).await.unwrap();

    if update.modified_count > 0 {
        HttpResponse::Ok().json(json!({"message": "Book updated successfully"}))
    } else {
        err_book_not_found()
    }
}

#[delete("/books/{id}")]
pub async fn delete(book_id: BookIdExists, data: web::Data<AppState>) -> impl Responder {
    if !book_id.exists {
        return err_book_not_found();
    }

    let collection = books_collection(&data.database);
    let deleted = collection.delete_one(doc! {"_id": book_id.id.unwrap()}, None).await.unwrap();

    if deleted.deleted_count > 0 {
        HttpResponse::Ok().json(json!({"message": "Book deleted successfully"}))
    } else {
        HttpResponse::Ok().json(json!({"message": "Book not found, or already deleted"}))
    }
}

#[delete("/books")]
pub async fn delete_all(data: web::Data<AppState>) -> impl Responder {
    let collection = books_collection(&data.database);

    let deleted = collection.delete_many(doc! {}, None).await.unwrap();

    HttpResponse::Ok().json(json!({"message": format!("{} books deleted successfully", deleted.deleted_count)}))
}



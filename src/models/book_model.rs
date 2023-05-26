use actix_web::web;
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DbBook {
    pub(crate) _id: ObjectId,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) available: bool,
    pub(crate) createdAt: DateTime,
    pub(crate) updatedAt: DateTime,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiBook {
    pub id: String,
    pub title: String,
    pub description: String,
    pub available: bool,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookForm {
    pub title: String,
    pub description: String,
    pub available: bool,
}

pub fn books_collection(db: &mongodb::Database)-> mongodb::Collection<DbBook>  {
    db.collection::<DbBook>("books")
}

pub fn convert_db_book_to_api_book(book: DbBook) -> ApiBook {
    ApiBook {
        id: book._id.to_hex(),
        title: book.title,
        description: book.description,
        available: book.available,
        updatedAt: book.updatedAt.try_to_rfc3339_string().unwrap(),
        createdAt: book.createdAt.try_to_rfc3339_string().unwrap(),
    }
}

pub fn validate_book_form(form: &web::Json<BookForm>) -> Result<bool, String> {
    // check if title is empty
    if form.title.is_empty() {
        return Err("Title is required".to_string());
    }

    // check if description is empty
    if form.description.is_empty() {
        return Err("Description is required".to_string());
    }

    Ok(true)
}
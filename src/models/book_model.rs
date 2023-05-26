use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DbBook {
    _id: ObjectId,
    title: String,
    description: String,
    available: bool,
    createdAt: DateTime,
    updatedAt: DateTime,
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
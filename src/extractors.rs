use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, error};

use futures::future::{ok, Ready, err};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use crate::models::book_model::books_collection;
use crate::types::AppState;

#[derive(Debug)]
pub struct BookIdExists {
    pub id: Option<ObjectId>,
    pub exists: bool,
}

impl FromRequest for BookIdExists {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    // type Config = ();

    fn  from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let book_id = req.match_info().get("id").unwrap_or("").to_string();

        // check if exists
        if book_id.is_empty() {
            return ok(BookIdExists {
                id: None,
                exists: false,
            });
        }

        // convert to ObjectId
        let book_id = match ObjectId::parse_str(&book_id) {
            Ok(id) => id,
            Err(_) => {
                return ok(BookIdExists {
                    id: None,
                    exists: false,
                });
            }
        };

        ok(BookIdExists {
            id: Some(book_id),
            exists: true,
        })
    }
}
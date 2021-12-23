//! Logic for voting for a book.

use actix_web::{
    error::ResponseError, http::StatusCode, post, web, HttpResponse,
    HttpResponseBuilder, Responder,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{BookDocument, BookResponse, ErrorResponse};

/// Endpoint handler for voting for a book.
#[post("/v1/books/{id}/supporters")]
async fn handle(
    info: web::Path<String>,
    vote_book: web::Json<VoteBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let vote_book = vote_book.into_inner();

    let updated_document = books
        .find_one_and_update(
            doc! {"_id": ObjectId::parse_str(info.into_inner())?},
            doc! {"$addToSet": {"supporters": vote_book.supporter}},
            FindOneAndUpdateOptions::builder()
                .return_document(Some(ReturnDocument::After))
                .build(),
        )
        .await?
        .ok_or(Error::NoSuchBook)?;

    let updated_book: BookResponse =
        bson::from_document::<BookDocument>(updated_document)?.into();

    Ok(HttpResponse::Ok().json(updated_book))
}

/// A request for voting for a book.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteBook {
    pub supporter: String,
}

/// Possible errors while voting for a book.
#[derive(Debug)]
pub enum Error {
    ObjectId(bson::oid::Error),
    MongoDb(mongodb::error::Error),
    NoSuchBook,
    Deserialization(bson::de::Error),
}

impl From<bson::oid::Error> for Error {
    fn from(err: bson::oid::Error) -> Self {
        Self::ObjectId(err)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDb(err)
    }
}

impl From<bson::de::Error> for Error {
    fn from(err: bson::de::Error) -> Self {
        Self::Deserialization(err)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            status_code: self.status_code().as_u16(),
            message: self.to_string(),
        };

        HttpResponseBuilder::new(self.status_code()).json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::NoSuchBook => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoSuchBook => {
                write!(f, "Book does not exist.")
            }
            _ => write!(f, "An internal error occurred."),
        }
    }
}

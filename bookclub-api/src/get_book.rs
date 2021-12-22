//! Logic for getting a single book.

use actix_web::{
    error::ResponseError, get, http::StatusCode, web, HttpResponse,
    HttpResponseBuilder, Responder,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Collection,
};
use std::fmt;

use crate::{BookDocument, BookResponse, ErrorResponse};

/// Endpoint handler for getting a single book.
#[get("/v1/books/{id}")]
async fn handle(
    info: web::Path<String>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let document = books
        .find_one(doc! {"_id": ObjectId::parse_str(info.into_inner())?}, None)
        .await?
        .ok_or(Error::NoSuchBook)?;

    let book: BookResponse =
        bson::from_document::<BookDocument>(document)?.into();

    Ok(HttpResponse::Ok().json(book))
}

/// Possible errors while getting a book.
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

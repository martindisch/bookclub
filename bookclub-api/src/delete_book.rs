//! Logic for deleting books.

use actix_web::{
    delete, error::ResponseError, http::StatusCode, web, HttpResponse,
    HttpResponseBuilder, Responder,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Collection,
};
use std::fmt;

use crate::ErrorResponse;

/// Endpoint handler for deleting books.
#[delete("/v1/books/{id}")]
async fn handle(
    info: web::Path<String>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    books
        .delete_one(
            doc! {"_id": ObjectId::parse_str(info.into_inner())?},
            None,
        )
        .await?;

    Ok(HttpResponse::NoContent())
}

/// Possible errors while deleting a book.
#[derive(Debug)]
pub enum Error {
    ObjectId(bson::oid::Error),
    MongoDb(mongodb::error::Error),
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

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            status_code: self.status_code().as_u16(),
            message: self.to_string(),
        };

        HttpResponseBuilder::new(self.status_code()).json(response)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An internal error occurred.")
    }
}

//! Logic for getting books.

use actix_web::{
    error::ResponseError, get, http::StatusCode, web, HttpResponse,
    HttpResponseBuilder, Responder,
};
use futures::StreamExt;
use mongodb::{
    bson::{self, Document},
    Collection,
};
use std::fmt;

use crate::{BookDocument, BookResponse, ErrorResponse};

/// Endpoint handler for getting books.
#[get("/v1/books")]
async fn handle(
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, GetError> {
    let mut cursor = books.find(None, None).await?;
    let mut books: Vec<BookResponse> = Vec::new();

    while let Some(Ok(document)) = cursor.next().await {
        books.push(bson::from_document::<BookDocument>(document)?.into());
    }

    Ok(HttpResponse::Ok().json(books))
}

/// Possible errors while getting books.
#[derive(Debug)]
pub enum GetError {
    Deserialization(bson::de::Error),
    MongoDb(mongodb::error::Error),
}

impl From<mongodb::error::Error> for GetError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDb(err)
    }
}

impl From<bson::de::Error> for GetError {
    fn from(err: bson::de::Error) -> Self {
        Self::Deserialization(err)
    }
}

impl ResponseError for GetError {
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

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An internal error occurred.")
    }
}

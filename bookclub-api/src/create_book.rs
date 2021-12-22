//! Logic for creating books.

use actix_web::{
    error::ResponseError, http::StatusCode, post, web, HttpResponse,
    HttpResponseBuilder, Responder,
};
use mongodb::{
    bson::{self, DateTime, Document},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{BookResponse, ErrorResponse};

/// Endpoint handler for creating books.
#[post("/v1/books")]
async fn handle(
    create_book: web::Json<CreateBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, CreateError> {
    let create_book = create_book.into_inner();
    let now = DateTime::now();

    let mut document = bson::to_document(&create_book)?;
    document.insert("firstSuggested", now);

    let insert_one_result = books.insert_one(document, None).await?;
    let id = insert_one_result
        .inserted_id
        .as_object_id()
        .ok_or(CreateError::BadObjectId)?;

    let book = BookResponse {
        id: id.to_hex(),
        title: create_book.title,
        author: create_book.author,
        description: create_book.description,
        page_count: create_book.page_count,
        pitch_by: create_book.pitch_by,
        first_suggested: now.into(),
        supporters: create_book.supporters,
    };

    Ok(HttpResponse::Ok().json(book))
}

/// A request for creating a new book.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub description: String,
    pub page_count: u32,
    pub pitch_by: String,
    pub supporters: Vec<String>,
}

/// Possible errors while creating a book.
#[derive(Debug)]
pub enum CreateError {
    Serialization(bson::ser::Error),
    MongoDb(mongodb::error::Error),
    BadObjectId,
}

impl From<bson::ser::Error> for CreateError {
    fn from(err: bson::ser::Error) -> Self {
        Self::Serialization(err)
    }
}

impl From<mongodb::error::Error> for CreateError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDb(err)
    }
}

impl ResponseError for CreateError {
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

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An internal error occurred.")
    }
}

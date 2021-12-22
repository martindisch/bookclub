//! Logic for updating books.

use actix_web::{
    error::ResponseError, http::StatusCode, patch, web, HttpResponse,
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

/// Endpoint handler for updating books.
#[patch("/v1/books/{id}")]
async fn handle(
    info: web::Path<String>,
    update_book: web::Json<UpdateBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, UpdateError> {
    let update_book = update_book.into_inner();

    let updated_document = books
        .find_one_and_update(
            doc! {"_id": ObjectId::parse_str(info.into_inner())?},
            build_update(update_book),
            FindOneAndUpdateOptions::builder()
                .return_document(Some(ReturnDocument::After))
                .build(),
        )
        .await?
        .ok_or(UpdateError::NoSuchBook)?;

    let updated_book: BookResponse =
        bson::from_document::<BookDocument>(updated_document)?.into();

    Ok(HttpResponse::Ok().json(updated_book))
}

/// A request for updating a book.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub page_count: Option<u32>,
    pub pitch_by: Option<String>,
}

/// Builds the MongoDB documents representing the update.
fn build_update(update_book: UpdateBook) -> Vec<Document> {
    let mut updates = Vec::new();

    // This is dumb. We could probably do something with serde to automatically
    // turn it into a Document.
    if let Some(value) = update_book.title {
        updates.push(doc! {"$set": {"title": value}})
    }
    if let Some(value) = update_book.author {
        updates.push(doc! {"$set": {"author": value}})
    }
    if let Some(value) = update_book.description {
        updates.push(doc! {"$set": {"description": value}})
    }
    if let Some(value) = update_book.page_count {
        updates.push(doc! {"$set": {"pageCount": value}})
    }
    if let Some(value) = update_book.pitch_by {
        updates.push(doc! {"$set": {"pitchBy": value}})
    }

    updates
}

/// Possible errors while updating a book.
#[derive(Debug)]
pub enum UpdateError {
    ObjectId(bson::oid::Error),
    MongoDb(mongodb::error::Error),
    NoSuchBook,
    Deserialization(bson::de::Error),
}

impl From<bson::oid::Error> for UpdateError {
    fn from(err: bson::oid::Error) -> Self {
        Self::ObjectId(err)
    }
}

impl From<mongodb::error::Error> for UpdateError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDb(err)
    }
}

impl From<bson::de::Error> for UpdateError {
    fn from(err: bson::de::Error) -> Self {
        Self::Deserialization(err)
    }
}

impl ResponseError for UpdateError {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            status_code: self.status_code().as_u16(),
            message: self.to_string(),
        };

        HttpResponseBuilder::new(self.status_code()).json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UpdateError::NoSuchBook => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateError::NoSuchBook => {
                write!(f, "Book does not exist.")
            }
            _ => write!(f, "An internal error occurred."),
        }
    }
}

//! Logic for creating a book.

use actix_web::{
    error::{Error, ErrorInternalServerError},
    post, web, HttpResponse, Responder,
};
use mongodb::{
    bson::{self, DateTime, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::BookResponse;

/// Endpoint handler for creating a book.
#[post("/v1/books")]
async fn handle(
    create_book: web::Json<CreateBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let create_book = create_book.into_inner();
    let now = DateTime::now();

    let mut document = bson::to_document(&create_book)
        .map_err(|_| ErrorInternalServerError("Serialization error"))?;
    document.insert("firstSuggested", now);
    document.insert("supporterCount", create_book.supporters.len() as u32);

    let insert_one_result = books
        .insert_one(document, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?;
    let id = insert_one_result
        .inserted_id
        .as_object_id()
        .ok_or_else(|| ErrorInternalServerError("Invalid ID"))?;

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

/// A request for creating a book.
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

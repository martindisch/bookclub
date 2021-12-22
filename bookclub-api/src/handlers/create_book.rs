use actix_web::{post, web, HttpResponse, Responder};
use mongodb::{
    bson::{self, DateTime, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::{book_service::Error, handlers::BookResponse};

#[post("/v1/books")]
async fn handle(
    create_book: web::Json<CreateBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let create_book = create_book.into_inner();
    let now = DateTime::now();

    let mut document = bson::to_document(&create_book).unwrap();
    document.insert("firstSuggested", now);

    let insert_one_result = books.insert_one(document, None).await.unwrap();
    let id = insert_one_result.inserted_id.as_object_id().unwrap();

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
struct CreateBook {
    pub title: String,
    pub author: String,
    pub description: String,
    pub page_count: u32,
    pub pitch_by: String,
    pub supporters: Vec<String>,
}

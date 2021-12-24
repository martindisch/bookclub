//! Logic for getting books.

use actix_web::{
    error::{Error, ErrorInternalServerError},
    get, web, HttpResponse, Responder,
};
use futures::StreamExt;
use mongodb::{
    bson::{self, Document},
    Collection,
};

use crate::{BookDocument, BookResponse};

/// Endpoint handler for getting books.
#[get("/v1/books")]
async fn handle(
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let mut cursor = books
        .find(None, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?;
    let mut books: Vec<BookResponse> = Vec::new();

    while let Some(Ok(document)) = cursor.next().await {
        books.push(
            bson::from_document::<BookDocument>(document)
                .map_err(|_| {
                    ErrorInternalServerError("Deserialization error")
                })?
                .into(),
        );
    }

    Ok(HttpResponse::Ok().json(books))
}

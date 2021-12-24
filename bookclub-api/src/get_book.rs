//! Logic for getting a book.

use actix_web::{
    error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    get, web, HttpResponse, Responder,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Collection,
};

use crate::{BookDocument, BookResponse};

/// Endpoint handler for getting a book.
#[get("/v1/books/{id}")]
async fn handle(
    info: web::Path<String>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let id = ObjectId::parse_str(info.into_inner())
        .map_err(|_| ErrorBadRequest("Invalid ID"))?;

    let document = books
        .find_one(doc! {"_id": id}, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?
        .ok_or_else(|| ErrorNotFound("Book does not exist"))?;

    let book: BookResponse = bson::from_document::<BookDocument>(document)
        .map_err(|_| ErrorInternalServerError("Deserialization error"))?
        .into();

    Ok(HttpResponse::Ok().json(book))
}

//! Logic for deleting a book.

use actix_web::{
    delete,
    error::{Error, ErrorBadRequest, ErrorInternalServerError},
    web, HttpResponse, Responder,
};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};

/// Endpoint handler for deleting a book.
#[delete("/v1/books/{id}")]
async fn handle(
    info: web::Path<String>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let id = ObjectId::parse_str(info.into_inner())
        .map_err(|_| ErrorBadRequest("Invalid ID"))?;

    books
        .delete_one(doc! {"_id": id}, None)
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::NoContent())
}

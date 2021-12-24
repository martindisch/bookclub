//! Logic for updating a book.

use actix_web::{
    error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    patch, web, HttpResponse, Responder,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::{BookDocument, BookResponse};

/// Endpoint handler for updating a book.
#[patch("/v1/books/{id}")]
async fn handle(
    info: web::Path<String>,
    update_book: web::Json<UpdateBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let update_book = update_book.into_inner();
    let id = ObjectId::parse_str(info.into_inner())
        .map_err(|_| ErrorBadRequest("Invalid ID"))?;

    let updated_document = books
        .find_one_and_update(
            doc! {"_id": id},
            build_update(update_book),
            FindOneAndUpdateOptions::builder()
                .return_document(Some(ReturnDocument::After))
                .build(),
        )
        .await
        .map_err(|_| ErrorInternalServerError("Database error"))?
        .ok_or_else(|| ErrorNotFound("Book does not exist"))?;

    let updated_book: BookResponse =
        bson::from_document::<BookDocument>(updated_document)
            .map_err(|_| ErrorInternalServerError("Deserialization error"))?
            .into();

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

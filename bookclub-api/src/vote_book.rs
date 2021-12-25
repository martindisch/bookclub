//! Logic for voting for a book.

use actix_web::{
    error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    post, web, HttpResponse, Responder,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::{BookDocument, BookResponse};

/// Endpoint handler for voting for a book.
#[post("/v1/books/{id}/supporters")]
async fn handle(
    info: web::Path<String>,
    vote_book: web::Json<VoteBook>,
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let vote_book = vote_book.into_inner();
    let id = ObjectId::parse_str(info.into_inner())
        .map_err(|_| ErrorBadRequest("Invalid ID"))?;

    let updated_document = books
        .find_one_and_update(
            doc! {"_id": id},
            vec![
                doc! {"$set": {"supporters":
                    {"$setUnion": ["$supporters", [vote_book.supporter]
                ]}}},
                doc! {"$set": {"supporterCount": {"$size": "$supporters"}}},
            ],
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

/// A request for voting for a book.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteBook {
    pub supporter: String,
}

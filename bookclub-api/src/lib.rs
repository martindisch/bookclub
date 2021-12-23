use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};

pub mod create_book;
pub mod delete_book;
pub mod get_book;
pub mod get_books;
pub mod update_book;
pub mod vote_book;

/// A book as returned by the API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub page_count: u32,
    pub pitch_by: String,
    pub first_suggested: DateTime<Utc>,
    pub supporters: Vec<String>,
}

/// The MongoDB model of a book.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookDocument {
    #[serde(rename(deserialize = "_id"))]
    id: ObjectId,
    title: String,
    author: String,
    description: String,
    page_count: u32,
    pitch_by: String,
    first_suggested: BsonDateTime,
    supporters: Vec<String>,
}

#[allow(clippy::from_over_into)]
impl Into<BookResponse> for BookDocument {
    fn into(self) -> BookResponse {
        BookResponse {
            id: self.id.to_hex(),
            title: self.title,
            author: self.author,
            description: self.description,
            page_count: self.page_count,
            pitch_by: self.pitch_by,
            first_suggested: self.first_suggested.into(),
            supporters: self.supporters,
        }
    }
}

/// The body of a non-2XX (error) response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub status_code: u16,
    pub message: String,
}

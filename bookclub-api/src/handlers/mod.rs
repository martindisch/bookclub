use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};

pub mod create_book;
pub mod get_books;
pub mod update_book;

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

/// A book as it is stored in MongoDB.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookDocument {
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

/// The error response that will be serialized to the body.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    status_code: u16,
    message: String,
}

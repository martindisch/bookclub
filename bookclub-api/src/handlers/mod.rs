use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::Book;

pub mod create_book;
pub mod get_books;
pub mod update_book;

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
    first_suggested: DateTime,
    supporters: Vec<String>,
}

#[allow(clippy::from_over_into)]
impl Into<Book> for BookDocument {
    fn into(self) -> Book {
        Book {
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

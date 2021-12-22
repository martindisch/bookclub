use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod book_repository;
pub mod book_service;
pub mod deprecated_handlers;
pub mod handlers;

/// A book.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub page_count: u32,
    pub pitch_by: String,
    pub first_suggested: DateTime<Utc>,
    pub supporters: Vec<String>,
}

/// A request for updating a book.
#[derive(Debug)]
pub struct UpdateBook {
    pub id: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub page_count: Option<u32>,
    pub pitch_by: Option<String>,
    pub first_suggested: Option<DateTime<Utc>>,
    pub supporters: Option<Vec<String>>,
}

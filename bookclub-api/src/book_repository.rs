//! Contains data-access logic.

use mongodb::bson::{self, doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use std::{error, fmt};

use crate::Book;

/// A book as it is stored in MongoDB.
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

/// The error type wrapping what can go wrong in the repository.
#[derive(Debug)]
pub enum Error {
    Serialization(bson::ser::Error),
    Deserialization(bson::de::Error),
    ObjectId(bson::oid::Error),
    MongoDb(mongodb::error::Error),
    BadObjectId,
    NoSuchBook,
}

impl From<bson::ser::Error> for Error {
    fn from(err: bson::ser::Error) -> Self {
        Self::Serialization(err)
    }
}

impl From<bson::de::Error> for Error {
    fn from(err: bson::de::Error) -> Self {
        Self::Deserialization(err)
    }
}

impl From<bson::oid::Error> for Error {
    fn from(err: bson::oid::Error) -> Self {
        Self::ObjectId(err)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoDb(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serialization(e) => e.fmt(f),
            Self::Deserialization(e) => e.fmt(f),
            Self::ObjectId(_) => write!(f, "Invalid ID."),
            Self::MongoDb(e) => e.fmt(f),
            Self::BadObjectId => write!(f, "Insert did not return ObjectId."),
            Self::NoSuchBook => write!(f, "Book does not exist."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Serialization(ref e) => Some(e),
            Self::Deserialization(ref e) => Some(e),
            Self::ObjectId(ref e) => Some(e),
            Self::MongoDb(ref e) => Some(e),
            Self::BadObjectId => None,
            Self::NoSuchBook => None,
        }
    }
}

//! Contains data-access logic.

use futures::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, DateTime, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{Deserialize, Serialize};

use std::{error, fmt};

use crate::{Book, CreateBook, UpdateBook};

/// Gives access to the MongoDB collection for books.
pub struct BookRepository {
    books: Collection<Document>,
}

impl BookRepository {
    /// Creates a new repository.
    pub fn new(books: Collection<Document>) -> Self {
        Self { books }
    }

    /// Inserts a new book, returning the ID.
    pub async fn insert_book(
        &self,
        create_book: &CreateBook,
    ) -> Result<String, Error> {
        let mut document = bson::to_document(create_book)?;

        // Sadly we need to replace DateTime<Utc> with the DateTime wrapper,
        // because DateTime<Utc> is serialized to String, whereas we want the
        // native BSON datetime type in the DB
        document.insert(
            "firstSuggested",
            bson::DateTime::from(create_book.first_suggested),
        );

        let insert_one_result = self.books.insert_one(document, None).await?;
        let id = insert_one_result
            .inserted_id
            .as_object_id()
            .ok_or(Error::BadObjectId)?;

        Ok(id.to_hex())
    }

    /// Updates a book and returns the new one.
    pub async fn update_book(
        &self,
        update_book: UpdateBook,
    ) -> Result<Book, Error> {
        let updated_document = self
            .books
            .find_one_and_update(
                doc! {"_id": ObjectId::parse_str(&update_book.id)?},
                build_update(update_book),
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(ReturnDocument::After))
                    .build(),
            )
            .await?
            .ok_or(Error::NoSuchBook)?;
        let updated_book: Book =
            bson::from_document::<BookDocument>(updated_document)?.into();

        Ok(updated_book)
    }

    /// Returns all books.
    pub async fn books(&self) -> Result<Vec<Book>, Error> {
        let mut cursor = self.books.find(None, None).await?;
        let mut books = Vec::new();

        while let Some(Ok(document)) = cursor.next().await {
            books.push(bson::from_document::<BookDocument>(document)?.into());
        }

        Ok(books)
    }
}

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
    if let Some(value) = update_book.first_suggested {
        updates.push(doc! {"$set": {"firstSuggested": value}})
    }
    if let Some(value) = update_book.supporters {
        updates.push(doc! {"$set": {"supporters": value}})
    }

    updates
}

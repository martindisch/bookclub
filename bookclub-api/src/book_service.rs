//! Contains domain logic.

use std::{error, fmt};

use crate::{
    book_repository::{BookRepository, Error as RepositoryError},
    Book, CreateBook, UpdateBook,
};

/// Represents the books domain.
pub struct BookService {
    repository: BookRepository,
}

impl BookService {
    /// Creates a new service.
    pub fn new(repository: BookRepository) -> Self {
        Self { repository }
    }

    /// Creates a new book.
    pub async fn create_book(
        &self,
        create_book: CreateBook,
    ) -> Result<Book, Error> {
        let id = self.repository.insert_book(&create_book).await?;

        Ok(Book {
            id,
            title: create_book.title,
            author: create_book.author,
            description: create_book.description,
            page_count: create_book.page_count,
            pitch_by: create_book.pitch_by,
            first_suggested: create_book.first_suggested,
            supporters: create_book.supporters,
        })
    }

    /// Updates a book.
    pub async fn update_book(
        &self,
        update_book: UpdateBook,
    ) -> Result<Book, Error> {
        Ok(self.repository.update_book(update_book).await?)
    }

    /// Returns all books.
    pub async fn books(&self) -> Result<Vec<Book>, Error> {
        Ok(self.repository.books().await?)
    }
}

/// The error type for everything.
#[derive(Debug)]
pub enum Error {
    Internal(RepositoryError),
    User(String),
}

impl From<RepositoryError> for Error {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NoSuchBook => Self::User(err.to_string()),
            RepositoryError::ObjectId(_) => Self::User(err.to_string()),
            err => Self::Internal(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal(_) => write!(f, "An internal error occurred."),
            Self::User(message) => write!(f, "{}", message),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Internal(ref e) => Some(e),
            Self::User(_) => None,
        }
    }
}

//! Contains domain logic.

use std::{error, fmt};

use crate::{
    book_repository::{BookRepository, Error as RepositoryError},
    Book, UpdateBook,
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

    /// Updates a book.
    pub async fn update_book(
        &self,
        update_book: UpdateBook,
    ) -> Result<Book, Error> {
        Ok(self.repository.update_book(update_book).await?)
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

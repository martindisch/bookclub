//! Contains domain logic.

use std::{error, fmt};

use crate::book_repository::Error as RepositoryError;

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

//! Contains all endpoint handlers.

use actix_web::{
    error::ResponseError, http::StatusCode, HttpResponse, HttpResponseBuilder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::book_service::Error;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            status_code: self.status_code().as_u16(),
            message: self.to_string(),
        };

        HttpResponseBuilder::new(self.status_code()).json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::User(_) => StatusCode::BAD_REQUEST,
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

/// An API request for updating a book.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequest {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub page_count: Option<u32>,
    pub pitch_by: Option<String>,
    pub first_suggested: Option<DateTime<Utc>>,
    pub supporters: Option<Vec<String>>,
}

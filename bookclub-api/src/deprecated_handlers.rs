//! Contains all endpoint handlers.

use actix_web::{
    error::ResponseError, get, http::StatusCode, patch, web, HttpResponse,
    HttpResponseBuilder, Responder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{book_service::Error, ServiceContainer, UpdateBook};

#[get("/v1/books")]
async fn books(
    service_container: web::Data<ServiceContainer>,
) -> Result<impl Responder, Error> {
    let books = service_container.book_service.books().await?;
    Ok(HttpResponse::Ok().json(books))
}

#[patch("/v1/books/{id}")]
async fn update_book(
    info: web::Path<String>,
    update_book_request: web::Json<UpdateBookRequest>,
    service_container: web::Data<ServiceContainer>,
) -> Result<impl Responder, Error> {
    let update_book_request = update_book_request.into_inner();
    let book = service_container
        .book_service
        .update_book((update_book_request, info.into_inner()).into())
        .await?;
    Ok(HttpResponse::Ok().json(book))
}

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

#[allow(clippy::from_over_into)]
impl Into<UpdateBook> for (UpdateBookRequest, String) {
    fn into(self) -> UpdateBook {
        UpdateBook {
            id: self.1,
            title: self.0.title,
            author: self.0.author,
            description: self.0.description,
            page_count: self.0.page_count,
            pitch_by: self.0.pitch_by,
            first_suggested: self.0.first_suggested,
            supporters: self.0.supporters,
        }
    }
}

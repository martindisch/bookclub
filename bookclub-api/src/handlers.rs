//! Contains all endpoint handlers.

use actix_web::{
    dev::HttpResponseBuilder, error::ResponseError, get, http::StatusCode,
    post, web, HttpResponse, Responder,
};
use serde::Serialize;

use crate::{meeting_service::Error, CreateMeeting, ServiceContainer};

#[get("/v1/meetings")]
async fn meetings(
    service_container: web::Data<ServiceContainer>,
) -> Result<impl Responder, Error> {
    let meetings = service_container.meeting_service.meetings().await?;
    Ok(HttpResponse::Ok().json(meetings))
}

#[post("/v1/meetings")]
async fn create_meeting(
    create_meeting: web::Json<CreateMeeting>,
    service_container: web::Data<ServiceContainer>,
) -> Result<impl Responder, Error> {
    let meeting_with_id = service_container
        .meeting_service
        .create_meeting(create_meeting.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(meeting_with_id))
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
            Self::User => StatusCode::BAD_REQUEST,
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

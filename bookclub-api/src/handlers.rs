//! Contains all endpoint handlers.

use actix_web::{
    dev::HttpResponseBuilder, error::ResponseError, get, http::StatusCode,
    patch, post, web, HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    meeting_service::Error, CreateMeeting, ServiceContainer, UpdateMeeting,
};

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
    let meeting = service_container
        .meeting_service
        .create_meeting(create_meeting.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(meeting))
}

#[patch("/v1/meetings/{id}")]
async fn update_meeting(
    info: web::Path<String>,
    update_meeting_request: web::Json<UpdateMeetingRequest>,
    service_container: web::Data<ServiceContainer>,
) -> Result<impl Responder, Error> {
    let update_meeting_request = update_meeting_request.into_inner();
    let meeting = service_container
        .meeting_service
        .update_meeting((update_meeting_request, info.into_inner()).into())
        .await?;
    Ok(HttpResponse::Ok().json(meeting))
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

/// An API request for updating a meeting.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMeetingRequest {
    pub date: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub pitched_by: Option<String>,
    pub first_suggested: Option<DateTime<Utc>>,
    pub supporters: Option<Vec<String>>,
}

#[allow(clippy::from_over_into)]
impl Into<UpdateMeeting> for (UpdateMeetingRequest, String) {
    fn into(self) -> UpdateMeeting {
        UpdateMeeting {
            id: self.1,
            date: self.0.date,
            location: self.0.location,
            title: self.0.title,
            author: self.0.author,
            description: self.0.description,
            pitched_by: self.0.pitched_by,
            first_suggested: self.0.first_suggested,
            supporters: self.0.supporters,
        }
    }
}

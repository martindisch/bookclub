//! Contains all endpoint handlers.

use actix_web::{
    error::ResponseError, get, post, web, HttpResponse, Responder,
};

use crate::{meeting_repository::Error, CreateMeeting, ServiceContainer};

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

impl ResponseError for Error {}

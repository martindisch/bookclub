//! Contains all endpoint handlers.

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{CreateMeeting, ServiceContainer};

#[get("/v1/meetings")]
async fn meetings(
    service_container: web::Data<ServiceContainer>,
) -> impl Responder {
    let meetings = service_container.meeting_service.meetings().await;
    HttpResponse::Ok().json(meetings)
}

#[post("/v1/meetings")]
async fn create_meeting(
    create_meeting: web::Json<CreateMeeting>,
    service_container: web::Data<ServiceContainer>,
) -> impl Responder {
    let meeting_with_id = service_container
        .meeting_service
        .create_meeting(create_meeting.into_inner())
        .await;
    HttpResponse::Ok().json(meeting_with_id)
}

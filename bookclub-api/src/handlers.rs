//! Contains all endpoint handlers.

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{Meeting, ServiceContainer};

#[get("/v1/meetings")]
async fn meetings() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/v1/meetings")]
async fn create_meeting(
    meeting: web::Json<Meeting>,
    service_container: web::Data<ServiceContainer>,
) -> impl Responder {
    service_container
        .meeting_service
        .create_meeting(&meeting)
        .await;
    HttpResponse::Ok()
}

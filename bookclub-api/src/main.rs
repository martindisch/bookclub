use actix_web::{
    get, middleware::Logger, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use mongodb::Client;

use std::{env, io::Result};

use bookclub_api::{Meeting, MeetingRepository, MeetingService};

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

struct ServiceContainer {
    meeting_service: MeetingService,
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    Builder::from_env(Env::default().default_filter_or("info")).init();

    let client = Client::with_uri_str(
        &env::var("MONGODB").expect("MONGODB env var not set"),
    )
    .await
    .expect("Can't establish connection to MongoDB");
    let database = client.database("bookclub");
    let collection = database.collection("meetings");

    HttpServer::new(move || {
        let meeting_repository = MeetingRepository::new(collection.clone());
        let meeting_service = MeetingService::new(meeting_repository);

        App::new()
            .data(ServiceContainer { meeting_service })
            .wrap(Logger::default())
            .service(meetings)
            .service(create_meeting)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

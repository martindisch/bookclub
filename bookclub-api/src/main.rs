use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use mongodb::Client;

use std::{env, io::Result};

use bookclub_api::{
    handlers, meeting_repository::MeetingRepository,
    meeting_service::MeetingService, ServiceContainer,
};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    Builder::from_env(Env::default().default_filter_or("info")).init();

    let client = Client::with_uri_str(
        &env::var("MONGODB").expect("MONGODB env var not set."),
    )
    .await
    .expect("Can't establish connection to MongoDB.");
    let database = client.database("bookclub");
    let collection = database.collection("meetings");

    HttpServer::new(move || {
        let meeting_repository = MeetingRepository::new(collection.clone());
        let meeting_service = MeetingService::new(meeting_repository);

        App::new()
            .app_data(Data::new(ServiceContainer::new(meeting_service)))
            .wrap(Logger::default())
            .service(handlers::meetings)
            .service(handlers::create_meeting)
            .service(handlers::update_meeting)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

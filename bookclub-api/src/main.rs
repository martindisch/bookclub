use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use mongodb::{bson::Document, Client};

use std::{env, io::Result};

use bookclub_api::handlers;

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
    let collection = database.collection::<Document>("books");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(collection.clone()))
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .service(handlers::get_books::handle)
            .service(handlers::create_book::handle)
            .service(handlers::update_book::handle)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use mongodb::Client;

use std::{env, io::Result};

use bookclub_api::{
    book_repository::BookRepository, book_service::BookService,
    deprecated_handlers, ServiceContainer,
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
    let collection = database.collection("books");

    HttpServer::new(move || {
        let book_repository = BookRepository::new(collection.clone());
        let book_service = BookService::new(book_repository);

        App::new()
            .app_data(Data::new(ServiceContainer::new(book_service)))
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .service(deprecated_handlers::books)
            .service(deprecated_handlers::create_book)
            .service(deprecated_handlers::update_book)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

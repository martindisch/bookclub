use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use mongodb::{
    bson::{doc, Document},
    error,
    results::CreateIndexResult,
    Client, Collection, IndexModel,
};

use std::{env, io::Result};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let client = Client::with_uri_str(
        &env::var("MONGODB").expect("MONGODB env var not set"),
    )
    .await
    .expect("Can't establish connection to MongoDB");
    let database = client.database("bookclub");
    let collection = database.collection::<Document>("books");
    deploy_indexes(&collection)
        .await
        .expect("Can't deploy indexes");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(collection.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(
                web::scope("/api")
                    .service(bookclub_api::create_book::handle)
                    .service(bookclub_api::get_books::handle)
                    .service(bookclub_api::get_book::handle)
                    .service(bookclub_api::update_book::handle)
                    .service(bookclub_api::delete_book::handle)
                    .service(bookclub_api::vote_book::handle),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn deploy_indexes(
    collection: &Collection<Document>,
) -> error::Result<CreateIndexResult> {
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"supporterCount": -1, "firstSuggested": 1})
                .build(),
            None,
        )
        .await
}

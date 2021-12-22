use actix_web::{get, web, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::{
    bson::{self, Document},
    Collection,
};

use crate::{book_service::Error, handlers::BookDocument, Book};

#[get("/v1/books")]
async fn handle(
    books: web::Data<Collection<Document>>,
) -> Result<impl Responder, Error> {
    let mut cursor = books.find(None, None).await.unwrap();
    let mut books: Vec<Book> = Vec::new();

    while let Some(Ok(document)) = cursor.next().await {
        books.push(
            bson::from_document::<BookDocument>(document)
                .unwrap()
                .into(),
        );
    }

    Ok(HttpResponse::Ok().json(books))
}

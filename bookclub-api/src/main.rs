use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use std::io::Result;

#[get("/v1/meetings")]
async fn meetings() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(meetings))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

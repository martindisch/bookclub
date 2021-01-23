use actix_web::{
    get, middleware::Logger, App, HttpResponse, HttpServer, Responder,
};
use env_logger::{Builder, Env};

use std::io::Result;

#[get("/v1/meetings")]
async fn meetings() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(meetings))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

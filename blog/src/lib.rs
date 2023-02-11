use actix_web::{HttpServer, HttpResponse, App, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run().await
}
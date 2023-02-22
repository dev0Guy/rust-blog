use actix_web::{get,HttpResponse};

#[get("/health")]
async fn health() -> HttpResponse{
    HttpResponse::Ok()
        .body("Evrey Thing Works")
}
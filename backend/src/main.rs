use actix_web::{HttpServer, web, App as ActixWebApp};
use backend::{{api::health},render};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        ActixWebApp::new()
            .service(actix_files::Files::new("dist", "./dist"))
            .service(
                web::scope("/api")
                .service(health)
            )
            .service(render)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}

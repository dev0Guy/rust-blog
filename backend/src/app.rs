use std::fs;
use actix_web::{get, Error, HttpResponse};
use frontend::App;
use yew::ServerRenderer;

#[get("/")]
pub async fn render() -> Result<HttpResponse, Error> {
    let index_html = fs::read_to_string("dist/index.html")?;
    let content = ServerRenderer::<App>::new().render().await;
    let tmp = format!("<body>{}",content);
    let index_html = index_html.replace("<body>",&tmp);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html)
    )
}
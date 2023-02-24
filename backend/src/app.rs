use std::fs;
use actix_web::{get, Error, HttpResponse,HttpRequest};
use frontend::{ServerAppProps, ServerApp};
use yew::{ServerRenderer};


#[get("/{tail:.*}")]
pub async fn render(req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html = fs::read_to_string("dist/index.html")?;
    let url  = req.uri().to_string();
    let props = ServerAppProps {url: url.into()};
    let content =  ServerRenderer::<ServerApp>::with_props(|| props );
    let content = content.render().await;
    let tmp = format!("<body>{}",content);
    let index_html = index_html.replace("<body>",&tmp);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html)
    )
}
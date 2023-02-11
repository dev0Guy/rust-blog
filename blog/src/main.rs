use std::net::TcpListener;
use tokio;

use blog::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("0.0.0.0:8080")?;
    run(address).await
}

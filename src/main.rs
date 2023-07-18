use std::net::TcpListener;

use news_pubsub::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}

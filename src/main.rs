use news_pubsub::configuration::get_configuration;
use news_pubsub::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to get the configuration");
    let db_pool = config
        .create_connection_pool()
        .await
        .expect("failed to create connection.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind listener");

    let app = run(listener, db_pool).expect("Error creating a new HttpServer");
    app.await?;

    Ok(())
}

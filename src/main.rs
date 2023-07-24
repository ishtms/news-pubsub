use news_pubsub::init_subscriber;
use news_pubsub::startup::run;
use news_pubsub::{configuration::get_configuration, setup_subscriber};
use std::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let ((non_blocking_appender, _guard), env_filter) = setup_subscriber(env_filter, false);
    init_subscriber(non_blocking_appender, env_filter);

    let configurator = get_configuration().expect("Failed to get the configuration");
    let db_pool = configurator
        .create_connection_pool()
        .await
        .expect("failed to create connection.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configurator.application_port))
        .expect("Failed to bind listener");

    let app = run(listener, db_pool).expect("Error creating a new HttpServer");
    app.await?;

    Ok(())
}

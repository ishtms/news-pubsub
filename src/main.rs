use mongodb::options::ClientOptions;
use news_pubsub::configuration::get_configuration;
use news_pubsub::startup::run;
use std::net::TcpListener;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // let log_file = rolling::hourly("./logs", "Main_Log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);

    // tracing_subscriber::fmt()
    //     .compact()
    //     .with_env_filter(env_filter)
    //     .with_ansi(false)
    //     .with_line_number(true)
    //     .with_writer(non_blocking)
    //     .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    //     .init();

    let config = get_configuration().expect("Failed to get the configuration");
    let db_pool = config
        .create_connection_pool()
        .await
        .expect("failed to create connection.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind listener");
    let uri = "mongodb://localhost:27017/db";
    let client_options = ClientOptions::parse(uri).await.unwrap();

    let (pg_native_client, connection) =
        tokio_postgres::connect(&config.database.connection_string(), NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let app = run(listener, db_pool, client_options, pg_native_client)
        .expect("Error creating a new HttpServer");
    app.await?;

    Ok(())
}

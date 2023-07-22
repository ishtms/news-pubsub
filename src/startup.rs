use crate::routes::{fetch_mongo, fetch_pg_native, fetch_sqlx, health_check};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    mut client_options: ClientOptions,
    pg_native_client: tokio_postgres::Client,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    client_options.max_pool_size = Some(10);
    client_options.min_pool_size = Some(0);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options).unwrap();
    let mongo_client = web::Data::new(client);
    let pg_native_client = web::Data::new(pg_native_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Logger::new("%{r}a Time taken: (%Dms) %s %r %{User-Agent}i").log_target("[ACTIX]"),
            )
            .app_data(pg_native_client.clone())
            .app_data(mongo_client.clone())
            .app_data(db_pool.clone())
            .service(health_check)
            .route("/mongo", web::get().to(fetch_mongo))
            .route("/sqlx", web::get().to(fetch_sqlx))
            .route("/pg_native", web::get().to(fetch_pg_native))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

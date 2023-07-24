use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool: web::Data<sqlx::Pool<sqlx::Postgres>> = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Logger::new("%{r}a Time taken: (%Dms) %s %r %{User-Agent}i").log_target("[ACTIX]"),
            )
            .app_data(db_pool.clone())
            .service(health_check)
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

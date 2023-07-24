use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub async fn create_connection_pool(&self) -> Result<PgPool, std::io::Error> {
        Ok(
            PgPool::connect(&self.database.connection_string().expose_secret())
                .await
                .expect("Failed to create a postgres connection."),
        )
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}

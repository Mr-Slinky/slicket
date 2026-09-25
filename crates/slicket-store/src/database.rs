use sqlx::PgPool;
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;

use std::time::Duration;

pub(crate) struct DbConfig {
    pub(crate) url: String,
    pub(crate) max_con: u32,
    pub(crate) min_con: u32,
    pub(crate) acquire_timeout: Duration,
    pub(crate) idle_timeout: Duration,
    pub(crate) max_lifetime: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: String::from("postgres://slinky:secret@localhost:5432/slicket"),
            max_con: 10,
            min_con: 3,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_lifetime: Duration::from_secs(1800),
        }
    }
}

pub(crate) async fn init_pool(cfg: &DbConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(cfg.max_con)
        .min_connections(cfg.min_con)
        .acquire_timeout(cfg.acquire_timeout)
        .idle_timeout(cfg.idle_timeout)
        .max_lifetime(cfg.max_lifetime)
        .connect(&cfg.url)
        .await
}

pub(crate) async fn init_database(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}

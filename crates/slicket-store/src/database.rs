//! Connects to the Postgres database and brings its schema up to date.
//!
//! A caller first builds a [`DbConfig`] with [`DbConfig::from_env`] and passes it to [`init_pool`].
//! The pool that `init_pool` returns then goes to [`init_database`], which applies every migration
//! the database has yet to run.
//!
//! ```ignore
//! let pool = init_pool(&DbConfig::from_env()?).await?;
//! init_database(&pool).await?;
//! ```

use sqlx::PgPool;
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;

use std::time::Duration;

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// The settings that [`init_pool`] uses to open a pool of Postgres connections.
pub(crate) struct DbConfig {
    /// The connection URL, including the user and password.
    pub(crate) url: String,
    /// The largest number of connections the pool opens.
    pub(crate) max_con: u32,
    /// The number of connections the pool keeps open when idle.
    pub(crate) min_con: u32,
    /// How long a caller waits for a free connection. A caller waiting longer receives
    /// `sqlx::Error::PoolTimedOut`.
    pub(crate) acquire_timeout: Duration,
    /// How long a connection above `min_con` stays open while idle.
    pub(crate) idle_timeout: Duration,
    /// The age at which the pool replaces a connection with a new one.
    pub(crate) max_lifetime: Duration,
}

impl DbConfig {
    /// Builds a `DbConfig` whose `url` is the value of the `DATABASE_URL` environment variable.
    ///
    /// Every other field takes the fixed value that the body of `from_env` sets.
    ///
    /// `from_env` reads the environment of the running process alone. A program that keeps
    /// `DATABASE_URL` in a `.env` file loads that file into the environment before it calls
    /// `from_env`.
    ///
    /// Returns `VarError::NotPresent` when `DATABASE_URL` is unset, and `VarError::NotUnicode`
    /// when its value is not valid Unicode.
    pub(crate) fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            url: std::env::var("DATABASE_URL")?,
            max_con: 10,
            min_con: 3,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_lifetime: Duration::from_secs(1800),
        })
    }
}

/// Opens a pool of Postgres connections with the settings in `cfg`.
///
/// The pool opens its first connection before `init_pool` returns. A wrong URL, a wrong password
/// or a database that is not running therefore fails here, with the `sqlx::Error` that Postgres or
/// the network reported.
///
/// `PgPool` is cheap to clone, and every clone shares the same connections. A caller clones the
/// pool to hand it to each task that needs the database.
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

/// Applies every migration in this crate's `migrations` folder that the database has yet to run.
///
/// `sqlx::migrate!()` reads the migration files at compile time and embeds them in the binary, so
/// the binary needs no `migrations` folder at run time. Sqlx records each migration it applies in
/// the `_sqlx_migrations` table, and on the next call it skips every migration listed there.
/// Calling `init_database` on a database that is already up to date therefore changes nothing.
///
/// Returns a `MigrateError` when a migration fails, or when a file already applied has changed
/// since sqlx applied it.
pub(crate) async fn init_database(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}

// ========================================================================================== \\
//                                           Tests                                            \\
// ========================================================================================== \\

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs `init_database` on an empty database and checks that it creates the four tables that
    /// `init.sql` defines.
    ///
    /// `#[sqlx::test]` creates a fresh database for this test and drops it once the test passes.
    /// `migrations = false` leaves that database empty when the test starts, so `init_database`
    /// is the only code that can create the tables.
    #[sqlx::test(migrations = false)]
    async fn test_init_database_with_empty_db_creates_tables(pool: PgPool) {
        // Arrange
        let expected = ["org", "person", "ticket_status", "ticket"];

        // Act
        let result = init_database(&pool).await;
        let found: Vec<String> = sqlx::query_scalar(
            "SELECT table_name::text FROM information_schema.tables WHERE table_schema = 'public'",
        )
        .fetch_all(&pool)
        .await
        .expect("the table listing query should succeed");
        let missing: Vec<&str> = expected.into_iter()
                                         .filter(|table| !found.iter().any(|name| name == *table))
                                         .collect();

        // Assert
        assert!(result.is_ok(), "init_database failed: {result:?}");
        assert!(missing.is_empty(), "tables missing after migration: {missing:?}");
    }
}

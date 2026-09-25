//! Connects to the Postgres database and brings its schema up to date.
//!
//! A caller first builds a [`DbConfig`], usually from [`DbConfig::default`], and passes it to
//! [`init_pool`]. The pool that `init_pool` returns then goes to [`init_database`], which applies
//! every migration the database has yet to run.
//!
//! ```ignore
//! let pool = init_pool(&DbConfig::default()).await?;
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
///
/// | Field             | Type       | Sets                                                           |
/// |-------------------|------------|----------------------------------------------------------------|
/// | `url`             | `String`   | the connection URL, including the user and password            |
/// | `max_con`         | `u32`      | the largest number of connections the pool opens               |
/// | `min_con`         | `u32`      | the number of connections the pool keeps open when idle        |
/// | `acquire_timeout` | `Duration` | how long a caller waits for a free connection                  |
/// | `idle_timeout`    | `Duration` | how long a connection above `min_con` stays open while idle    |
/// | `max_lifetime`    | `Duration` | the age at which the pool replaces a connection with a new one |
///
/// A caller waiting longer than `acquire_timeout` receives `sqlx::Error::PoolTimedOut`.
pub(crate) struct DbConfig {
    pub(crate) url: String,
    pub(crate) max_con: u32,
    pub(crate) min_con: u32,
    pub(crate) acquire_timeout: Duration,
    pub(crate) idle_timeout: Duration,
    pub(crate) max_lifetime: Duration,
}

impl Default for DbConfig {
    /// Returns the settings for the local development database that `docker-compose.yml` starts.
    ///
    /// | Field             | Value                                              |
    /// |-------------------|----------------------------------------------------|
    /// | `url`             | `postgres://slinky:secret@localhost:5432/slicket`  |
    /// | `max_con`         | 10                                                 |
    /// | `min_con`         | 3                                                  |
    /// | `acquire_timeout` | 30 seconds                                         |
    /// | `idle_timeout`    | 5 minutes                                          |
    /// | `max_lifetime`    | 30 minutes                                         |
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

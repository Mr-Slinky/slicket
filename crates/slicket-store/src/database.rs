//! Connects to the PostgreSQL database and brings its schema up to date.
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
//                                      DbConfig Struct                                       \\
// ========================================================================================== \\
/// The settings that [`init_pool`] uses to open a pool of PostgreSQL connections.
pub struct DbConfig {
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
    /// The age at which the pool closes a connection.
    pub(crate) max_lifetime: Duration,
}

impl DbConfig {
    /// Builds a `DbConfig` whose `url` is the value of the `DATABASE_URL` environment variable.
    ///
    /// Every other field takes a fixed value:
    ///
    /// | Field             | Value      |
    /// |-------------------|------------|
    /// | `max_con`         | 10         |
    /// | `min_con`         | 3          |
    /// | `acquire_timeout` | 30 seconds |
    /// | `idle_timeout`    | 5 minutes  |
    /// | `max_lifetime`    | 30 minutes |
    ///
    /// `from_env` reads the environment of the running process alone. A program that keeps
    /// `DATABASE_URL` in a `.env` file loads that file into the environment before it calls
    /// `from_env`.
    ///
    /// Returns `VarError::NotPresent` when `DATABASE_URL` is unset, and `VarError::NotUnicode`
    /// when its value is not valid Unicode.
    pub fn from_env() -> Result<Self, std::env::VarError> {
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

// ========================================================================================== \\
//                                         Functions                                          \\
// ========================================================================================== \\
/// Opens a pool of PostgreSQL connections with the settings in `cfg`.
///
/// The pool opens its first connection before `init_pool` returns. As a result, `init_pool`
/// returns an `sqlx::Error` when the URL is malformed, the password is wrong or the database is
/// not running.
///
/// Cloning a `PgPool` copies a reference-counted pointer, and every clone shares the same
/// connections. A caller clones the pool to hand it to each task that needs the database.
pub async fn init_pool(cfg: &DbConfig) -> Result<PgPool, sqlx::Error> {
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
/// the binary needs no `migrations` folder at run time. SQLx records each migration it applies in
/// the `_sqlx_migrations` table, skipping every migration listed there on each call thereafter.
/// Calling `init_database` on a database that is already up to date therefore changes nothing.
///
/// Returns a `MigrateError` when a migration fails, when a file already applied has changed since
/// SQLx applied it, or when `_sqlx_migrations` lists a migration that has no file in this crate.
pub async fn init_database(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}

// ========================================================================================== \\
//                                           Tests                                            \\
// ========================================================================================== \\
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs `init_database` on an empty database and checks that it creates every table named in
    /// `expected`.
    ///
    /// `#[sqlx::test]` creates a fresh database for this test and drops it once the test passes.
    /// `migrations = false` leaves that database empty when the test starts, meaning
    /// `init_database` is the only code that can create the tables.
    #[sqlx::test(migrations = false)]
    async fn test_init_database_with_empty_db_creates_tables(pool: PgPool) {
        // Arrange
        let expected = [
            "org",
            "tenant",
            "person",
            "ticket_status",
            "ticket_type",
            "ticket",
        ];

        // Act
        let result = init_database(&pool).await;
        let found: Vec<String> = sqlx::query_scalar(
            "SELECT table_name::text FROM information_schema.tables WHERE table_schema = 'public'",
        )
        .fetch_all(&pool)
        .await
        .expect("the table listing query should succeed");
        let missing: Vec<&str> = expected
            .into_iter()
            .filter(|table| !found.iter().any(|name| name == *table))
            .collect();

        // Assert
        assert!(result.is_ok(), "init_database failed: {result:?}");
        assert!(
            missing.is_empty(),
            "tables missing after migration: {missing:?}"
        );
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("happy_path_setup")))]
    async fn test_happy_path_fixture_with_migrated_db_inserts_expected_rows(pool: PgPool) {
        // Arrange
        let expected = [
            ("org", 9),
            ("tenant", 1),
            ("person", 168),
            ("ticket_type", 8),
            ("ticket_status", 6),
        ];

        // Act
        let mut found = Vec::with_capacity(expected.len());
        for (table, _) in expected {
            let count: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM {table}"))
                .fetch_one(&pool)
                .await
                .expect("the row count query should succeed");
            found.push((table, count));
        }

        // Assert
        assert_eq!(found, expected);
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("happy_path_setup")))]
    async fn test_happy_path_fixture_with_migrated_db_realises_scenarios(pool: PgPool) {
        // Arrange
        // Pairs each scenario ID from docs/fixtures/ticket-scenarios.md with a query that returns
        // true when the loaded fixture realises it. The table stores each ticket's current state
        // alone, which is why LC-01 checks the status and CHG-01 has no query
        let scenarios = [
            ("LC-01", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                         WHERE name = 'New' AND closed_at IS NULL)"),
            ("LC-02", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                         WHERE name = 'In Progress' AND closed_at IS NULL)"),
            ("LC-03", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                         WHERE name = 'Waiting on Customer')"),
            ("LC-04", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                         WHERE name = 'Waiting on Third Party')"),
            ("LC-05", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                         WHERE name = 'Resolved' AND closed_at IS NULL)"),
            ("LC-06", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                         WHERE name = 'Closed' AND closed_at > created_at)"),
            ("FLD-01", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE priority = 0)"),
            ("FLD-02", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE priority = 255)"),
            ("FLD-03", r"SELECT EXISTS (SELECT 1 FROM ticket GROUP BY priority HAVING count(*) > 1)"),
            ("FLD-04", r"SELECT EXISTS (SELECT 1 FROM ticket GROUP BY created_at HAVING count(*) > 1)"),
            ("FLD-05", r"SELECT EXISTS (SELECT 1 FROM ticket JOIN ticket_status USING (status_id)
                          WHERE name = 'Closed' AND closed_at = created_at)"),
            ("FLD-06", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE title LIKE '%''%')"),
            ("FLD-07", r"SELECT EXISTS (SELECT 1 FROM ticket
                          WHERE title = 'Café till prints résumé names wrongly ☕')"),
            ("FLD-08", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE length(title) > 1000)"),
            ("FLD-10", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE description IS NULL)"),
            ("FLD-12", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE strpos(description, chr(10)) > 0)"),
            ("PER-01", r"SELECT EXISTS (SELECT 1 FROM ticket t
                          JOIN person p ON p.person_id = t.raised_by
                          JOIN tenant USING (org_id))"),
            ("PER-02", r"SELECT EXISTS (SELECT 1 FROM ticket t
                          JOIN person p ON p.person_id = t.raised_by
                          JOIN org o USING (org_id)
                          WHERE o.name = 'Fiona Marsh Bookkeeping' AND p.name = 'Fiona Marsh'
                            AND (SELECT count(*) FROM person WHERE org_id = o.org_id) = 1)"),
            ("PER-03", r"SELECT count(DISTINCT p.person_id) = 2 FROM ticket t
                          JOIN person p ON p.person_id = t.raised_by
                          WHERE p.name = 'Chloe Adams'"),
            ("PER-04", r"SELECT count(DISTINCT p.person_id) = 2 FROM ticket t
                          JOIN person p ON p.person_id = t.raised_by
                          WHERE p.name = 'Sipho Dlamini'"),
            ("PER-05", r"SELECT count(DISTINCT o.org_id) = 2 FROM ticket t
                          JOIN person p ON p.person_id = t.raised_by
                          JOIN org o USING (org_id)
                          WHERE o.name IN ('Karoo Group Holdings', 'Karoo Cold Chain')"),
            ("PER-06", r"SELECT EXISTS (SELECT 1 FROM ticket t
                          JOIN person p ON p.person_id = t.raised_by
                          JOIN org o USING (org_id)
                          WHERE o.name = 'Karoo Freight' AND p.email LIKE '%@gmail.com')"),
            ("PER-07", r"SELECT EXISTS (SELECT 1 FROM ticket GROUP BY raised_by HAVING count(*) >= 3)"),
            ("PER-08", r"SELECT EXISTS (SELECT 1 FROM person p
                          WHERE NOT EXISTS (SELECT 1 FROM ticket t WHERE t.raised_by = p.person_id))"),
            ("PER-09", r"SELECT EXISTS (SELECT 1 FROM org o
                          WHERE NOT EXISTS (SELECT 1 FROM ticket t
                                            JOIN person p ON p.person_id = t.raised_by
                                            WHERE p.org_id = o.org_id))"),
            ("LKP-01", r"SELECT NOT EXISTS (SELECT 1 FROM ticket_status s
                          WHERE NOT EXISTS (SELECT 1 FROM ticket t WHERE t.status_id = s.status_id))"),
            ("LKP-02", r"SELECT EXISTS (SELECT 1 FROM ticket_type y
                          WHERE NOT EXISTS (SELECT 1 FROM ticket t
                                            WHERE t.ticket_type_id = y.ticket_type_id))"),
            // Checks that the inserted title is gone as well as that the edited one is present
            ("CHG-02", r"SELECT NOT EXISTS (SELECT 1 FROM ticket WHERE title = 'Printer offline')
                            AND EXISTS (SELECT 1 FROM ticket
                                        WHERE title = 'Printer offline in reception')"),
            // Checks that no ticket keeps the inserted priority of 64
            ("CHG-03", r"SELECT NOT EXISTS (SELECT 1 FROM ticket WHERE priority = 64)
                            AND EXISTS (SELECT 1 FROM ticket WHERE priority = 192)"),
            // Checks the lowered priority alone, since CHG-03 also leaves a ticket at 192
            ("CHG-04", r"SELECT EXISTS (SELECT 1 FROM ticket WHERE priority = 32)"),
            ("CHG-05", r"SELECT EXISTS (SELECT 1
                          FROM generate_series(1, (SELECT max(ticket_id) FROM ticket)) AS g(id)
                          WHERE NOT EXISTS (SELECT 1 FROM ticket t WHERE t.ticket_id = g.id))"),
        ];

        // Act
        let mut failed = Vec::new();
        for (id, query) in scenarios {
            let realised: bool = sqlx::query_scalar(query)
                .fetch_one(&pool)
                .await
                .unwrap_or_else(|e| panic!("the query for {id} should succeed: {e}"));
            if !realised {
                failed.push(id);
            }
        }

        // Assert
        assert!(failed.is_empty(), "scenarios missing from the fixture: {failed:?}");
    }
}

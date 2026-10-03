//! The executable for Slicket. On start, it connects to the PostgreSQL database at the URL in
//! `DATABASE_URL` and applies every migration that database has yet to run. It then exits.

use slicket_store::database::{DbConfig, init_database, init_pool};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Loads the variables in the first `.env` file found in the working directory or one of its
    // parents, `DATABASE_URL` among them. A variable the shell already sets keeps its value.
    // `.ok()` discards the error a missing file returns, since the shell can set `DATABASE_URL`.
    dotenvy::dotenv().ok();

    let pool = init_pool(&DbConfig::from_env()?).await?;
    init_database(&pool).await?;

    Ok(())
}

//! The executable for Slicket. It will serve the API, and for now it runs code by hand against the
//! local database.

use slicket_store::database::{DbConfig, init_database, init_pool};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Loads DATABASE_URL from the .env file at the repository root. A missing file is fine when
    // the shell already sets the variable.
    dotenvy::dotenv().ok();

    let pool = init_pool(&DbConfig::from_env()?).await?;
    init_database(&pool).await?;

    Ok(())
}

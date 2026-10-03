mod fixtures;
mod sql;

use std::env;
use std::fs;

fn main() -> anyhow::Result<()> {
    fixtures::generate_tickets()?;
    Ok(())
}
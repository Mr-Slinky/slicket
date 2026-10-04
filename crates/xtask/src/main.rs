mod fixtures;
mod sql;

fn main() -> anyhow::Result<()> {
    fixtures::seed_fixtures()?;
    Ok(())
}
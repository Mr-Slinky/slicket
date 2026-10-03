//! Generates test fixture for ticket data

use crate::sql::Script;
use anyhow::ensure;
use rand::SeedableRng;
use rand::rngs::ChaCha8Rng;
use rand::seq::index;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// ========================================================================================== \\
//                                         Constants                                          \\
// ========================================================================================== \\

const FIXTURES_DIR: &str = "../slicket-store/fixtures";

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// Main entry point to the file; only public API for the module.
/// Searches for all TOML fixtures under slicket-store/fixtures and converts them into .sql
/// files to be run after migrations are applied.
pub fn seed_database() -> anyhow::Result<()> {
    for path in find_setup_files()? {
        let sql = build_dml_script(&path)?;
        fs::write(path.with_extension("sql"), sql)?;
    }

    Ok(())
}

pub fn build_dml_script(path: &Path) -> anyhow::Result<String> {
    let setup = read_setup(path)?;
    let mut script = Script::default();

    for org in &setup.org {
        push_org(&mut script, org);
        if org.id == setup.tenant.org_id {
            push_tenant(&mut script, org);
        }
        push_people(&mut script, org)?;
    }

    push_named_rows(
        &mut script,
        "ticket_type",
        "ticket_type_id",
        &setup.ticket_type,
    );
    push_named_rows(
        &mut script,
        "ticket_status",
        "status_id",
        &setup.ticket_status,
    );

    Ok(script.finalise())
}

// ========================================================================================== \\
//                                          Helpers                                           \\
// ========================================================================================== \\
/// Describes the shape of our toml file
#[derive(Deserialize)]
struct Setup {
    ticket_type: Vec<NamedRow>,   // Singular name to match TOML file
    ticket_status: Vec<NamedRow>, // Singular name to match TOML file
    tenant: Tenant,
    org: Vec<Org>, // Singular name to match TOML file
}

// id/name pairs
#[derive(Deserialize)]
struct NamedRow {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct Tenant {
    org_id: i32,
}

#[derive(Deserialize)]
struct Org {
    id: i32,
    name: String,
    domain: String,
    email_pattern: String,
    #[serde(default)]
    extra_domains: BTreeMap<String, usize>,
    #[serde(default)]
    person: Vec<NamedRow>,
}

/// Finds all toml files in the /fixtures directory and returns them as a vector of PathBuf
fn find_setup_files() -> io::Result<Vec<PathBuf>> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURES_DIR);
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            paths.push(path);
        }
    }
    paths.sort();

    Ok(paths)
}

fn read_setup(path: &Path) -> anyhow::Result<Setup> {
    let text = fs::read_to_string(path)?;

    Ok(toml::from_str(&text)?)
}

fn push_org(script: &mut Script, org: &Org) {
    let row = format!("{}, {}", org.id, quote(&org.name));
    script
        .insert_into("org", Some("org_id"), &["org_id", "name"])
        .values(&[row.as_str()]);
}

fn push_tenant(script: &mut Script, org: &Org) {
    let row = org.id.to_string();
    script
        .insert_into("tenant", None, &["org_id"])
        .values(&[row.as_str()]);
}

fn push_people(script: &mut Script, org: &Org) -> anyhow::Result<()> {
    if org.person.is_empty() {
        return Ok(());
    }

    let domains = assign_domains(org)?;
    let rows: Vec<String> = org
        .person
        .iter()
        .zip(domains)
        .map(|(person, domain)| {
            let email = build_email(&person.name, &org.email_pattern, domain);
            format!(
                "{}, {}, {}, {}",
                person.id,
                org.id,
                quote(&person.name),
                quote(&email)
            )
        })
        .collect();
    script
        .insert_into(
            "person",
            Some("person_id"),
            &["person_id", "org_id", "name", "email"],
        )
        .values(&borrow_all(&rows));

    Ok(())
}

fn assign_domains(org: &Org) -> anyhow::Result<Vec<&str>> {
    let people = org.person.len();
    let outliers: usize = org.extra_domains.values().sum();
    ensure!(
        outliers <= people,
        "org {} puts {outliers} people on extra domains but has only {people} people",
        org.id
    );

    let mut domains = vec![org.domain.as_str(); people];
    let mut rng = ChaCha8Rng::seed_from_u64(org.id as u64);
    let mut picked = index::sample(&mut rng, people, outliers).into_iter();
    for (domain, count) in &org.extra_domains {
        for person in picked.by_ref().take(*count) {
            domains[person] = domain;
        }
    }

    Ok(domains)
}

fn push_named_rows(
    script: &mut Script,
    table_name: &str,
    id_column: &str,
    named_rows: &[NamedRow],
) {
    let rows: Vec<String> = named_rows
        .iter()
        .map(|row| format!("{}, {}", row.id, quote(&row.name)))
        .collect();
    script
        .insert_into(table_name, Some(id_column), &[id_column, "name"])
        .values(&borrow_all(&rows));
}

fn build_email(name: &str, pattern: &str, domain: &str) -> String {
    let mut words = name.split_whitespace();
    let first = words.next().unwrap_or_default();
    let last: String = words.collect();
    let local: String = pattern
        .replace("{first}", first)
        .replace("{last}", &last)
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '.' | '-'))
        .collect();

    format!("{}@{domain}", local.to_lowercase())
}

fn quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

fn borrow_all(rows: &[String]) -> Vec<&str> {
    rows.iter().map(String::as_str).collect()
}

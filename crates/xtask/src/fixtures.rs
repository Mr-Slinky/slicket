//! Generates the SQL fixture files for `slicket-store` from the TOML setup files beside them.
//!
//! Each `.toml` file in `slicket-store/fixtures` describes the rows of one fixture: the ticket
//! types, the ticket statuses, the organisations and the people in each one. It also states which
//! organisation is the tenant. [`seed_fixtures`] reads every such file and writes a `.sql` file of
//! the same name next to it, ready to run once the migrations have been applied.

use crate::sql::Script;
use anyhow::ensure;
use rand::SeedableRng;
use rand::rngs::ChaCha8Rng;
use rand::seq::index;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// ========================================================================================== \\
//                                          Modules                                           \\
// ========================================================================================== \\

mod tickets;

// ========================================================================================== \\
//                                         Constants                                          \\
// ========================================================================================== \\

/// The fixtures folder of `slicket-store`, relative to the `xtask` crate folder.
const FIXTURES_DIR: &str = "../slicket-store/fixtures";

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// Writes one `.sql` file for each `.toml` setup file in `slicket-store/fixtures`.
///
/// Each SQL file takes the name of its setup file with the extension changed to `.sql`, replacing
/// any file of that name already there. [`build_dml_script`] builds the contents.
///
/// # Errors
///
/// Returns an error if the fixtures folder or a setup file cannot be read, if a setup file fails to
/// parse, if an organisation puts more people on extra domains than it has people, or if a SQL file
/// cannot be written.
pub fn seed_fixtures() -> anyhow::Result<()> {
    for path in find_setup_files()? {
        let sql = build_dml_script(&path)?;
        fs::write(path.with_extension("sql"), sql)?;
    }

    Ok(())
}

/// Reads the setup file at `path` and returns the SQL script that inserts its rows.
///
/// The script inserts the organisations in the order the file lists them. Each organisation row is
/// followed by the `tenant` row where that organisation is the tenant, then by the organisation's
/// people. The ticket types and ticket statuses come thereafter, and the `setval` lines from
/// [`Script::finalise`] end the script.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed, or if an organisation puts more people
/// on extra domains than it has people.
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

    let fixture = path.file_stem().and_then(OsStr::to_str).unwrap_or_default(); // blank if `None`
    tickets::push_tickets(&mut script, &setup, &tickets::find_tickets(fixture))?;

    Ok(script.finalise())
}

// ========================================================================================== \\
//                                          Helpers                                           \\
// ========================================================================================== \\
/// The contents of one TOML setup file.
#[derive(Deserialize)]
struct Setup {
    ticket_type: Vec<NamedRow>,   // Singular to match the key in the setup file
    ticket_status: Vec<NamedRow>, // Singular to match the key in the setup file
    tenant: Tenant,
    org: Vec<Org>, // Singular to match the key in the setup file
}

/// One TOML entry that gives an id and a name. Ticket types, ticket statuses and people all take
/// this shape in the setup file.
#[derive(Deserialize)]
struct NamedRow {
    id: i32,
    name: String,
}

/// The `[tenant]` table, whose `org_id` identifies the organisation that runs this instance of
/// Slicket.
#[derive(Deserialize)]
struct Tenant {
    org_id: i32,
}

/// One `[[org]]` table: an organisation and the people who belong to it.
#[derive(Deserialize)]
struct Org {
    id: i32,
    name: String,
    /// The email domain each person takes unless `extra_domains` assigns them another.
    domain: String,
    /// The local part of each email address, in which `{first}` and `{last}` stand for the
    /// person's first and last names.
    email_pattern: String,
    /// Each extra domain, mapped to the number of people who take it in place of `domain`.
    #[serde(default)]
    extra_domains: BTreeMap<String, usize>,
    #[serde(default)]
    person: Vec<NamedRow>,
}

/// Returns the path of every `.toml` file in the fixtures folder, sorted by path.
///
/// `fs::read_dir` returns entries in an order that varies between platforms, which is why the
/// function sorts the paths.
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

/// Reads the file at `path` and parses it as a [`Setup`].
fn read_setup(path: &Path) -> anyhow::Result<Setup> {
    let text = fs::read_to_string(path)?;

    Ok(toml::from_str(&text)?)
}

/// Appends an `INSERT` that writes `org` to the `org` table.
fn push_org(script: &mut Script, org: &Org) {
    let row = format!("{}, {}", org.id, quote(&org.name));
    script
        .insert_into("org", Some("org_id"), &["org_id", "name"])
        .values(&[row.as_str()]);
}

/// Appends an `INSERT` that writes the id of `org` to the `tenant` table, making `org` the tenant.
///
/// The `tenant` table has no identity column, which is why the insert passes `None` as its id
/// column.
fn push_tenant(script: &mut Script, org: &Org) {
    let row = org.id.to_string();
    script
        .insert_into("tenant", None, &["org_id"])
        .values(&[row.as_str()]);
}

/// Appends one `INSERT` that writes every person in `org` to the `person` table, each with an
/// email address built from their name.
///
/// An organisation with no people adds nothing to the script, since an `INSERT` needs at least
/// one row.
///
/// # Errors
///
/// Returns an error if `org` puts more people on extra domains than it has people.
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

/// Returns one email domain for each person in `org`, in the order `org.person` lists them.
///
/// Every person takes `org.domain`, apart from the people picked for `org.extra_domains`. The
/// function picks those people at random, using a generator seeded with the organisation id. As a
/// result, every run picks the same people, meaning the fixture comes out identical each time.
///
/// # Errors
///
/// Returns an error if the counts in `org.extra_domains` add up to more than the number of people.
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

/// Appends one `INSERT` that writes every row in `named_rows` to `table_name`, with the id in
/// `id_column` and the name in the `name` column.
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

/// Builds an email address for the person called `name`, from `pattern` and `domain`.
///
/// The first word of `name` replaces `{first}` in `pattern`. The remaining words, joined with no
/// spaces, replace `{last}`. The function then keeps only the letters, digits, dots and hyphens in
/// that local part, lowercases it, and appends `@` and `domain`.
///
/// # Examples
///
/// A surname of several words joins into one, and an apostrophe drops out:
///
/// ```ignore
/// assert_eq!(
///     build_email("Pieter van der Merwe", "{first}.{last}", "slicket.com"),
///     "pieter.vandermerwe@slicket.com"
/// );
/// assert_eq!(
///     build_email("Megan O'Connor", "{first}.{last}", "slicket.com"),
///     "megan.oconnor@slicket.com"
/// );
/// ```
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

/// Returns `text` as a PostgreSQL string literal, wrapped in single quotes with each single quote
/// inside doubled.
///
/// # Examples
///
/// The apostrophe in a surname doubles inside the literal:
///
/// ```ignore
/// assert_eq!(quote("Incident"), "'Incident'");
/// assert_eq!(quote("O'Connor"), "'O''Connor'");
/// ```
fn quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// Returns a `&str` borrowing each `String` in `rows`, the form `Statement::values` accepts.
fn borrow_all(rows: &[String]) -> Vec<&str> {
    rows.iter().map(String::as_str).collect()
}

// ========================================================================================== \\
//                                           Tests                                            \\
// ========================================================================================== \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_dml_script_with_committed_fixtures_returns_file_contents() {
        let paths = find_setup_files().expect("the fixtures folder should be readable");

        let stale: Vec<PathBuf> = paths
            .into_iter()
            .filter(|path| {
                let expected = build_dml_script(path).expect("the setup file should build");
                let actual = fs::read_to_string(path.with_extension("sql")).unwrap_or_default();
                expected != actual
            })
            .collect();

        assert!(stale.is_empty(), "stale fixtures, run `cargo xtask`: {stale:?}");
    }
}

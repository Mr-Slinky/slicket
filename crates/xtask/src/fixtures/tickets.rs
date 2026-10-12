//! Builds the tickets of each SQL fixture and appends the statements that write them to a
//! [`Script`].
//!
//! The setup file of a fixture lists its organisations, people, ticket types and ticket statuses.
//! The tickets come from the code in this module, and each one refers to those rows by name. A
//! caller asks [`find_tickets`] for the tickets of a fixture, then hands them to [`push_tickets`]:
//!
//! ```ignore
//! let tickets = tickets::find_tickets("happy_path_setup", &setup)?;
//! tickets::push_tickets(&mut script, &setup, &tickets)?;
//! ```
//!
//! [`push_tickets`] appends one `INSERT` for every ticket. An `UPDATE` or a `DELETE` follows for
//! each change that a ticket records, meaning the fixture also contains tickets edited after
//! creation.
//!
//! # The happy path tickets
//!
//! The happy path tickets realise the scenarios in `docs/fixtures/ticket-scenarios.md`. Each
//! scenario that one ticket can realise becomes a [`Rule`] in [`HAPPY_PATH_RULES`]. A rule sets
//! some fields of a ticket, and two rules share a ticket where they set different fields. As a
//! result, the fixture has far fewer tickets than rules.
//!
//! Some scenarios describe the whole set of tickets rather than any one ticket, such as `LKP-01`,
//! which needs every status on some ticket. [`check_happy_path`] confirms those once the rules
//! have been applied, and the build fails if one of them is unmet.

use super::{NamedRow, Setup, borrow_all, quote};
use crate::sql::Script;
use anyhow::{Context, ensure};
use rand::rngs::ChaCha8Rng;
use rand::seq::IndexedRandom;
use rand::{RngExt, SeedableRng};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display, Formatter};

// ========================================================================================== \\
//                                         Constants                                          \\
// ========================================================================================== \\

/// The priority a ticket takes where no rule sets one, halfway along the range from 0 to 255.
const DEFAULT_PRIORITY: u8 = 128;

/// The seed of the generator that picks the customer, ticket type and creation time of each new
/// happy path ticket. A fixed seed means every run writes the same tickets.
const HAPPY_PATH_SEED: u64 = 1;

/// The ticket type that no happy path ticket refers to, which realises `LKP-02`.
const UNUSED_TICKET_TYPE: &str = "Licensing";

/// Each fixture that has tickets, as the stem of its setup file paired with the function that
/// builds its tickets.
pub(super) const FIXTURES: &[(&str, BuildTickets)] = &[
    ("happy_path_setup", build_happy_path),
];

/// One rule for each happy path scenario in `docs/fixtures/ticket-scenarios.md` that a single
/// ticket can realise. A scenario that needs several tickets lists one rule for each of them.
const HAPPY_PATH_RULES: &[Rule] = &[
    // First, so that later tickets follow the deleted one and leave a gap in the ids
    Rule {
        scenario: "CHG-05",
        fields: Field::ALL,
        apply: |ticket| ticket.changes = vec![Change::Delete],
    },
    Rule {
        scenario: "LC-01",
        fields: &[Field::Status, Field::Changes],
        apply: |ticket| ticket.status = "New",
    },
    Rule {
        scenario: "LC-02",
        fields: &[Field::Status],
        apply: |ticket| ticket.status = "In Progress",
    },
    Rule {
        scenario: "LC-03",
        fields: &[Field::Status],
        apply: |ticket| ticket.status = "Waiting on Customer",
    },
    Rule {
        scenario: "LC-04",
        fields: &[Field::Status],
        apply: |ticket| ticket.status = "Waiting on Third Party",
    },
    Rule {
        scenario: "LC-05",
        fields: &[Field::Status],
        apply: |ticket| ticket.status = "Resolved",
    },
    Rule {
        scenario: "LC-06",
        fields: &[Field::Status, Field::CreatedAt, Field::ClosedAt],
        apply: |ticket| {
            ticket.status = "Closed";
            ticket.created_at = at(12, 9);
            ticket.closed_at = Some(at(14, 17));
        },
    },
    Rule {
        scenario: "FLD-01",
        fields: &[Field::Priority],
        apply: |ticket| ticket.priority = 0,
    },
    Rule {
        scenario: "FLD-02",
        fields: &[Field::Priority],
        apply: |ticket| ticket.priority = 255,
    },
    Rule {
        scenario: "FLD-04",
        fields: &[Field::CreatedAt],
        apply: |ticket| ticket.created_at = at(15, 10),
    },
    Rule {
        scenario: "FLD-04",
        fields: &[Field::CreatedAt],
        apply: |ticket| ticket.created_at = at(15, 10),
    },
    Rule {
        scenario: "FLD-05",
        fields: &[Field::Status, Field::CreatedAt, Field::ClosedAt],
        apply: |ticket| {
            ticket.status = "Closed";
            ticket.created_at = at(18, 11);
            ticket.closed_at = Some(at(18, 11));
        },
    },
    Rule {
        scenario: "FLD-06",
        fields: &[Field::Title],
        apply: |ticket| ticket.title = "O'Connor's laptop will not start".to_owned(),
    },
    Rule {
        scenario: "FLD-07",
        fields: &[Field::Title],
        apply: |ticket| ticket.title = "Café till prints résumé names wrongly ☕".to_owned(),
    },
    Rule {
        scenario: "FLD-08",
        fields: &[Field::Title],
        apply: |ticket| ticket.title = "The shared drive keeps disconnecting. ".repeat(50),
    },
    Rule {
        scenario: "FLD-10",
        fields: &[Field::Description],
        apply: |ticket| ticket.description = None,
    },
    Rule {
        scenario: "FLD-12",
        fields: &[Field::Description],
        apply: |ticket| {
            ticket.description = Some(
                "Steps to reproduce:\n1. Open Outlook.\n2. Send an email.\n3. Watch it stay in the outbox."
                    .to_owned(),
            );
        },
    },
    Rule {
        scenario: "PER-01",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Slinky IT", "Thandiwe Nkosi"),
    },
    Rule {
        scenario: "PER-02",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Fiona Marsh Bookkeeping", "Fiona Marsh"),
    },
    Rule {
        scenario: "PER-03",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Slinky IT", "Chloe Adams"),
    },
    Rule {
        scenario: "PER-03",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Harbourview Dental", "Chloe Adams"),
    },
    Rule {
        scenario: "PER-04",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Slinky IT", "Sipho Dlamini"),
    },
    Rule {
        scenario: "PER-04",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Oakridge Primary School", "Sipho Dlamini"),
    },
    Rule {
        scenario: "PER-05",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Karoo Group Holdings", "Marius Oberholzer"),
    },
    Rule {
        scenario: "PER-05",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Karoo Cold Chain", "Anton Steenkamp"),
    },
    Rule {
        scenario: "PER-06",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Karoo Freight", "Werner Horn"),
    },
    Rule {
        scenario: "PER-07",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Karoo Freight", "Jason Naicker"),
    },
    Rule {
        scenario: "PER-07",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Karoo Freight", "Jason Naicker"),
    },
    Rule {
        scenario: "PER-07",
        fields: &[Field::Raiser],
        apply: |ticket| raise_by(ticket, "Karoo Freight", "Jason Naicker"),
    },
    Rule {
        scenario: "CHG-01",
        fields: &[Field::Status, Field::Changes],
        apply: |ticket| {
            ticket.status = "New";
            ticket.changes = vec![
                Change::Status("In Progress"),
                Change::Status("Waiting on Customer"),
                Change::Status("In Progress"),
            ];
        },
    },
    Rule {
        scenario: "CHG-02",
        fields: &[Field::Title, Field::Changes],
        apply: |ticket| {
            ticket.title = "Printer offline".to_owned();
            ticket.changes = vec![Change::Title("Printer offline in reception")];
        },
    },
    Rule {
        scenario: "CHG-03",
        fields: &[Field::Priority, Field::Changes],
        apply: |ticket| {
            ticket.priority = 64;
            ticket.changes = vec![Change::Priority(192)];
        },
    },
    Rule {
        scenario: "CHG-04",
        fields: &[Field::Priority, Field::Changes],
        apply: |ticket| {
            ticket.priority = 192;
            ticket.changes = vec![Change::Priority(32)];
        },
    },
];

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// One row of the `ticket` table, together with the changes the script makes to that row after
/// inserting it.
///
/// A ticket refers to its organisation, person, ticket type and status by the names the setup
/// file gives them. [`push_tickets`] looks up the id behind each name as it writes the row.
pub(super) struct Ticket<'a> {
    /// The name of the organisation that the person in `raised_by` belongs to.
    org: &'a str,
    /// The name of the person who raised the ticket. Two organisations can each have a person of
    /// the same name, which is why `org` and `raised_by` together identify the person.
    raised_by: &'a str,
    /// The name of the ticket type.
    ticket_type: &'a str,
    /// The name of the status the `INSERT` writes. A [`Change::Status`] in `changes` replaces it
    /// later in the script.
    status: &'a str,
    /// The priority the `INSERT` writes, from 0 to 255. A [`Change::Priority`] in `changes`
    /// replaces it later in the script.
    priority: u8,
    title: String,
    created_at: Moment,
    /// The closing time, written as `NULL` where it is `None`.
    closed_at: Option<Moment>,
    /// The description, written as `NULL` where it is `None`.
    description: Option<String>,
    /// The changes the script makes to the ticket once every ticket has been inserted, in the
    /// order they run.
    changes: Vec<Change>,
}

impl<'a> Ticket<'a> {
    /// Returns a ticket raised by `raised_by` in `org` and titled `title`.
    ///
    /// The ticket is an `Incident` in status `New` with [`DEFAULT_PRIORITY`], created at 09:00 on
    /// 1 January. It has no closing time, no description and no changes.
    fn base(org: &'a str, raised_by: &'a str, title: String) -> Self {
        Self {
            org,
            raised_by,
            ticket_type: "Incident",
            status: "New",
            priority: DEFAULT_PRIORITY,
            title,
            created_at: at(1, 9),
            closed_at: None,
            description: None,
            changes: Vec::new(),
        }
    }

    /// Returns the status the ticket ends in once the script has applied all of its changes.
    ///
    /// The last [`Change::Status`] in `changes` decides it. With none, the result is `status`.
    fn find_final_status(&self) -> &str {
        self.changes
            .iter()
            .rev()
            .find_map(|change| match change {
                Change::Status(status) => Some(*status),
                _ => None,
            })
            .unwrap_or(self.status)
    }

    /// Returns the priority the ticket ends with once the script has applied all of its changes.
    ///
    /// The last [`Change::Priority`] in `changes` decides it. With none, the result is
    /// `priority`.
    fn find_final_priority(&self) -> u8 {
        self.changes
            .iter()
            .rev()
            .find_map(|change| match change {
                Change::Priority(priority) => Some(*priority),
                _ => None,
            })
            .unwrap_or(self.priority)
    }

    /// Returns `true` if `changes` contains [`Change::Delete`], meaning the script deletes the
    /// ticket after inserting it.
    fn is_deleted(&self) -> bool {
        self.changes
            .iter()
            .any(|change| matches!(change, Change::Delete))
    }
}

/// Returns the tickets of the fixture whose setup file has the stem `fixture`, such as
/// `happy_path_setup`.
///
/// A fixture with no tickets of its own returns an empty list.
///
/// # Errors
///
/// Returns an error if the setup file lacks the people or ticket types the happy path tickets need,
/// if a happy path ticket refers to an organisation, person, ticket type or status that the setup
/// file lacks, or if those tickets fail to realise one of their scenarios.
pub(super) fn find_tickets<'a>(fixture: &str, setup: &'a Setup) -> anyhow::Result<Vec<Ticket<'a>>> {
    match fixture {
        "happy_path_setup" => build_happy_path(setup),
        _ => Ok(Vec::new()),
    }
}

/// Appends one `INSERT` that writes every ticket in `tickets` to the `ticket` table, followed by
/// one statement for each of their changes.
///
/// The tickets take ids from 1 upwards, in the order `tickets` lists them. The changes come after
/// the `INSERT`, ticket by ticket, each one an `UPDATE` or a `DELETE` of that ticket's row. An
/// empty `tickets` adds nothing to the script, since an `INSERT` needs at least one row.
///
/// # Errors
///
/// Returns an error if a ticket refers to an organisation, person, ticket type or status that
/// `setup` lacks.
pub(super) fn push_tickets(
    script: &mut Script,
    setup: &Setup,
    tickets: &[Ticket<'_>],
) -> anyhow::Result<()> {
    if tickets.is_empty() {
        return Ok(());
    }

    let mut rows = Vec::with_capacity(tickets.len());
    for (ticket_id, ticket) in (1..).zip(tickets) {
        rows.push(build_row(setup, ticket_id, ticket)?);
    }
    script
        .insert_into(
            "ticket",
            Some("ticket_id"),
            &[
                "ticket_id",
                "ticket_type_id",
                "status_id",
                "priority",
                "title",
                "raised_by",
                "created_at",
                "closed_at",
                "description",
            ],
        )
        .values(&borrow_all(&rows));

    for (ticket_id, ticket) in (1..).zip(tickets) {
        for change in &ticket.changes {
            push_change(script, setup, ticket_id, change)?;
        }
    }

    Ok(())
}

// ========================================================================================== \\
//                                          Helpers                                           \\
// ========================================================================================== \\

/// A function that builds the tickets of one fixture from its setup file.
type BuildTickets = fn(&Setup) -> anyhow::Result<Vec<Ticket<'_>>>;

/// One scenario, as a change to a ticket and the fields that change touches.
///
/// Two rules can share a ticket where their fields do not overlap.
struct Rule {
    /// The ID of the scenario in `docs/fixtures/ticket-scenarios.md`, which an error cites when no
    /// ticket can be opened for the rule.
    scenario: &'static str,
    /// The fields that `apply` sets, which the rule claims on its ticket.
    fields: &'static [Field],
    /// Sets the fields in `fields` on the ticket the rule lands on.
    apply: fn(&mut Ticket<'_>),
}

/// A part of a ticket that a rule claims. `Changes` covers the whole list of changes, so that
/// no two rules add changes to the same ticket.
#[derive(Clone, Copy, PartialEq)]
enum Field {
    /// The organisation and the person who raised the ticket, set together.
    Raiser,
    /// The ticket type.
    Type,
    Status,
    Priority,
    Title,
    CreatedAt,
    ClosedAt,
    Description,
    /// The whole list in [`Ticket::changes`].
    Changes,
}

impl Field {
    /// Every field, for a rule that needs a ticket to itself.
    const ALL: &[Field] = &[
        Field::Raiser,
        Field::Type,
        Field::Status,
        Field::Priority,
        Field::Title,
        Field::CreatedAt,
        Field::ClosedAt,
        Field::Description,
        Field::Changes,
    ];
}

/// A change the script makes to a ticket after inserting it.
enum Change {
    /// An `UPDATE` that moves the ticket to the status with this name.
    Status(&'static str),
    /// An `UPDATE` that replaces the title with this one.
    Title(&'static str),
    /// An `UPDATE` that replaces the priority with this one.
    Priority(u8),
    /// A `DELETE` of the ticket's row.
    Delete,
}

/// An hour on a day in January 2026, in UTC. [`at`] creates one.
#[derive(Clone, Copy)]
struct Moment {
    day: u8,
    hour: u8,
}

/// Writes the moment as a PostgreSQL `TIMESTAMPTZ` literal, such as
/// `TIMESTAMPTZ '2026-01-05 09:00:00+00'`.
impl Display for Moment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TIMESTAMPTZ '2026-01-{:02} {:02}:00:00+00'",
            self.day, self.hour
        )
    }
}

/// Returns the moment at `hour` o'clock on `day` January 2026, in UTC.
///
/// # Panics
///
/// Panics if `day` is outside 1 to 31, or if `hour` is outside 0 to 23.
fn at(day: u8, hour: u8) -> Moment {
    assert!((1..=31).contains(&day), "day {day} is outside January");
    assert!(hour < 24, "hour {hour} is outside 0 to 23");

    Moment { day, hour }
}

/// Sets `ticket` as raised by the person called `raised_by` in the organisation called `org`.
fn raise_by(ticket: &mut Ticket<'_>, org: &'static str, raised_by: &'static str) {
    ticket.org = org;
    ticket.raised_by = raised_by;
}

/// Builds the happy path tickets by packing [`HAPPY_PATH_RULES`] onto as few tickets as their
/// fields allow, then checks the scenarios that describe the whole set of tickets.
///
/// Each rule goes on the first ticket whose claimed fields it leaves alone, or on a new ticket
/// where none fits. A new ticket takes a random customer, ticket type and creation time from a
/// generator with a fixed seed, meaning every run writes the same rows.
///
/// # Errors
///
/// Returns an error if `setup` lists no person outside the tenant or no ticket type besides
/// [`UNUSED_TICKET_TYPE`]. It also returns an error if [`check_names`] finds a name that `setup`
/// lacks, or if [`check_happy_path`] finds a scenario the tickets fail to realise.
fn build_happy_path(setup: &Setup) -> anyhow::Result<Vec<Ticket<'_>>> {
    let mut rng = ChaCha8Rng::seed_from_u64(HAPPY_PATH_SEED);
    // Lists every person outside the tenant as an organisation name and a person name
    let customers: Vec<(&str, &str)> = setup
        .org
        .iter()
        .filter(|org| org.id != setup.tenant.org_id)
        .flat_map(|org| {
            org.person
                .iter()
                .map(move |person| (org.name.as_str(), person.name.as_str()))
        })
        .collect();
    // Lists every ticket type name apart from the one that LKP-02 leaves unused
    let ticket_types: Vec<&str> = setup
        .ticket_type
        .iter()
        .map(|ticket_type| ticket_type.name.as_str())
        .filter(|&name| name != UNUSED_TICKET_TYPE)
        .collect();

    // Pairs each ticket with the fields its rules have claimed so far
    let mut packed: Vec<(Ticket<'_>, Vec<Field>)> = Vec::new();
    for rule in HAPPY_PATH_RULES {
        // Finds the first ticket where the rule claims none of the fields already claimed, or
        // opens a new ticket where no ticket qualifies
        let fits = packed.iter().position(|(_, claimed)| {
            !rule.fields.iter().any(|field| claimed.contains(field))
        });
        let index = match fits {
            Some(index) => index,
            None => {
                let ticket = open_ticket(packed.len() + 1, &mut rng, &customers, &ticket_types)
                    .with_context(|| format!("no ticket for scenario {}", rule.scenario))?;
                packed.push((ticket, Vec::new()));
                packed.len() - 1
            }
        };
        // Applies the rule to that ticket and records the fields it claims
        let (ticket, claimed) = &mut packed[index];
        (rule.apply)(ticket);
        claimed.extend_from_slice(rule.fields);
    }

    // Drops the claimed fields, confirms every name the tickets use, then checks the scenarios
    // that cover every ticket at once
    let tickets: Vec<Ticket<'_>> = packed.into_iter().map(|(ticket, _)| ticket).collect();
    check_names(setup, &tickets)?;
    check_happy_path(setup, &tickets)?;

    Ok(tickets)
}

/// Returns a new happy path ticket titled `Test Ticket {number}`, with a matching description.
///
/// `rng` picks the person who raises it from `customers` and its ticket type from `ticket_types`.
/// It also picks a creation time between 1 and 20 January, from 08:00 to 16:00. Every other field
/// comes from [`Ticket::base`].
///
/// # Errors
///
/// Returns an error if `customers` or `ticket_types` is empty.
fn open_ticket<'a>(
    number: usize,
    rng: &mut ChaCha8Rng,
    customers: &[(&'a str, &'a str)],
    ticket_types: &[&'a str],
) -> anyhow::Result<Ticket<'a>> {
    let &(org, raised_by) = customers
        .choose(rng)
        .context("the setup file lists no person outside the tenant")?;
    let &ticket_type = ticket_types
        .choose(rng)
        .with_context(|| format!("the setup file lists no ticket type besides {UNUSED_TICKET_TYPE}"))?;
    let created_at = at(rng.random_range(1..=20), rng.random_range(8..=16));
    let title = format!("Test Ticket {number}");

    Ok(Ticket {
        ticket_type,
        created_at,
        description: Some(format!("Description of test ticket {number}")),
        ..Ticket::base(org, raised_by, title)
    })
}

/// Confirms that `setup` contains every ticket type, status, organisation and person that `tickets`
/// refer to, including the status in each [`Change::Status`].
///
/// [`build_happy_path`] calls this before [`check_happy_path`]. A misspelt name in a rule is then
/// reported as that name, rather than as a scenario that the tickets fail to realise.
///
/// # Errors
///
/// Returns an error that gives the first name `setup` lacks.
fn check_names(setup: &Setup, tickets: &[Ticket<'_>]) -> anyhow::Result<()> {
    for ticket in tickets {
        find_id(&setup.ticket_type, "ticket type", ticket.ticket_type)?;
        find_id(&setup.ticket_status, "ticket status", ticket.status)?;
        find_person(setup, ticket.org, ticket.raised_by)?;
        for change in &ticket.changes {
            if let Change::Status(name) = change {
                find_id(&setup.ticket_status, "ticket status", name)?;
            }
        }
    }

    Ok(())
}

/// Checks the happy path scenarios that describe the whole set of tickets rather than any one of
/// them. A deleted ticket counts towards none of them.
///
/// Each check looks at a ticket as it stands once the script has applied its changes, using
/// [`Ticket::find_final_status`] and [`Ticket::find_final_priority`].
///
/// # Errors
///
/// Returns an error that cites the ID of the first scenario `tickets` fails to realise.
fn check_happy_path(setup: &Setup, tickets: &[Ticket<'_>]) -> anyhow::Result<()> {
    let kept: Vec<&Ticket<'_>> = tickets.iter().filter(|ticket| !ticket.is_deleted()).collect();

    // Counts the tickets at each priority, for FLD-03
    let mut priorities = BTreeMap::new();
    for ticket in &kept {
        *priorities.entry(ticket.find_final_priority()).or_insert(0) += 1;
    }
    ensure!(
        priorities.values().any(|&count| count > 1),
        "scenario FLD-03 fails: no two tickets share a priority"
    );

    // Finds every Karoo Freight person whose entry gives the domain gmail.com, for PER-06
    let mut on_gmail = BTreeSet::new();
    for org in setup.org.iter().filter(|org| org.name == "Karoo Freight") {
        for person in &org.person {
            if person.domain.as_deref() == Some("gmail.com") {
                on_gmail.insert((org.name.as_str(), person.name.as_str()));
            }
        }
    }
    ensure!(
        kept.iter()
            .any(|ticket| on_gmail.contains(&(ticket.org, ticket.raised_by))),
        "scenario PER-06 fails: nobody in Karoo Freight on gmail.com raised a ticket"
    );

    // Checks for a person with no ticket (PER-08) and an organisation with no ticket (PER-09)
    let raised = |org: &str, person: &str| {
        kept.iter()
            .any(|ticket| ticket.org == org && ticket.raised_by == person)
    };
    ensure!(
        setup
            .org
            .iter()
            .any(|org| org.person.iter().any(|person| !raised(&org.name, &person.name))),
        "scenario PER-08 fails: every person raised a ticket"
    );
    ensure!(
        setup
            .org
            .iter()
            .any(|org| kept.iter().all(|ticket| ticket.org != org.name)),
        "scenario PER-09 fails: every organisation has a ticket"
    );

    // Checks that every status ends up on a ticket (LKP-01), and that no ticket uses the unused
    // ticket type (LKP-02)
    for status in &setup.ticket_status {
        ensure!(
            kept.iter()
                .any(|ticket| ticket.find_final_status() == status.name),
            "scenario LKP-01 fails: no ticket ends in status {}",
            status.name
        );
    }
    ensure!(
        kept.iter()
            .all(|ticket| ticket.ticket_type != UNUSED_TICKET_TYPE),
        "scenario LKP-02 fails: a ticket refers to {UNUSED_TICKET_TYPE}"
    );

    // Checks that a kept ticket comes after a deleted one, for CHG-05. Ids follow the order of
    // `tickets`, meaning the deleted id is a gap only where a higher id survives
    let first_deleted = tickets.iter().position(Ticket::is_deleted);
    let last_kept = tickets.iter().rposition(|ticket| !ticket.is_deleted());
    ensure!(
        matches!((first_deleted, last_kept), (Some(deleted), Some(kept)) if deleted < kept),
        "scenario CHG-05 fails: no deleted ticket leaves a gap in the ticket ids"
    );

    Ok(())
}

/// Returns the values of `ticket` as one row of the `INSERT` that [`push_tickets`] appends, with
/// `ticket_id` as its id.
///
/// The values follow the column order of that `INSERT`. The row has the id of each name the ticket
/// refers to, and `NULL` for a missing closing time or description.
///
/// # Errors
///
/// Returns an error if `setup` lacks the ticket type, status, organisation or person that `ticket`
/// refers to.
fn build_row(setup: &Setup, ticket_id: i32, ticket: &Ticket<'_>) -> anyhow::Result<String> {
    let ticket_type_id = find_id(&setup.ticket_type, "ticket type", ticket.ticket_type)?;
    let status_id = find_id(&setup.ticket_status, "ticket status", ticket.status)?;
    let raised_by = find_person(setup, ticket.org, ticket.raised_by)?;
    let closed_at = ticket
        .closed_at
        .map_or_else(|| "NULL".to_owned(), |moment| moment.to_string());
    let description = ticket
        .description
        .as_deref()
        .map_or_else(|| "NULL".to_owned(), quote);

    Ok(format!(
        "{ticket_id}, {ticket_type_id}, {status_id}, {}, {}, {raised_by}, {}, {closed_at}, {description}",
        ticket.priority,
        quote(&ticket.title),
        ticket.created_at,
    ))
}

/// Appends the statement that makes `change` to the ticket with id `ticket_id`.
///
/// [`Change::Delete`] appends a `DELETE` of the ticket's row. Every other change appends an
/// `UPDATE` that sets one column of that row.
///
/// # Errors
///
/// Returns an error if a [`Change::Status`] refers to a status that `setup` lacks.
fn push_change(
    script: &mut Script,
    setup: &Setup,
    ticket_id: i32,
    change: &Change,
) -> anyhow::Result<()> {
    let condition = format!("ticket_id = {ticket_id}");
    let assignment = match change {
        Change::Status(name) => {
            format!(
                "status_id = {}",
                find_id(&setup.ticket_status, "ticket status", name)?
            )
        }
        Change::Title(title) => format!("title = {}", quote(title)),
        Change::Priority(priority) => format!("priority = {priority}"),
        Change::Delete => {
            script.delete_from("ticket").where_(&condition);
            return Ok(());
        }
    };
    script
        .update("ticket")
        .set(&[assignment.as_str()])
        .where_(&condition);

    Ok(())
}

/// Returns the id of the row in `rows` called `name`.
///
/// # Errors
///
/// Returns an error if no row is called `name`. The message describes the rows as `kind`, such
/// as "ticket status".
fn find_id(rows: &[NamedRow], kind: &str, name: &str) -> anyhow::Result<i32> {
    rows.iter()
        .find(|row| row.name == name)
        .map(|row| row.id)
        .with_context(|| format!("the setup file has no {kind} called {name}"))
}

/// Returns the id of the person called `name` in the organisation called `org`.
///
/// The function finds the organisation first, since two organisations can each have a person
/// called `name`.
///
/// # Errors
///
/// Returns an error if `setup` has no organisation called `org`, or if that organisation has no
/// person called `name`.
fn find_person(setup: &Setup, org: &str, name: &str) -> anyhow::Result<i32> {
    setup
        .org
        .iter()
        .find(|candidate| candidate.name == org)
        .with_context(|| format!("the setup file has no organisation called {org}"))?
        .person
        .iter()
        .find(|person| person.name == name)
        .map(|person| person.id)
        .with_context(|| format!("{org} has no person called {name}"))
}

// ========================================================================================== \\
//                                           Tests                                            \\
// ========================================================================================== \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_final_status_with_several_status_changes_returns_last_status() {
        let ticket = Ticket {
            changes: vec![
                Change::Status("In Progress"),
                Change::Status("Waiting on Customer"),
                Change::Status("Resolved"),
            ],
            ..Ticket::base("Karoo Freight", "Werner Horn", "Printer offline".to_owned())
        };
        let expected = "Resolved";

        let actual = ticket.find_final_status();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_final_priority_with_priority_change_returns_changed_priority() {
        let ticket = Ticket {
            priority: 64,
            changes: vec![Change::Priority(192)],
            ..Ticket::base("Karoo Freight", "Werner Horn", "Printer offline".to_owned())
        };
        let expected = 192;

        let actual = ticket.find_final_priority();

        assert_eq!(expected, actual);
    }

}

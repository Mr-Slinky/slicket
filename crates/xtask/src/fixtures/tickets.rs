use super::{NamedRow, Setup, assign_domains, borrow_all, quote};
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

const DEFAULT_PRIORITY: u8 = 128;

const HAPPY_PATH_SEED: u64 = 1;

/// The ticket type that no happy path ticket refers to, which realises `LKP-02`.
const UNUSED_TICKET_TYPE: &str = "Licensing";

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

pub(super) struct Ticket<'a> {
    org: &'a str,
    raised_by: &'a str,
    ticket_type: &'a str,
    status: &'a str,
    priority: u8,
    title: String,
    created_at: Moment,
    closed_at: Option<Moment>,
    description: Option<String>,
    changes: Vec<Change>,
}

impl<'a> Ticket<'a> {
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

    /// Returns the status the ticket holds once the script has applied all of its changes.
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

    /// Returns the priority the ticket holds once the script has applied all of its changes.
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

    fn is_deleted(&self) -> bool {
        self.changes
            .iter()
            .any(|change| matches!(change, Change::Delete))
    }
}

pub(super) fn find_tickets<'a>(fixture: &str, setup: &'a Setup) -> anyhow::Result<Vec<Ticket<'a>>> {
    match fixture {
        "happy_path_setup" => build_happy_path(setup),
        _ => Ok(Vec::new()),
    }
}

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

type BuildTickets = fn(&Setup) -> anyhow::Result<Vec<Ticket<'_>>>;

/// One scenario, as a change to a ticket and the fields that change touches.
///
/// Two rules can share a ticket where their fields do not overlap.
struct Rule {
    scenario: &'static str,
    fields: &'static [Field],
    apply: fn(&mut Ticket<'_>),
}

/// A part of a ticket that a rule claims. `Changes` covers the whole list of changes, so that
/// no two rules add changes to the same ticket.
#[derive(Clone, Copy, PartialEq)]
enum Field {
    Raiser,
    Type,
    Status,
    Priority,
    Title,
    CreatedAt,
    ClosedAt,
    Description,
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

enum Change {
    Status(&'static str),
    Title(&'static str),
    Priority(u8),
    Delete,
}

#[derive(Clone, Copy)]
struct Moment {
    day: u8,
    hour: u8,
}

impl Display for Moment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TIMESTAMPTZ '2026-01-{:02} {:02}:00:00+00'",
            self.day, self.hour
        )
    }
}

fn at(day: u8, hour: u8) -> Moment {
    assert!((1..=31).contains(&day), "day {day} is outside January");
    assert!(hour < 24, "hour {hour} is outside 0 to 23");

    Moment { day, hour }
}

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
fn build_happy_path(setup: &Setup) -> anyhow::Result<Vec<Ticket<'_>>> {
    let mut rng = ChaCha8Rng::seed_from_u64(HAPPY_PATH_SEED);
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
    let ticket_types: Vec<&str> = setup
        .ticket_type
        .iter()
        .map(|ticket_type| ticket_type.name.as_str())
        .filter(|&name| name != UNUSED_TICKET_TYPE)
        .collect();

    let mut packed: Vec<(Ticket<'_>, Vec<Field>)> = Vec::new();
    for rule in HAPPY_PATH_RULES {
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
        let (ticket, claimed) = &mut packed[index];
        (rule.apply)(ticket);
        claimed.extend_from_slice(rule.fields);
    }

    let tickets: Vec<Ticket<'_>> = packed.into_iter().map(|(ticket, _)| ticket).collect();
    check_happy_path(setup, &tickets)?;

    Ok(tickets)
}

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

/// Checks the happy path scenarios that describe the whole set of tickets rather than any one of
/// them. A deleted ticket counts towards none of them.
///
/// # Errors
///
/// Returns an error naming the first scenario that `tickets` fails to realise.
fn check_happy_path(setup: &Setup, tickets: &[Ticket<'_>]) -> anyhow::Result<()> {
    let kept: Vec<&Ticket<'_>> = tickets.iter().filter(|ticket| !ticket.is_deleted()).collect();

    let mut priorities = BTreeMap::new();
    for ticket in &kept {
        *priorities.entry(ticket.find_final_priority()).or_insert(0) += 1;
    }
    ensure!(
        priorities.values().any(|&count| count > 1),
        "scenario FLD-03 fails: no two tickets share a priority"
    );

    let mut on_extra_domain = BTreeSet::new();
    for org in &setup.org {
        for (person, domain) in org.person.iter().zip(assign_domains(org)?) {
            if domain != org.domain {
                on_extra_domain.insert((org.name.as_str(), person.name.as_str()));
            }
        }
    }
    ensure!(
        kept.iter()
            .any(|ticket| on_extra_domain.contains(&(ticket.org, ticket.raised_by))),
        "scenario PER-06 fails: nobody on an extra domain raised a ticket"
    );

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

    let first_deleted = tickets.iter().position(Ticket::is_deleted);
    let last_kept = tickets.iter().rposition(|ticket| !ticket.is_deleted());
    ensure!(
        matches!((first_deleted, last_kept), (Some(deleted), Some(kept)) if deleted < kept),
        "scenario CHG-05 fails: no deleted ticket leaves a gap in the ticket ids"
    );

    Ok(())
}

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

fn find_id(rows: &[NamedRow], kind: &str, name: &str) -> anyhow::Result<i32> {
    rows.iter()
        .find(|row| row.name == name)
        .map(|row| row.id)
        .with_context(|| format!("the setup file has no {kind} called {name}"))
}

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

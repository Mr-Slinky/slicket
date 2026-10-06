use super::{NamedRow, Setup, borrow_all, quote};
use crate::sql::Script;
use anyhow::Context;
use rand::rngs::ChaCha8Rng;
use rand::seq::IndexedRandom;
use rand::{RngExt, SeedableRng};
use std::fmt::{self, Display, Formatter};

// ========================================================================================== \\
//                                         Constants                                          \\
// ========================================================================================== \\

const DEFAULT_PRIORITY: u8 = 128;

const HAPPY_PATH_SEED: u64 = 1;

pub(super) const FIXTURES: &[(&str, fn(&Setup) -> Vec<Ticket<'_>>)] = &[
    ("happy_path_setup", build_happy_path),
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
}

pub(super) fn find_tickets<'a>(fixture: &str, setup: &'a Setup) -> Vec<Ticket<'a>> {
    match fixture {
        "happy_path_setup" => build_happy_path(setup),
        _ => Vec::new(),
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

fn build_happy_path(setup: &Setup) -> Vec<Ticket<'_>> {
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

    let pairs = setup.ticket_type.iter().flat_map(|ticket_type| {
        setup
            .ticket_status
            .iter()
            .map(move |status| (ticket_type, status))
    });

    (1..)
        .zip(pairs)
        .map(|(number, (ticket_type, status))| {
            let &(org, raised_by) = customers
                .choose(&mut rng)
                .expect("the setup file should list a person outside the tenant");
            let created_at = at(rng.random_range(1..=20), rng.random_range(8..=16));
            let closed_at =
                (status.name == "Closed").then(|| at(created_at.day + rng.random_range(0..=7), 17));
            let title = format!("Test Ticket {number}");

            Ticket {
                ticket_type: &ticket_type.name,
                status: &status.name,
                priority: rng.random(),
                created_at,
                closed_at,
                description: Some(title.clone()),
                ..Ticket::base(org, raised_by, title)
            }
        })
        .collect()
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

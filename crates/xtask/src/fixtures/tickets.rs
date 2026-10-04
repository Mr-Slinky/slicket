use super::{NamedRow, Setup, borrow_all, quote};
use crate::sql::Script;
use anyhow::{Context, ensure};
use std::fmt::{self, Display, Formatter};

const DEFAULT_PRIORITY: u8 = 128;

pub(super) const FIXTURES: &[(&str, fn() -> Vec<Ticket>)] = &[
    ("happy_path_setup", build_happy_path),
];

pub(super) struct Ticket {
    scenarios: &'static [&'static str],
    org: &'static str,
    raised_by: &'static str,
    ticket_type: &'static str,
    status: &'static str,
    priority: u8,
    title: &'static str,
    created_at: Moment,
    closed_at: Option<Moment>,
    description: Option<&'static str>,
    changes: Vec<Change>,
}

impl Ticket {
    fn base(org: &'static str, raised_by: &'static str, title: &'static str) -> Self {
        Self {
            scenarios: &[],
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

pub(super) fn find_tickets(fixture: &str) -> Vec<Ticket> {
    match fixture {
        "happy_path_setup" => build_happy_path(),
        _ => Vec::new(),
    }
}

pub(super) fn push_tickets(
    script: &mut Script,
    setup: &Setup,
    tickets: &[Ticket],
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

fn build_happy_path() -> Vec<Ticket> {
    vec![
        Ticket {
            scenarios: &["LC-01"],
            created_at: at(5, 8),
            ..Ticket::base(
                "Harbourview Dental",
                "Nadia Hendricks",
                "Front desk printer shows a paper jam",
            )
        },
        Ticket {
            scenarios: &["LC-02"],
            ticket_type: "Hardware & Peripherals",
            status: "In Progress",
            created_at: at(2, 10),
            ..Ticket::base(
                "Oakridge Primary School",
                "Elaine Visser",
                "Staff room projector has no signal",
            )
        },
        Ticket {
            scenarios: &["LC-03"],
            ticket_type: "Service Request",
            status: "Waiting on Customer",
            created_at: at(3, 11),
            ..Ticket::base(
                "Karoo Freight",
                "Hendrik Coetzee",
                "Access to the shared finance folder",
            )
        },
        Ticket {
            scenarios: &["LC-04"],
            status: "Waiting on Third Party",
            created_at: at(3, 14),
            ..Ticket::base("Karoo Freight", "Jason Naicker", "Fibre line at the depot is down")
        },
        Ticket {
            scenarios: &["LC-05"],
            ticket_type: "Service Request",
            status: "Resolved",
            created_at: at(4, 9),
            ..Ticket::base(
                "Harbourview Dental",
                "Craig Williams",
                "Set up email on a new phone",
            )
        },
        Ticket {
            scenarios: &["LC-06"],
            ticket_type: "Onboarding",
            status: "Closed",
            closed_at: Some(at(2, 16)),
            ..Ticket::base(
                "Oakridge Primary School",
                "Themba Cele",
                "Laptop and accounts for a new teacher",
            )
        },
    ]
}

fn build_row(setup: &Setup, ticket_id: i32, ticket: &Ticket) -> anyhow::Result<String> {
    ensure!(
        !ticket.scenarios.is_empty(),
        "ticket {ticket_id} lists no scenarios"
    );

    let ticket_type_id = find_id(&setup.ticket_type, "ticket type", ticket.ticket_type)?;
    let status_id = find_id(&setup.ticket_status, "ticket status", ticket.status)?;
    let raised_by = find_person(setup, ticket.org, ticket.raised_by)?;
    let closed_at = ticket
        .closed_at
        .map_or_else(|| "NULL".to_owned(), |moment| moment.to_string());
    let description = ticket.description.map_or_else(|| "NULL".to_owned(), quote);

    Ok(format!(
        "{ticket_id}, {ticket_type_id}, {status_id}, {}, {}, {raised_by}, {}, {closed_at}, {description}",
        ticket.priority,
        quote(ticket.title),
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

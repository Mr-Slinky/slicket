# Ticket scenarios

This document lists every ticket scenario that Slicket's tests and fixtures cover, or are meant to
cover. A scenario describes one shape of ticket data, such as a closed ticket or a ticket with the
highest priority. Each scenario has an ID, and a test refers to a scenario by that ID.

The list covers every scenario, whichever fixture realises it. Scenarios that break the domain model
belong here as well, even though the happy path fixture leaves them out.

## Conventions

A scenario ID joins a category prefix to a number, as in `LC-03`. The prefix identifies the section
that lists the scenario.

| Prefix | Section                          |
|--------|----------------------------------|
| `LC`   | Lifecycle                        |
| `FLD`  | Field values                     |
| `PER`  | People who raise tickets         |
| `LKP`  | Ticket types and statuses        |
| `CHG`  | Changes after creation           |
| `INV`  | Data that breaks the domain model |

The developer gives each new scenario the next free number in its category. An ID keeps its scenario
for good: a scenario that no longer applies is marked retired and stays in its table, leaving every
test that cites it pointing at something.

Each table has the same four columns:

| Column   | Contents                                                                  |
|----------|---------------------------------------------------------------------------|
| ID       | the scenario ID                                                           |
| Scenario | what the ticket data looks like, in one sentence                          |
| Fixture  | the fixture that realises the scenario, or "undecided"                    |
| Tickets  | the ids of the tickets that realise it, or "pending" until the script writes them. A scenario about rows that no ticket refers to gives the id of that organisation, type or status instead, as in "org 6" |

The happy path fixture is `happy_path_setup` in `crates/slicket-store/fixtures`. Its organisations,
people, ticket types and ticket statuses come from `happy_path_setup.toml`, and the scenarios below
use those names.

As tickets gain new data, such as an assignee or comments, each new kind of data gets a section of
its own.

## Lifecycle

These scenarios cover the stages a ticket passes through, from creation to closure.

| ID      | Scenario                                                                 | Fixture    | Tickets |
|---------|--------------------------------------------------------------------------|------------|---------|
| `LC-01` | A new ticket, in status New, that nobody has changed since creation.    | happy path | pending |
| `LC-02` | A ticket in status In Progress, with no closing time.                   | happy path | pending |
| `LC-03` | A ticket in status Waiting on Customer.                                 | happy path | pending |
| `LC-04` | A ticket in status Waiting on Third Party.                              | happy path | pending |
| `LC-05` | A ticket in status Resolved, with no closing time.                      | happy path | pending |
| `LC-06` | A ticket in status Closed, with a closing time after its creation time. | happy path | pending |
| `LC-07` | A ticket that was closed, then reopened to In Progress.                 | undecided  | pending |

`LC-07` depends on an open point: whether reopening a ticket clears its closing time.

## Field values

These scenarios put each field of a ticket at the edge of what the database accepts.

| ID       | Scenario                                                                  | Fixture    | Tickets |
|----------|---------------------------------------------------------------------------|------------|---------|
| `FLD-01` | A ticket with priority 0, the lowest the database accepts.               | happy path | pending |
| `FLD-02` | A ticket with priority 255, the highest the database accepts.            | happy path | pending |
| `FLD-03` | Several tickets with the same priority.                                   | happy path | pending |
| `FLD-04` | Two tickets with the same creation time.                                  | happy path | pending |
| `FLD-05` | A closed ticket whose closing time equals its creation time.             | happy path | pending |
| `FLD-06` | A ticket whose title contains an apostrophe, such as `O'Connor's laptop`. | happy path | pending |
| `FLD-07` | A ticket whose title contains accented letters and an emoji.             | happy path | pending |
| `FLD-08` | A ticket with a very long title.                                          | happy path | pending |
| `FLD-09` | A ticket whose title is the empty string.                                 | undecided  | pending |
| `FLD-10` | A ticket with no description, stored as `NULL`.                           | happy path | pending |
| `FLD-11` | A ticket whose description is the empty string.                           | undecided  | pending |
| `FLD-12` | A ticket whose description runs over several lines.                       | happy path | pending |

`FLD-10` and `FLD-11` exist as a pair. The store keeps `NULL` and the empty string apart, and a test
reads both back to confirm it.

`FLD-09` and `FLD-11` depend on an open point: whether the domain model accepts an empty title or an
empty description.

## People who raise tickets

These scenarios cover the person who raised a ticket. The happy path setup already contains each
person they need.

| ID       | Scenario                                                                              | Fixture    | Tickets |
|----------|---------------------------------------------------------------------------------------|------------|---------|
| `PER-01` | A ticket raised by someone in Slinky IT, the tenant's own organisation.               | happy path | pending |
| `PER-02` | A ticket raised by Fiona Marsh, the only person in Fiona Marsh Bookkeeping.          | happy path | pending |
| `PER-03` | Tickets raised by both people called Chloe Adams, one in Slinky IT and one in Harbourview Dental. | happy path | pending |
| `PER-04` | Tickets raised by both people called Sipho Dlamini, one in Slinky IT and one in Oakridge Primary School. | happy path | pending |
| `PER-05` | Tickets raised by people in Karoo Group Holdings and in Karoo Cold Chain, two organisations that share the domain `karoogroup.co.za`. | happy path | pending |
| `PER-06` | A ticket raised by the Karoo Freight person whose address is on `gmail.com`.         | happy path | pending |
| `PER-07` | One person who raised many tickets.                                                   | happy path | pending |
| `PER-08` | A person who raised no tickets.                                                       | happy path | pending |
| `PER-09` | An organisation with no tickets. Umhlanga Physio has no people, which makes it this case. | happy path | org 6   |

## Ticket types and statuses

These scenarios cover the ticket types and statuses that tickets refer to.

| ID       | Scenario                                                        | Fixture    | Tickets |
|----------|-----------------------------------------------------------------|------------|---------|
| `LKP-01` | Every ticket status has at least one ticket.                    | happy path | pending |
| `LKP-02` | A ticket type that no ticket refers to.                         | happy path | type 8  |
| `LKP-03` | A ticket status that no ticket refers to.                       | undecided  | pending |

The lifecycle scenarios `LC-01` to `LC-06` between them use all six statuses in the happy path
setup. `LKP-03` therefore needs a seventh status or a fixture of its own.

## Changes after creation

These scenarios cover tickets that the ticket script changes or deletes after inserting them.

| ID       | Scenario                                                           | Fixture    | Tickets |
|----------|--------------------------------------------------------------------|------------|---------|
| `CHG-01` | A ticket whose status changed several times before reaching its current one. | happy path | pending |
| `CHG-02` | A ticket whose title was edited.                                   | happy path | pending |
| `CHG-03` | A ticket whose priority was raised.                                | happy path | pending |
| `CHG-04` | A ticket whose priority was lowered.                               | happy path | pending |
| `CHG-05` | A deleted ticket, which leaves a gap in the ticket ids.            | happy path | pending |

## Data that breaks the domain model

These scenarios describe ticket rows that the database accepts and the domain model rejects. The happy
path fixture leaves them out, and a separate fixture is to realise them.

| ID       | Scenario                                                     | Fixture   | Tickets |
|----------|--------------------------------------------------------------|-----------|---------|
| `INV-01` | A ticket in status Closed with no closing time.              | undecided | pending |
| `INV-02` | A ticket in status New with a closing time.                  | undecided | pending |

The domain rules these scenarios break are the developer's intent at this stage. The code has yet to
enforce them.

## Open points

These points are undecided, and the scenarios that depend on them have "undecided" in their Fixture
column.

* Whether reopening a ticket clears its closing time. `LC-07` depends on it.
* Whether the domain model accepts an empty title or an empty description. `FLD-09` and `FLD-11`
  depend on it. A rule that rejects them moves both scenarios to the last section.
* Which fixture realises `LKP-03`.
* What the separate fixture for the scenarios that break the domain model is called.

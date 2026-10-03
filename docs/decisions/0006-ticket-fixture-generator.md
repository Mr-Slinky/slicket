---
status: accepted
date: 2026-10-02
decision-makers: Kheagen Haskins
---

# Generate the ticket fixture with a script that prints SQL

## Context and Problem Statement

The tests of `slicket-store` run against a real PostgreSQL database. `#[sqlx::test]` creates a fresh
database for each test and applies the migrations to it. It then runs each fixture the test asks
for, starting the test body thereafter. A fixture is a file of plain SQL in
`crates/slicket-store/fixtures`.

The happy path fixture, `happy_path_setup.sql`, fills that database with a tenant, the organisations the
tenant serves, the people in each organisation, the ticket types and the ticket statuses. Each
statement, thus far, has been written by hand.

The fixture now needs tickets. Some of those tickets may change after they are created, and some may
be deleted. Slicket needs a way to produce the SQL for them.

## Decision Drivers

* An independent starting point. The tests check `slicket-store` against the data in the fixture. A
  fixture that `slicket-store` built would inherit the bugs of the crate being tested.
* Few moving parts. The fewer parts the fixture depends on, the better.
* The same output on every run. Two runs of the same script print the same SQL.
* One tool for a developer and for a CI pipeline. The tool a developer runs by hand is the tool a
  future CI pipeline can run.

## Considered Options

* Tickets written by hand in the fixture
* A test helper that calls `slicket-store` at the start of each test
* A generator that calls `slicket-store` on a scratch database and writes out the resulting rows
* A generator that prints SQL text

## Decision Outcome

Chosen option: "A generator that prints SQL text", because the fixture then depends on the
generator alone.

The generator is a small command-line tool in the Slicket workspace. Its source contains the ticket
script, which is Rust code that describes each ticket and each change thereto. The tool turns the
script into `INSERT`, `UPDATE` and `DELETE` statements and prints them as text. That text is the
ticket part of the happy path fixture. PostgreSQL runs the statements when a test loads the fixture.

The tool runs without a database. It also leaves `slicket-store` out of its dependencies, thereby
keeping the fixture independent of the crate that the tests check.

The fixture exists to test `slicket-store` on a proper database.

The script uses little randomness. Where the script does use a random number generator, that
generator takes a fixed seed. As a result, every run prints the same SQL.

### Consequences

* Good, because a bug in `slicket-store` leaves the fixture unchanged. The ticket script is the one
  place a fault in the ticket data can come from.
* Good, because a developer generates the fixture with the Rust toolchain alone.
* Good, because every run prints the same SQL, meaning a change therein always traces back to a
  change in the script.
* Good, because a CI pipeline can run the same tool.
* Bad, because the script builds SQL as text, which the compiler leaves unchecked. A mistake in a
  statement shows up when a test loads the fixture and PostgreSQL rejects the statement.
* Bad, because the script states the columns of the `ticket` table a second time, outside the
  migrations. A migration that changes the table needs a matching change to the script.

## Pros and Cons of the Options

### Tickets written by hand in the fixture

The developer writes each ticket statement into `happy_path_setup.sql`, as the file already does for
organisations and people.

* Good, because the fixture needs no tool.
* Good, because the fixture is independent of `slicket-store`.
* Bad, because each ticket, and each change to a ticket, is a statement that a person writes and
  maintains by hand.

### A test helper that calls `slicket-store`

The SQL fixture ends at the ticket statuses. A shared test helper then calls the insert, update and
delete functions of `slicket-store`, and each test calls that helper first.

* Good, because the tickets come from the code the application uses.
* Bad, because the starting data of every test depends on the code being tested. A bug in the insert
  function fails every test at once.

### A generator that calls `slicket-store`

The tool runs the migrations and `happy_path_setup.sql` on a scratch database. It then calls the functions
of `slicket-store` in the order the script sets, reads the `ticket` table back and writes one
`INSERT` for each row thereof.

* Good, because the tickets come from the code the application uses.
* Bad, because the fixture breaks on a bug in `slicket-store` and on a bug in the script. The chosen
  option breaks on a bug in the script alone.
* Bad, because generating the fixture needs a running PostgreSQL, on a developer's machine and in
  CI.
* Bad, because it checks whether `slicket-store` can produce a proper database. The fixture exists
  for a different test, which is how `slicket-store` behaves on a proper database.

### A generator that prints SQL text

The tool turns the ticket script into SQL statements and prints them. It connects to nothing.

* Good, because the fixture depends on the generator alone.
* Good, because generating the fixture needs the Rust toolchain alone.
* Good, because it has the fewest parts of the three options that script the tickets.
* Bad, because the compiler leaves the generated SQL unchecked.
* Bad, because the script repeats the columns of the `ticket` table.

## Open Points

Three details are undecided at the date of this record. A later change to this file, or a later ADR,
records each one once the developer settles it.

* The name of the tool and its place in the workspace.
* Whether the tool's output is appended to `happy_path_setup.sql` or written to a fixture file that the
  tool owns.
* What a CI pipeline does with the tool beyond running it.

---
status: accepted
date: 2026-09-24
decision-makers: Kheagen Haskins
---

# Store Slicket's data in PostgreSQL

## Context and Problem Statement

Slicket stores organisations, the people in them, the ticket statuses each organisation defines,
and the tickets those people raise. The data is relational: a ticket refers to a person and a
status, and a person and a status each refer to an organisation. The schema is in third normal
form (3NF), so each fact is stored in one place and the tables refer to each other by key.

Slicket needs a database that stores this schema and enforces its rules, such as a ticket that
refers to a person who exists.

## Decision Drivers

* Enforced integrity. The database rejects a row that breaks the schema's rules, whichever code
  wrote it.
* Types that match the model. A point in time, such as a ticket's `created_at`, has a column type
  of its own.
* Safe migrations. A migration that fails partway leaves the schema as it was before the
  migration started.
* Many users at once. Several people raise and update tickets at the same time.
* Support in the Rust database libraries.

## Considered Options

* PostgreSQL
* MySQL
* SQLite

## Decision Outcome

Chosen option: "PostgreSQL", because it meets every driver above.

PostgreSQL enforces foreign keys and `CHECK` constraints on every write. The schema uses both: a
foreign key ties each ticket to a person, and a `CHECK` keeps a ticket's priority between 0 and
255. PostgreSQL stores a point in time as a `TIMESTAMPTZ`, and it numbers new rows through identity
columns (`GENERATED ALWAYS AS IDENTITY`).

PostgreSQL also runs schema changes inside a transaction. A migration that fails halfway through
therefore rolls back in full, and the database returns to the schema it had before.

### Consequences

* Good, because the database enforces the schema's rules for every writer, whether that is
  Slicket's code or a person running SQL by hand.
* Good, because a failed migration rolls back in full.
* Good, because a trigger can enforce a rule that a foreign key alone cannot, such as a ticket
  whose status belongs to the same organisation as the person who raised it.
* Bad, because PostgreSQL has signed integer types alone. Every key type in `slicket-core` wraps a
  signed `i32` as a result, and a key allows 2,147,483,647 values where an unsigned `u32` would
  allow twice as many.
* Bad, because PostgreSQL runs as a server of its own, which every developer and every deployment
  has to start and look after.

## Pros and Cons of the Options

### PostgreSQL

A database server that stores relational data and enforces its constraints.

* Good, because it enforces foreign keys and `CHECK` constraints.
* Good, because it runs schema changes inside a transaction.
* Good, because `TIMESTAMPTZ` stores a point in time with its time zone.
* Good, because sqlx, Diesel and SeaORM all support it.
* Bad, because it has signed integer types alone.

### MySQL

A database server that stores relational data, as PostgreSQL does.

* Good, because it has unsigned integer types, which match the `u32` keys `slicket-core` used
  before.
* Good, because sqlx, Diesel and SeaORM all support it.
* Bad, because a schema change commits the open transaction, so a migration that fails halfway
  through leaves the schema half changed.

### SQLite

A database stored in a single file, which the application opens as a library.

* Good, because it runs inside the application, with no server to start.
* Good, because sqlx, Diesel and SeaORM all support it.
* Bad, because one connection writes at a time, which limits how many users can update tickets at
  once.
* Bad, because a column takes a declared type only in a `STRICT` table, and a point in time is
  stored as text or a number.

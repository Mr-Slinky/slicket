//! The persistence layer of Slicket, which saves the components of `slicket-core` to a Postgres
//! database and loads them back.
//!
//! The crate talks to Postgres through [sqlx](https://docs.rs/sqlx). The schema lives as plain SQL
//! files in the `migrations` folder at the root of this crate. Each file is one migration, and sqlx
//! applies them in the order of the timestamp at the start of each file name.
//!
//! The schema maps each core component to a table of its own:
//!
//! | Component    | Table    |
//! |--------------|----------|
//! | `OrgCore`    | `org`    |
//! | `PersonCore` | `person` |
//! | `TicketCore` | `ticket` |
//!
//! An optional component that wraps a single value is a nullable column on its entity's table.
//! `ClosedAt` is the `closed_at` column of `ticket`, and `Description` is its `description` column.
//! A ticket has the component exactly when the column is not `NULL`.
//!
//! The `ticket_status` table stores the statuses a ticket can be in, and every ticket refers to one
//! of them through its `status_id` column.

mod database;
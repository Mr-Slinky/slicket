//! The persistence layer of Slicket. The goal of this crate is to save the components of
//! `slicket-core` to a PostgreSQL database and load them back. For now, the [`database`] module
//! connects to the database and applies the schema.
//!
//! The crate talks to PostgreSQL through [sqlx](https://docs.rs/sqlx). The schema lives as plain
//! SQL files in the `migrations` folder at the root of this crate. Each file is one migration, and
//! sqlx applies them in the order of the timestamp at the start of each file name.
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
//! The `Tenant` resource is the one row of the `tenant` table. The table's primary key is a
//! boolean that must be `true`, so the table accepts at most one row. That row's `org_id` column
//! refers to the organisation that is the tenant.
//!
//! Two lookup tables store the values a ticket chooses from. Every ticket refers to one row in each.
//!
//! | Table           | Stores                          | Column on `ticket` |
//! |-----------------|---------------------------------|--------------------|
//! | `ticket_status` | the statuses a ticket can be in | `status_id`        |
//! | `ticket_type`   | the types a ticket can have     | `ticket_type_id`   |

pub mod database;

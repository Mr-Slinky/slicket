# TODO

This document tracks the planned work on Slicket. Each item is a checkbox, which the developer ticks
once the work is done.

## Current scope

Slicket's scope is currently the ticket alone. The holy ticketing system's other modules, such as
opportunities, sales and documentation, are out of scope.

## Items

- [ ] **Build test fixtures for the database.** *(In progress.)*
  Each fixture starts as a TOML setup file in `crates/slicket-store/fixtures`. The `xtask` crate
  reads each setup file and writes a SQL file of the same name beside it. Running that SQL file
  fills a test database with known data, such as the tenant, its organisations and their people.
  The tests of `slicket-store` are then meant to run against that known data.

- [x] **Build an initialisation layer for the database.**
  The `slicket-store` crate opens a pool of connections to the PostgreSQL database and applies the
  migrations to it. A test runs those migrations on an empty database and checks that every table
  exists. This confirms that the crate can reach the database before it stores any ticket data.

- [x] **Flesh out a single ticket.**
  Work out what a ticket looks like in the backend's Entity Component System (ECS). The backend models several kinds of entity, such
  as tickets, people and organisations, and each kind has its own set of components. This item
  settles the ticket alone: which components make up a ticket, and what data each component stores.
  The design stays open to change as it develops.

# TODO

This document tracks the planned work on Slicket. Each item is a checkbox, ticked once the work is
done.

## Current scope

Slicket's scope is currently the ticket alone. The holy ticketing system's other modules, such as
opportunities, sales and documentation, are out of scope.

## Items

- [ ] **Build test fixtures for the database.** *(In progress.)*
  The fixtures are SQL files in `crates/slicket-store/fixtures` that fill a test database with known
  data, such as the tenant and its organisations. They give the tests of `slicket-store` a solid
  base to run against.

- [x] **Build an initialisation layer for the database.**
  The `slicket-store` crate opens a pool of connections to the PostgreSQL database and applies the
  migrations to it. This proves that the crate can reach the database before any ticket data goes
  through it.

- [x] **Flesh out a single ticket.**
  Work out what a ticket looks like in ECS terms. The backend models several kinds of entity, such
  as tickets, people and organisations, and each kind has its own set of components. This item
  settles the ticket alone: which components make up a ticket, and what data each component stores.
  The design stays open to change as it develops.

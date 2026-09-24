# TODO

This document tracks the planned work on Slicket. Each item is a checkbox, ticked once the work is
done.

## Current scope

Slicket's scope is currently the ticket alone. The holy ticketing system's other modules, such as
opportunities, sales and documentation, are out of scope.

## Items

- [ ] **Build an initialisation layer for the database.** *(Next.)*
  The `slicket-store` crate now defines the initial schema in its first migration. This item makes
  `slicket-store` open a connection to the PostgreSQL database and execute SQL against it. It proves
  that the crate can reach the database before any ticket data goes through it.

- [x] **Flesh out a single ticket.**
  Work out what a ticket looks like in ECS terms. The backend will model several kinds of entity,
  such as tickets, agents and departments. Each kind will have its own set of components, possibly
  tracked with bitmasks. This item settles the ticket alone: which components make up a ticket, and
  what data each component stores. The design stays open to change as it develops.

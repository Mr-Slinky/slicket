# Slicket

Slicket is a ticketing system written in Rust. A tenant is the business that runs an instance of
Slicket. The tenant sets up the organisations it serves and the people in each one. Those people can
then log tickets.

The name joins Slinky, my handle, with ticket.

## Background

Slicket takes its inspiration from a well-known IT service management (ITSM) and professional
services automation (PSA) platform, which I'll call the holy ticketing system. I have worked with the
holy ticketing system for a long time, and I find it complicated and unintuitive. I have long felt
that I could build a ticketing system that feels far better to use.

Slicket is that attempt. The aim is a system that feels better to use, rather than one that matches
the holy ticketing system feature for feature.

## Goals

- Prove to myself that I can build the improvements I spot in the software I use every day.
- Keep my Rust current through a project I can return to over the long term.
- Model the backend as an Entity Component System (ECS), both as a challenge and to learn how one
  works.

In an ECS, every object in the system is an entity, and an entity is only an id. Components are plain
data attached to entities. Systems are functions that run over every entity with a given set of
components.

## Status

Slicket is at an early design stage. [docs/TODO.md](docs/TODO.md) lists the planned work and the
scope it covers so far.

## Crates

Slicket is a Cargo workspace, with each crate in its own folder under `crates/`.

| Crate            | Contents                                                                  |
|------------------|---------------------------------------------------------------------------|
| `slicket-core`   | the ECS types: entity ids, components and resources                       |
| `slicket-store`  | the PostgreSQL schema, and the code that connects to the database         |
| `slicket-server` | the executable, which will serve the frontend that ADR 0004 describes     |
| `xtask`          | a developer tool that writes the SQL fixtures of `slicket-store`          |

The architecture decisions behind this layout are in [docs/decisions](docs/decisions/README.md).

## Building

Building Slicket needs a Rust toolchain that supports the 2024 edition, which means Rust 1.85 or
later. The tests also need a running PostgreSQL database, which `docker-compose.yml` provides.

A developer sets up the database once, from the repository root. The first command copies the
example environment file, which gives `DATABASE_URL` the address of the database that Docker
starts. The second command starts that database in the background.

```
cp .env.example .env
docker compose up -d
```

That developer then builds and tests every crate, again from the repository root:

```
cargo build
cargo test
```

Each test that uses the database creates a fresh database of its own and drops it once the test
passes. The `slicket` database therefore keeps whatever data a developer put in it.
`docker compose down` stops the database, and `docker-compose.yml` describes the other commands.

## Conventions

These conventions apply across the codebase.

### Ids and keys

Slicket identifies an object in two places: by its index in the ECS and by its row in the database.
The code uses one word for each.

- An **id** is an index in the ECS. An `Entity<T>` is an id, so a field or getter that returns an
  `Entity<T>` ends in `_id`, such as `PersonCore::org_id`.
- A **key** is a primary or foreign key in the database. Each kind of key has its own type ending in
  `Key`, such as `TicketKey`. A field or getter that returns a key ends in `_key`, such as
  `TicketCore::ticket_key`.

These two words govern Rust names alone. The database schema follows the usual SQL naming, meaning
its key columns end in `_id`, such as `person.org_id`. That column stores the organisation's key,
which the Rust code calls an `OrgKey`.

## Use of AI

I write Slicket's code myself, and I use AI (Claude Code) as a tutor and a peer reviewer. It explains
Rust concepts, reviews the code I write, writes documentation and writes commit messages. When I ask,
it also writes a single function to a specification I give it.

Work happens on the `dev` branch, where my work and the AI's work go in separate commits. The git
author of each commit records whose work it contains: my name on my commits, and
`Claude <noreply@anthropic.com>` on the AI's. I review every commit before it goes in, and I am the
committer on all of them.

The `main` branch receives `dev` as squash merges, each one a single commit that combines the work
since the last merge. `dev` therefore keeps the full record of who wrote what, and it stays
alongside `main` for good.

This command lists every commit on `dev` that contains the AI's work:

```
git log dev --author=Claude
```

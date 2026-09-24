# Slicket

Slicket is a ticketing system written in Rust. A user launches an instance of Slicket and sets up an
organisation and its employees. Those employees can then log tickets.

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

In an ECS, every object in the system is an entity, and an entity is only an ID. Components are plain
data attached to entities. Systems are functions that run over every entity with a given set of
components.

## Status

Slicket is at an early design stage.

## Building

Slicket is a Cargo workspace, with each crate in its own folder under `crates/`. Building it needs a
Rust toolchain that supports the 2024 edition, which means Rust 1.85 or later.

These commands build and test every crate, run from the repository root:

```
cargo build
cargo test
```

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

## Use of AI

I write Slicket's code myself, and I use AI (Claude Code) as a tutor and a peer reviewer. It explains
Rust concepts, reviews the code I write, writes documentation and writes commit messages. When I ask,
it also writes a single function to a specification I give it.

My work and the AI's work go in separate commits. The git author of each commit records whose work
it contains: my name on my commits, and `Claude <noreply@anthropic.com>` on the AI's. I review every
commit before it goes in, and I am the committer on all of them.

This command lists every commit that contains the AI's work:

```
git log --author=Claude
```

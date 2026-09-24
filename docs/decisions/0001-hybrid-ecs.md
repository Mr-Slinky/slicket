---
status: accepted
date: 2026-09-24
decision-makers: Kheagen Haskins
---

# Model the backend as a hybrid Entity Component System

## Context and Problem Statement

Slicket stores tickets, people and organisations. Every ticket has a few fields that all tickets
share, such as a title and a priority. Some tickets also have further data, such as the time they
were closed or a longer description. That further data comes and goes over the life of a ticket.

The backend needs a data model in which a ticket stores its further data only while the ticket has
it. The model should also let the compiler catch as many mistakes as it can.

An Entity Component System (ECS) is one such model. In an ECS, every object is an entity, and an
entity is only an id. Components are plain data attached to entities. Systems are functions that
run over every entity that has a given set of components.

Games use an ECS for speed as well. An ECS lays components out in memory so that the CPU cache
serves most reads, which matters to a game that draws at least 60 frames a second. A ticketing
system answers requests as users make them, so its workload differs from a frame loop.

## Decision Drivers

* Composition. A ticket gains and loses data as components, in place of a struct that grows an
  `Option` field for each piece of optional data.
* Type safety. The compiler rejects as many mistakes as it can, such as a ticket's id passed where
  a person's id is expected.
* Performance. Slicket aims to be fast. This is a decision: tuning memory layout for cache hits, as
  games do, falls outside that aim.
* Experimentation. Trying an unusual architecture is a goal of the project in its own right.

## Considered Options

* A struct per kind of object, with an `Option` field for each piece of optional data
* A pure ECS
* A hybrid ECS, which borrows from ECS where it helps

## Decision Outcome

Chosen option: "A hybrid ECS", because it gives Slicket the composition of an ECS and adds two
guarantees that a pure ECS leaves out.

Slicket takes components from ECS. Optional data, such as `ClosedAt` and `Description`, is a
component of its own, which an entity has only while the data exists. Slicket also takes the entity
id: an `Entity` pairs an id with a generation.

Slicket departs from a pure ECS in two places.

1. **A core component per kind of entity.** Each kind of entity has one component that stores the
   fields every entity of that kind has. The three core components are `TicketCore`, `PersonCore`
   and `OrgCore`. This is a decision: every entity of a kind will have its core component. The
   code defines the core components today, and nothing yet checks that an entity has one.
2. **A typed entity id per kind.** `Entity<T>` takes a marker type, `Ticket`, `Person` or `Org`,
   as `T`. `Entity<Ticket>` and `Entity<Person>` are separate types, so the compiler rejects an
   `Entity<Ticket>` passed where an `Entity<Person>` is expected. A pure ECS gives an entity no
   kind at all. Its kind there is whatever set of components it has at that moment. Slicket gives
   each entity a kind, and the compiler checks it. The check covers the kind of the id alone, so
   whether a given ticket has a `Description` is still a lookup at run time.

### Consequences

* Good, because a ticket stores its closing time and its description only while it has them.
* Good, because code that holds a `TicketCore` reads the ticket's title and priority directly,
  with no `Option` to unwrap.
* Good, because the compiler rejects an id of one kind passed where another kind is expected.
* Bad, because each object has two ids: its `Entity` in the ECS and its key in the database, such
  as `TicketKey`.
* Bad, because the rule that every entity has its core component depends on entity storage that
  Slicket has yet to write.

## Pros and Cons of the Options

### A struct per kind of object

A `Ticket` struct stores every field a ticket can have, with an `Option` field for each piece of
optional data.

* Good, because every field of a ticket is in one struct, where a reader finds it in one place.
* Bad, because the struct grows an `Option` field for each piece of optional data, and every ticket
  stores all of them.

### A pure ECS

Every piece of data is a component, and every entity shares one id type.

* Good, because entities compose freely from components.
* Bad, because every component is optional, so the model cannot promise that a ticket has a title.
* Bad, because a ticket's id and a person's id share one type, so the compiler accepts one where the
  other belongs.

### A hybrid ECS

Components as in a pure ECS, with a core component and a typed id for each kind of entity.

* Good, because optional data composes as components.
* Good, because the core component promises the fields every entity of its kind has.
* Good, because the typed id lets the compiler check the kind of every entity id.
* Bad, because the core component rule depends on entity storage that Slicket has yet to write.

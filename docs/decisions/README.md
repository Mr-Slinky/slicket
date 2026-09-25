# Architecture decision records

This folder contains Slicket's architecture decision records (ADRs), written in the
[MADR](https://adr.github.io/madr/) format. Each file records one decision, and its number sets
the order in which the decisions were made.

Slicket has one maintainer, so an ADR takes the status `accepted` when it is written.

| ADR                         | Decision                                               |
|-----------------------------|--------------------------------------------------------|
| [0001](0001-hybrid-ecs.md)  | Model the backend as a hybrid Entity Component System  |
| [0002](0002-postgres.md)    | Store Slicket's data in PostgreSQL                     |
| [0003](0003-sqlx.md)        | Access PostgreSQL through sqlx                         |
| [0004](0004-htmx-askama.md) | Render the frontend on the server with askama and htmx |

---
status: accepted
date: 2026-10-12
decision-makers: Kheagen Haskins
---

# Record each status change of a ticket in a history table

## Context and Problem Statement

Each ticket in Slicket has a status, such as New, In Progress or Closed. The `ticket` table stores
the current status in its `status_id` column, which refers to a row of `ticket_status`. The same
table stores the time at which the ticket was closed in its `closed_at` column. That column is
`NULL` for a ticket that is still open. In `slicket-core`, the optional `ClosedAt` component stores
the same time for a closed ticket.

A ticket can be closed and then reopened, and a reopened ticket can be closed again. The catalogue
of ticket scenarios lists this case as `LC-07`. The `closed_at` column stores one time, meaning it
keeps the most recent closing at best. On a reopen, the store either clears it and loses that
closing, or leaves it set on a ticket that is open again.

A second scenario meets the same limit. `CHG-01` is a ticket whose status changed several times
before reaching its current one. The fixture generator writes each change as an `UPDATE` to
`status_id`, and every `UPDATE` replaces the value before it. The database therefore keeps the last
status alone, with no record of the statuses the ticket passed through or when it entered each one.

Slicket needs a way to store every closing of a ticket, and every other change of its status.

## Decision Drivers

* A full record. The store keeps every closing and every reopening, with the time of each.
* One source of truth. Two places in the schema never give different answers to whether a ticket is
  closed.
* Questions about a ticket's past. The schema answers how long a ticket spent in a status, and how
  often it was reopened.
* Simple queries for the current state. Listing the open tickets stays a cheap query.

## Considered Options

* Keep the `closed_at` column on `ticket`
* Store each closing in a table of its own
* Store each status change in a history table

## Decision Outcome

Chosen option: "Store each status change in a history table", because one table then records every
closing, every reopening and every other change of status.

A new table, `ticket_status_change`, stores one row for each change. Each row refers to the ticket
and to the status the ticket moved into, and it records the time of the change. Creating a ticket
writes the first row, for the status the ticket starts in. Every later change of status adds a row
and leaves the earlier rows as they are.

The `closed_at` column leaves the `ticket` table. The store derives the facts it used to read from
that column, and the derived facts follow from the rows:

| Fact                     | Where it comes from                                    |
|--------------------------|--------------------------------------------------------|
| The current status       | the latest row for the ticket                          |
| Each closing time        | each row whose status is Closed                        |
| Each reopening           | each row that follows a row whose status is Closed     |
| The most recent closing  | the latest row for the ticket whose status is Closed   |

`LC-07` then has a plain shape in the fixture: a ticket whose rows run from New through Closed to
In Progress. `CHG-01` becomes a ticket with one row for each status it passed through.

### Consequences

* Good, because reopening a ticket keeps every earlier closing, with its time.
* Good, because the closing time of a ticket is always the time of a Closed row. A ticket in status
  Closed with no closing time, which is `INV-01`, can no longer be written. A ticket in status New
  with a closing time, which is `INV-02`, can no longer be written either.
* Good, because the time a ticket spent in each status is the gap between one row and the next.
* Good, because the backend can model the history as one component of a ticket entity, which fits
  the Entity Component System that ADR 0001 sets out.
* Bad, because the query for the current status of a ticket reads its latest row, where it used to
  read one column. Listing the open tickets needs an index on the ticket and the time of the change
  to stay fast.
* Bad, because a `CHECK` constraint reads one row alone. The rule that a change comes no earlier
  than the creation of its ticket spans two tables, which takes a trigger or the store's own code to
  enforce.
* Bad, because the `ClosedAt` component in `slicket-core`, the store and the fixture generator all
  change to match the new table.

## Pros and Cons of the Options

### Keep the `closed_at` column on `ticket`

The schema stays as it is. Reopening a ticket either clears `closed_at` or leaves it set.

* Good, because the schema and the code stay as they are.
* Good, because the current status and the closing time are each one column away.
* Bad, because a ticket closed twice loses its first closing time.
* Bad, because `status_id` and `closed_at` can disagree, as `INV-01` and `INV-02` show.
* Bad, because the earlier statuses of a ticket are lost, as `CHG-01` shows.

### Store each closing in a table of its own

A new table stores one row for each closing of a ticket. A ticket with two rows has been closed,
reopened and closed again.

* Good, because reopening a ticket keeps every earlier closing.
* Good, because the change to the schema is small, touching closings alone.
* Bad, because the closure table and `status_id` both say whether a ticket is closed, and the two
  can disagree.
* Bad, because the table records when a ticket was closed and leaves out when it was reopened. The
  status a reopened ticket moved into is lost.
* Bad, because every status other than Closed keeps the problem that `CHG-01` shows.

### Store each status change in a history table

A new table stores one row for each change of status, and the store derives the current status and
the closing times from those rows.

* Good, because one table records every closing, every reopening and every other change of status.
* Good, because the schema keeps one source of truth for the status of a ticket.
* Bad, because the current status takes a query over several rows.
* Bad, because the store, `slicket-core` and the fixture generator all change.

## Open Points

Two details are undecided. A later change to this file, or a later ADR, records each one once the
developer settles it.

* Whether `ticket` keeps `status_id` as a copy of the latest status. The store would then update
  that column in the same transaction that adds the history row. Listing the open tickets would then
  read one column again. The schema would also store the status in two places, which a bug in the
  store could leave disagreeing.
* What takes the place of the `ClosedAt` component in `slicket-core`.

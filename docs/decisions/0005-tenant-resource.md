---
status: accepted
date: 2026-09-28
decision-makers: Kheagen Haskins
---

# Model the tenant as a single resource that refers to an organisation

## Context and Problem Statement

A tenant is the business that runs an instance of Slicket. The tenant sets up the organisations it
serves, and the people in each organisation then log tickets. Each person belongs to one
organisation, which the `org_id` of their `PersonCore` identifies.

The tenant has staff of its own. Those staff are people too, and each of them needs an organisation
to belong to, in the same way as the people the tenant serves.

Slicket needs a way to store the tenant, both in the ECS of `slicket-core` and in the database.

## Decision Drivers

* One tenant per instance. An instance of Slicket runs for exactly one business.
* Staff as ordinary people. The tenant's staff belong to an organisation, which lets the code treat
  them as it treats every other person.
* One place for each detail. The database stores the tenant's name once.
* Enforced integrity. The database rejects a second tenant, whichever code tries to insert one.

## Considered Options

* A tenant table of its own, separate from the organisations
* A tenant table with one row per tenant, each referring to an organisation
* A single `Tenant` resource that refers to an organisation

## Decision Outcome

Chosen option: "A single `Tenant` resource that refers to an organisation", because it gives the
tenant's staff an organisation to belong to and allows one tenant per instance.

In an ECS, a resource is a single value that belongs to the whole world rather than to one entity.
`Tenant` is a resource in `slicket-core`. It stores the `Entity<Org>` of the organisation that is the
tenant. The tenant's name and other details are the components of that organisation, such as its
`OrgCore`.

The database stores the resource as the one row of the `tenant` table. The table's primary key,
`tenant_id`, is a `BOOLEAN` that defaults to `true`, and a `CHECK` rejects `false`. As a result, the
table accepts at most one row. The `org_id` column of that row refers to the tenant's organisation.
A table with one row can never repeat a value, which is why `org_id` takes no `UNIQUE` constraint.

### Consequences

* Good, because the tenant's staff are people in an organisation, like everyone else. Code that
  works with a `PersonCore` works with them unchanged.
* Good, because the database stores the tenant's name once, in the `org` table.
* Good, because the database rejects a second tenant row.
* Bad, because the schema also accepts an empty `tenant` table. This means the code that will load
  the `Tenant` resource has to deal with a missing row.
* Bad, because one database serves one business. A second business runs its own instance of
  Slicket, with its own database.

## Pros and Cons of the Options

### A tenant table of its own

The `tenant` table stores the tenant's name, and it has no link to the `org` table.

* Good, because the tenant and the organisations it serves stay apart in the schema.
* Bad, because every person belongs to an organisation, meaning the tenant's staff need an
  organisation that the tenant table has no link to.

### A tenant table with one row per tenant

Each row has a numbered key, a name, and an `org_id` that refers to the tenant's organisation. A
`UNIQUE` constraint on `org_id` stops two tenants from sharing one organisation.

* Good, because the tenant's staff belong to the tenant's organisation.
* Good, because one database can serve several businesses.
* Bad, because the database stores the tenant's name twice, once in `tenant` and once in `org`.
* Bad, because an instance of Slicket runs for one business, while the schema accepts any number of
  tenants.

### A single `Tenant` resource

The `Tenant` resource stores the `Entity<Org>` of the tenant's organisation, and the `tenant` table
accepts at most one row.

* Good, because the tenant's staff belong to the tenant's organisation.
* Good, because the database stores the tenant's name once.
* Good, because the database accepts at most one tenant.
* Bad, because the database also accepts no tenant at all.

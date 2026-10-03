---
status: accepted
date: 2026-10-03
decision-makers: Kheagen Haskins
---

# Store email addresses in lower case

## Context and Problem Statement

Each person in Slicket has an email address, which `slicket-core` stores as an `Email`. The database
stores the same address in the `email` column of the `person` table, where a unique index allows
one person per address.

An email address has two parts, separated by its last `@`. The local part comes before it and the
domain comes after it. RFC 5321, the standard for sending mail, treats the two parts differently in
section 2.4:

> The local-part of a mailbox MUST BE treated as case sensitive. Therefore, SMTP implementations
> MUST take care to preserve the case of mailbox local-parts. In particular, for some hosts, the
> user "smith" is different from the user "Smith". However, exploiting the case sensitivity of
> mailbox local-parts impedes interoperability and is discouraged. Mailbox domains follow normal DNS
> rules and are hence not case sensitive.

Under the standard, `Jane@example.com` and `jane@example.com` can therefore reach two different
mailboxes. A person typing their own address, though, uses whatever case comes to hand, and expects
Slicket to recognise them either way.

The Rust code and the database once applied different rules to the same pair of addresses. `Email`
derives `PartialEq` and `Hash` on its `String`, which compare bytes, and so treated the two
spellings as different addresses. The unique index, on `lower(email)`, treated them as the same
address. Code checking for a duplicate in memory accepted an address that the database then
rejected on insert.

Slicket needs one rule for when two email addresses are the same person.

## Decision Drivers

* One rule in both places. The Rust code and the database agree on whether two addresses are the
  same.
* One person per address. Two spellings of one address belong to one person.
* A rule enforced in one place. `Email` and the database each enforce the rule once, and every
  query and comparison then follows it without applying it again.
* Mail reaches the person. An address Slicket stores still delivers mail.

## Considered Options

* Store and compare each address exactly as entered
* Store each address as entered, and compare without regard to case
* Store each address in lower case

## Decision Outcome

Chosen option: "Store each address in lower case", because it gives the Rust code and the database
one rule that neither has to remember to apply.

`Email::new` trims the address it receives and converts it to lower case before running its checks.
The `Email` it returns stores that lower-case address. As a result, the `PartialEq` and `Hash` that
`Email` derives compare two addresses without regard to case. The `email` column has a `CHECK` that
the stored value equals `lower(email)`, meaning the database rejects an address in any other case,
whichever code tries to insert it.

This is a decision to depart from RFC 5321 for the local part. Slicket treats an email address as
the identity of a person, rather than as a mailbox it delivers to exactly as typed. The standard
itself discourages a mail host from relying on case in the local part. An address on a host that
does rely on it is the one address this decision handles badly.

### Consequences

* Good, because `Email` and the database agree on every pair of addresses.
* Good, because a person who types their address in a different case is still the same person.
* Good, because a plain `WHERE email = $1` with a lower-case value finds the person, and the derived
  traits on `Email` need no code of their own.
* Bad, because Slicket keeps no record of the case the person typed. Every place that shows the
  address shows it in lower case.
* Bad, because a mail host that treats `Jane` and `jane` as two mailboxes receives mail for the
  lower-case one. Mail that Slicket sends to such an address can reach the wrong mailbox, or none.

## Pros and Cons of the Options

### Store and compare each address exactly as entered

`Email` stores the address as typed and keeps its derived traits. The unique index moves to `email`
itself.

* Good, because Slicket follows RFC 5321 to the letter.
* Good, because mail always goes to the mailbox exactly as typed.
* Bad, because `Jane@example.com` and `jane@example.com` become two people, each able to log
  tickets on their own.
* Bad, because a person who types their address in a different case from the stored one goes
  unrecognised.

### Store each address as entered, and compare without regard to case

`Email` stores the address as typed. It implements `PartialEq` and `Hash` by hand, lowering both
sides before comparing them. The unique index stays on `lower(email)`, and each query that looks up
an address wraps both sides in `lower()`.

* Good, because Slicket keeps the case the person typed, and mail goes to the mailbox as typed.
* Good, because two spellings of one address belong to one person.
* Bad, because `PartialEq` and `Hash` need hand-written code that has to stay consistent with each
  other.
* Bad, because every query that compares addresses has to apply `lower()`. A query that leaves it
  out finds no match for a spelling in a different case, and nothing flags the mistake.

### Store each address in lower case

`Email::new` converts the address to lower case, and a `CHECK` on the `email` column accepts only
lower-case values.

* Good, because one rule holds in Rust and in the database, and both enforce it.
* Good, because two spellings of one address belong to one person.
* Bad, because the case the person typed is lost.
* Bad, because mail to a host that treats case in the local part as significant can go astray.

//! Components that describe a ticket.
//!
//! [`TicketCore`] stores the fields that every ticket has. [`ClosedAt`] and [`Description`] store
//! data that only some tickets have.

use crate::world::{Entity, Person};

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// `TicketCore` is the component that stores the six fields every ticket has.
///
/// | Field        | Type                      | Stores                                   |
/// |--------------|---------------------------|------------------------------------------|
/// | `ticket_key` | [`TicketKey`]             | the ticket's key in the database         |
/// | `status_key` | [`StatusKey`]             | the status the ticket is in              |
/// | `priority`   | [`Priority`]              | how urgent the ticket is                 |
/// | `title`      | `String`                  | the one-line summary of the ticket       |
/// | `raised_by`  | [`Entity<Person>`]        | the person who raised the ticket         |
/// | `created_at` | [`UnixEpochSeconds`]      | the time at which the ticket was created |
///
/// Data that only some tickets have goes in separate components, such as [`ClosedAt`] and
/// [`Description`].
///
/// The fields are private. A caller sets all six at once through [`TicketCore::new`], then reads
/// each one through the method of the same name, such as [`TicketCore::priority`]. A caller
/// changes `status_key`, `priority` and `title` afterwards through a setter, such as
/// [`TicketCore::set_priority`]. The other three fields keep the values that `new` gave them.
///
/// Only code inside `slicket-core` can create a `TicketKey`, a `StatusKey` or a `Priority`. As
/// such, only code inside `slicket-core` can create a `TicketCore`.
///
/// `TicketCore` derives `Clone`, so a caller copies one with `.clone()`. The copy is explicit
/// because `title` is a `String`, which owns memory on the heap. Two `TicketCore` values are equal
/// when all six fields are equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TicketCore {
    ticket_key: TicketKey,
    status_key: StatusKey,
    priority: Priority,
    title: String,
    raised_by: Entity<Person>,
    created_at: UnixEpochSeconds,
}

impl TicketCore {
    /// Creates a `TicketCore` from the six fields that every ticket has.
    pub fn new(
        ticket_key: TicketKey,
        status_key: StatusKey,
        priority: Priority,
        title: String,
        raised_by: Entity<Person>,
        created_at: UnixEpochSeconds,
    ) -> Self {
        Self {
            ticket_key,
            status_key,
            priority,
            title,
            raised_by,
            created_at,
        }
    }

    /// Returns the `TicketKey` of this ticket.
    pub fn ticket_key(&self) -> TicketKey {
        self.ticket_key
    }

    /// Returns the `StatusKey` of this ticket.
    pub fn status_key(&self) -> StatusKey {
        self.status_key
    }

    /// Returns the `Priority` of this ticket.
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Returns the title of this ticket.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the person who raised this ticket.
    pub fn raised_by(&self) -> Entity<Person> {
        self.raised_by
    }

    /// Returns the time at which this ticket was created.
    pub fn created_at(&self) -> UnixEpochSeconds {
        self.created_at
    }

    /// Sets the `StatusKey` of this ticket.
    pub fn set_status_key(&mut self, status_key: StatusKey) {
        self.status_key = status_key;
    }

    /// Sets the `Priority` of this ticket.
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    /// Sets the title of this ticket.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

/// A `TicketKey` is the key that identifies a ticket in the database.
///
/// A `TicketKey` and an [`Entity<Ticket>`] both identify a ticket, but they are separate values.
/// The `Entity<Ticket>` is the ticket's index in the ECS, while the `TicketKey` is the ticket's key
/// in the database.
///
/// Only code inside `slicket-core` can create a `TicketKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a Postgres `INTEGER`, which is signed, and numbers keys
/// from 1 upwards. An `i32` therefore allows 2,147,483,647 tickets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TicketKey(pub(crate) i32);

impl TicketKey {
    /// Returns the number that identifies this ticket.
    pub fn get(self) -> i32 {
        self.0
    }
}

/// A `StatusKey` is the key that identifies a ticket status in the database.
///
/// Only code inside `slicket-core` can create a `StatusKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a Postgres `INTEGER`, which is signed, and numbers keys
/// from 1 upwards. An `i32` therefore allows 2,147,483,647 statuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusKey(pub(crate) i32);

impl StatusKey {
    /// Returns the number that identifies this status.
    pub fn get(self) -> i32 {
        self.0
    }
}

/// A `Priority` sets how urgent a ticket is, as a number.
///
/// Only code inside `slicket-core` can create a `Priority`. The number is a `u8`, which allows 256
/// priority levels. A higher number means a more urgent ticket, so 0 is the lowest priority and
/// `u8::MAX` (255) is the highest.
///
/// `Priority` derives `Ord`, so a less urgent priority compares as less than a more urgent one.
/// Sorting tickets by `Priority` therefore puts the least urgent ticket first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(pub(crate) u8);

/// A `UnixEpochSeconds` stores a point in time, in whole seconds since the Unix epoch
/// (1970-01-01 00:00:00 UTC).
///
/// `UnixEpochSeconds` derives `Ord`, so an earlier time compares as less than a later one.
/// Sorting a list of them therefore puts the earliest time first.
///
/// # Examples
///
/// A caller creates a `UnixEpochSeconds` from a Unix timestamp and compares two of them with `<`.
///
/// ```
/// use slicket_core::ticket::UnixEpochSeconds;
///
/// let earlier = UnixEpochSeconds(1_735_689_600); // 2025-01-01 00:00:00 UTC
/// let later = UnixEpochSeconds(1_735_693_200); // 2025-01-01 01:00:00 UTC
///
/// assert!(earlier < later);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnixEpochSeconds(pub u64);

/// `ClosedAt` stores the time at which a ticket was closed.
///
/// `ClosedAt` is an optional component, meant only for closed tickets. It derives `Ord`, so sorting
/// closed tickets by `ClosedAt` puts the earliest closure first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClosedAt(pub UnixEpochSeconds);

/// `Description` stores the longer text that explains a ticket, beyond its title.
///
/// `Description` is an optional component, since a title is enough for some tickets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description(pub String);

//! Components that describe a ticket.
//!
//! Every ticket entity has a [`TicketCore`]. The other two components are optional, and a ticket
//! entity has each one only where the data applies to it.
//!
//! | Component       | Found on                      | Stores                                      |
//! |-----------------|-------------------------------|---------------------------------------------|
//! | [`TicketCore`]  | every ticket                  | the seven fields every ticket has           |
//! | [`ClosedAt`]    | closed tickets                | the time at which the ticket was closed     |
//! | [`Description`] | tickets with a longer account | the text that explains the ticket in detail |

use crate::world::{Entity, Key, Person, Ticket};

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// `TicketCore` is the component that stores the seven fields every ticket has.
///
/// | Field        | Type                      | Stores                                   |
/// |--------------|---------------------------|------------------------------------------|
/// | `ticket_key` | [`TicketKey`]             | the ticket's key in the database         |
/// | `type_key`   | [`TicketTypeKey`]         | the type of the ticket                   |
/// | `status_key` | [`StatusKey`]             | the status the ticket is in              |
/// | `priority`   | [`Priority`]              | how urgent the ticket is                 |
/// | `title`      | `String`                  | the one-line summary of the ticket       |
/// | `raised_by`  | [`Entity<Person>`]        | the person who raised the ticket         |
/// | `created_at` | [`UnixEpochSeconds`]      | the time at which the ticket was created |
///
/// Data that only some tickets have goes in separate components, such as [`ClosedAt`] and
/// [`Description`].
///
/// The fields are private. A caller sets all seven at once through [`TicketCore::new`], then reads
/// each one through the method of the same name, such as [`TicketCore::priority`]. A caller
/// changes `status_key`, `priority` and `title` afterwards through a setter, such as
/// [`TicketCore::set_priority`]. The other four fields keep the values that `new` gave them.
///
/// Only code inside `slicket-core` can create a `TicketKey`, a `TicketTypeKey`, a `StatusKey` or a
/// `Priority`. As such, only code inside `slicket-core` can create a `TicketCore`.
///
/// `TicketCore` derives `Clone`. A caller therefore copies one with `.clone()`. The copy is
/// explicit because `title` is a `String`, which owns memory on the heap. Two `TicketCore`
/// values are equal when all seven fields are equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TicketCore {
    ticket_key: TicketKey,
    type_key: TicketTypeKey,
    status_key: StatusKey,
    priority: Priority,
    title: String,
    raised_by: Entity<Person>,
    created_at: UnixEpochSeconds,
}

impl TicketCore {
    /// Creates a `TicketCore` from the seven fields that every ticket has.
    pub fn new(
        ticket_key: TicketKey,
        type_key: TicketTypeKey,
        status_key: StatusKey,
        priority: Priority,
        title: String,
        raised_by: Entity<Person>,
        created_at: UnixEpochSeconds,
    ) -> Self {
        Self {
            ticket_key,
            type_key,
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

    /// Returns the `TicketTypeKey` of this ticket.
    pub fn type_key(&self) -> TicketTypeKey {
        self.type_key
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
/// decision: the database stores the key as a PostgreSQL `INTEGER`, which is signed, and numbers
/// keys from 1, upwards. An `i32` therefore allows 2,147,483,647 tickets.
pub type TicketKey = Key<Ticket>;

/// A `TicketTypeKey` is the key that identifies a ticket type in the database.
///
/// Only code inside `slicket-core` can create a `TicketTypeKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a PostgreSQL `INTEGER`, which is signed, and numbers
/// keys from 1, upwards. An `i32` therefore allows 2,147,483,647 ticket types.
pub type TicketTypeKey = Key<TicketType>;

pub struct TicketType;

/// A `StatusKey` is the key that identifies a ticket status in the database.
///
/// Only code inside `slicket-core` can create a `StatusKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a PostgreSQL `INTEGER`, which is signed, and numbers
/// keys from 1, upwards. An `i32` therefore allows 2,147,483,647 statuses.
pub type StatusKey = Key<Status>;

pub struct Status;

/// A `Priority` sets how urgent a ticket is, as a number.
///
/// Only code inside `slicket-core` can create a `Priority`. The number is a `u8`, which allows 256
/// priority levels. A higher number means a more urgent ticket. As a result, 0 is the lowest
/// priority and `u8::MAX` (255) is the highest.
///
/// `Priority` derives `Ord`, meaning a less urgent priority compares as less than a more urgent
/// one. Sorting tickets by `Priority` therefore puts the least urgent ticket first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(pub(crate) u8);

impl From<u8> for Priority {
    fn from(priority: u8) -> Self {
        Self(priority)
    }
}

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
/// `ClosedAt` is an optional component, meant only for closed tickets. Since it derives `Ord`,
/// sorting closed tickets by `ClosedAt` puts the earliest closure first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClosedAt(pub UnixEpochSeconds);

/// `Description` stores the longer text that explains a ticket, beyond its title.
///
/// `Description` is an optional component, since a title is enough for some tickets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description(pub String);

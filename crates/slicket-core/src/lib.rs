//! The core of Slicket, a ticketing system modelled as an Entity Component System (ECS).
//!
//! In an ECS, every object in the system is an entity, and an entity is only an id. Components
//! are plain data attached to entities. Systems are functions that run over every entity that has
//! a given set of components.
//!
//! The crate is split into these modules:
//!
//! - [`world`] defines [`Entity`], the id of one object, and the marker types [`Ticket`],
//!   [`Person`] and [`Org`] that state which kind of object an `Entity` identifies.
//! - [`ticket`] defines the components that describe a ticket, starting with [`TicketCore`].
//! - [`person`] defines the components that describe a person and an organisation,
//!   [`PersonCore`] and [`OrgCore`].
//!
//! The crate root re-exports every public type, so a caller can write `slicket_core::TicketCore`
//! in place of `slicket_core::ticket::TicketCore`.

pub mod person;
pub mod ticket;
pub mod world;

pub use person::{OrgCore, OrgKey, PersonCore, PersonKey};
pub use ticket::{
    ClosedAt, Description, Priority, StatusKey, TicketCore, TicketKey, UnixEpochSeconds,
};
pub use world::{Entity, Org, Person, Ticket};

//! The types that identify objects in the ECS, and the resources that belong to the whole world.
//!
//! [`Entity`] identifies one object. Its type parameter is one of the marker types [`Ticket`],
//! [`Person`] or [`Org`], which states the kind of object the entity identifies. [`Tenant`] is a
//! resource, which stores the organisation that runs this instance of Slicket.

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// An `Entity` identifies one object in Slicket's ECS, such as a ticket.
///
/// Each `Entity` pairs an id with a generation. `Entity` derives `PartialEq`, meaning two values
/// are equal when their ids match and their generations match.
///
/// The type parameter `T` is a marker type, such as [`Ticket`] or [`Person`], that states the kind
/// of object the entity identifies. `Entity<Ticket>` and `Entity<Person>` are separate types. As a
/// result, the compiler rejects an `Entity<Ticket>` passed where an `Entity<Person>` is expected.
/// `T` exists only at compile time, so creating an `Entity` never requires a value of `T`.
///
/// # Examples
///
/// ```
/// use slicket_core::{Entity, Person, Ticket};
///
/// let ticket = Entity::<Ticket>::new(0, 0);
/// let raised_by: Entity<Person> = Entity::new(1, 0);
///
/// assert_eq!(ticket.id(), 0);
/// assert_eq!(raised_by.generation(), 0);
/// ```
pub struct Entity<T> {
    pub(crate) id: u32,
    pub(crate) generation: u32,
    _kind: PhantomData<fn() -> T>,
}

impl<T> Entity<T> {
    /// Creates an entity from an id and a generation.
    ///
    /// The caller picks `T` by writing it, as in `Entity::<Ticket>::new(0, 0)`, or leaves the
    /// compiler to infer it from where the result is used. Passing the result to a parameter of
    /// type `Entity<Person>`, for example, sets `T` to [`Person`].
    pub fn new(id: u32, generation: u32) -> Self {
        Self {
            id,
            generation,
            _kind: PhantomData,
        }
    }

    /// Returns the id of this entity.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the generation of this entity.
    pub fn generation(&self) -> u32 {
        self.generation
    }
}

impl<T> Clone for Entity<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Entity<T> {}

impl<T> PartialEq for Entity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.generation == other.generation
    }
}

impl<T> Eq for Entity<T> {}

impl<T> PartialOrd for Entity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Entity<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.id, self.generation).cmp(&(other.id, other.generation))
    }
}

impl<T> Hash for Entity<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.generation.hash(state);
    }
}

impl<T> Debug for Entity<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("id", &self.id)
            .field("generation", &self.generation)
            .finish()
    }
}

pub struct Key<T> {
    pub(crate) value: i32,
    _kind: PhantomData<fn() -> T>,
}

impl<T> Key<T> {
    pub fn get(self) -> i32 {
        self.value
    }
}

impl<T> TryFrom<i32> for Key<T> {
    type Error = KeyError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 1 {
            Ok(Self {
                value,
                _kind: PhantomData,
            })
        } else {
            Err(KeyError(value))
        }
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Key<T> {}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Key<T> {}

impl<T> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Hash for Key<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> Debug for Key<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Key").field(&self.value).finish()
    }
}

/// Marks an [`Entity`] as a ticket, as in `Entity<Ticket>`.
///
/// `Ticket` is a unit struct, which callers use only as the type parameter of an `Entity`.
pub struct Ticket;

/// Marks an [`Entity`] as a person, such as an employee who raises a ticket, as in
/// `Entity<Person>`.
///
/// `Person` is a unit struct, which callers use only as the type parameter of an `Entity`.
pub struct Person;

/// Marks an [`Entity`] as an organisation, such as the one a person belongs to, as in
/// `Entity<Org>`.
///
/// `Org` is a unit struct, which callers use only as the type parameter of an `Entity`.
pub struct Org;

/// A `Tenant` is the business that runs this instance of Slicket.
///
/// `Tenant` is an ECS resource. A resource is a single value that belongs to the whole world
/// rather than to one entity, and an instance of Slicket has exactly one tenant.
///
/// The tenant is also an organisation. `Tenant` therefore stores the [`Entity<Org>`] of that
/// organisation. The tenant's name and other details are the components of that organisation.
///
/// # Examples
///
/// ```
/// use slicket_core::{Entity, Org, Tenant};
///
/// let org: Entity<Org> = Entity::new(0, 0);
/// let tenant = Tenant::new(org);
///
/// assert_eq!(tenant.org(), org);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tenant {
    org: Entity<Org>,
}

impl Tenant {
    /// Creates the tenant whose details belong to the organisation `org`.
    pub fn new(org: Entity<Org>) -> Self {
        Self { org }
    }

    /// Returns the `Entity<Org>` of the organisation that is the tenant.
    pub fn org(&self) -> Entity<Org> {
        self.org
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyError(pub(crate) i32);

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "the key {} is below 1", self.0)
    }
}

impl std::error::Error for KeyError {}

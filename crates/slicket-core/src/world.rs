//! The types that identify objects, and the resources that belong to the whole world.
//!
//! [`Entity`] identifies one object in the ECS. Its type parameter is one of the marker types
//! [`Ticket`], [`Person`] or [`Org`], which states the kind of object the entity identifies.
//! [`Key`] identifies one row in the database, and its type parameter states the kind of row.
//!
//! A resource is a single value that belongs to the whole world rather than to one entity. This
//! module defines these resources:
//!
//! | Resource   | Stores                                                               |
//! |------------|----------------------------------------------------------------------|
//! | [`Tenant`] | the `Entity<Org>` of the business that runs this instance of Slicket |
//! | [`Lookup`] | every value of one kind, each under its `Key`                        |

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// ========================================================================================== \\
//                                       Entity Struct                                        \\
// ========================================================================================== \\
/// An `Entity` identifies one object in Slicket's ECS, such as a ticket.
///
/// Each `Entity` pairs an id with a generation. `Entity` implements `PartialEq`, meaning two values
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

// ========================================================================================== \\
//                                       Marker Structs                                       \\
// ========================================================================================== \\
/// Marks an [`Entity`] or a [`Key`] as a ticket, as in `Entity<Ticket>` and `Key<Ticket>`.
///
/// `Ticket` is a unit struct, which callers use only as the type parameter of an `Entity` or a
/// `Key`.
pub struct Ticket;

/// Marks an [`Entity`] or a [`Key`] as a person, such as an employee who raises a ticket, as in
/// `Entity<Person>` and `Key<Person>`.
///
/// `Person` is a unit struct, which callers use only as the type parameter of an `Entity` or a
/// `Key`.
pub struct Person;

/// Marks an [`Entity`] or a [`Key`] as an organisation, such as the one a person belongs to, as in
/// `Entity<Org>` and `Key<Org>`.
///
/// `Org` is a unit struct, which callers use only as the type parameter of an `Entity` or a `Key`.
pub struct Org;

// ========================================================================================== \\
//                                         Key Struct                                         \\
// ========================================================================================== \\
/// A `Key` is the number that identifies one row in the database, such as the row of a ticket.
///
/// The type parameter `T` states the kind of row the key identifies. `Key<Ticket>` and
/// `Key<Person>` are separate types. As a result, the compiler rejects a `Key<Ticket>` passed
/// where a `Key<Person>` is expected. `T` exists only at compile time, meaning creating a `Key`
/// never requires a value of `T`.
///
/// The number is an `i32`, and it is always 1 or higher. This is a decision: the database stores
/// a key as a PostgreSQL `INTEGER`, which is signed, and numbers keys from 1 upwards. An `i32`
/// therefore allows 2,147,483,647 keys of each kind.
///
/// A caller creates a `Key` from an `i32` with `try_from`, which returns a [`KeyError`] for a
/// number below 1. A caller reads the number back through [`Key::get`].
///
/// Two keys are equal when their numbers are equal. A key with a lower number compares as less
/// than a key with a higher one.
///
/// # Examples
///
/// ```
/// use slicket_core::{Key, Ticket};
///
/// let key = Key::<Ticket>::try_from(42).unwrap();
/// assert_eq!(key.get(), 42);
///
/// assert!(Key::<Ticket>::try_from(0).is_err());
/// ```
pub struct Key<T> {
    value: i32,
    _kind: PhantomData<fn() -> T>,
}

impl<T> Key<T> {
    /// Returns the number of this key.
    pub fn get(self) -> i32 {
        self.value
    }
}

impl<T> TryFrom<i32> for Key<T> {
    type Error = KeyError;

    /// Creates a key from `value`, or returns a [`KeyError`] where `value` is below 1.
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

/// A `KeyError` reports that a number is below 1, the lowest number a [`Key`] accepts.
///
/// `KeyError` stores the rejected number. The `Display` implementation returns a message that
/// includes it, such as "the key 0 is below 1". `KeyError` also implements [`std::error::Error`],
/// meaning the `?` operator can convert it into a `Box<dyn Error>`.
///
/// # Examples
///
/// ```
/// use slicket_core::{Key, Ticket};
///
/// let error = Key::<Ticket>::try_from(0).unwrap_err();
///
/// assert_eq!(error.to_string(), "the key 0 is below 1");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyError(pub(crate) i32);

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "the key {} is below 1", self.0)
    }
}

impl std::error::Error for KeyError {}

// ========================================================================================== \\
//                                       Tenant Struct                                        \\
// ========================================================================================== \\
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

// ========================================================================================== \\
//                                       Lookup Struct                                        \\
// ========================================================================================== \\
/// A `Lookup` stores every value of one kind, each under the [`Key`] that identifies it in the
/// database.
///
/// `Lookup` is an ECS resource. A resource is a single value that belongs to the whole world
/// rather than to one entity. A `Lookup` is meant for the values that many entities share. An
/// entity then stores the `Key` alone, and a caller passes that key to [`Lookup::find`] to read
/// the value.
///
/// The type parameter `T` is the type of the stored values. A `Lookup<T>` accepts only a
/// `Key<T>`. As a result, the compiler rejects a key of another kind.
///
/// # Examples
///
/// A caller stores one name under the key 1, then looks up the keys 1 and 2.
///
/// ```
/// use slicket_core::Key;
/// use slicket_core::world::Lookup;
///
/// let first = Key::<String>::try_from(1).unwrap();
/// let second = Key::<String>::try_from(2).unwrap();
///
/// let mut names = Lookup::new();
/// names.insert(first, String::from("Incident"));
///
/// assert_eq!(names.find(first), Some(&String::from("Incident")));
/// assert_eq!(names.find(second), None);
/// ```
#[derive(Debug, Clone)]
pub struct Lookup<T> {
    values: BTreeMap<Key<T>, T>,
}

impl<T> Lookup<T> {
    /// Creates an empty `Lookup`.
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Stores `value` under `key`.
    ///
    /// Returns the value that `key` identified before the call, or `None` where `key` is new to
    /// this `Lookup`.
    pub fn insert(&mut self, key: Key<T>, value: T) -> Option<T> {
        self.values.insert(key, value)
    }

    /// Returns the value stored under `key`, or `None` where this `Lookup` stores no value under
    /// that key.
    pub fn find(&self, key: Key<T>) -> Option<&T> {
        self.values.get(&key)
    }
}

impl<T> Default for Lookup<T> {
    fn default() -> Self {
        Self::new()
    }
}

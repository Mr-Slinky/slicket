use std::marker::PhantomData;

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// An `Entity` identifies one object in Slicket's ECS, such as a ticket.
///
/// Each `Entity` pairs an id with a generation. `Entity` derives `PartialEq`, so two values are
/// equal when their ids match and their generations match.
///
/// The type parameter `T` is a marker type, such as [`Ticket`] or [`Person`], that states the kind
/// of object the entity identifies. `Entity<Ticket>` and `Entity<Person>` are separate types, so
/// the compiler rejects an `Entity<Ticket>` passed where an `Entity<Person>` is expected. `T`
/// exists only at compile time, so creating an `Entity` never requires a value of `T`.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Marks an [`Entity`] as a ticket, as in `Entity<Ticket>`.
///
/// `Ticket` is a unit struct, which callers use only as the type parameter of an `Entity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ticket;

/// Marks an [`Entity`] as a person, such as an employee who raises a ticket, as in
/// `Entity<Person>`.
///
/// `Person` is a unit struct, which callers use only as the type parameter of an `Entity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Person;

/// Marks an [`Entity`] as an organisation, such as the one a person belongs to, as in
/// `Entity<Org>`.
///
/// `Org` is a unit struct, which callers use only as the type parameter of an `Entity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Org;

use std::marker::PhantomData;

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity<T> {
    pub(crate) id: u32,
    pub(crate) generation: u32,
    _kind: PhantomData<fn() -> T>,
}

impl<T> Entity<T> {
    pub fn new(id: u32, generation: u32) -> Self {
        Self {
            id,
            generation,
            _kind: PhantomData,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ticket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Person;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Org;

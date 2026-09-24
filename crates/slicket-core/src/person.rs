//! Components that describe a person and the organisation they belong to.
//!
//! [`PersonCore`] stores the fields that every person has, and [`OrgCore`] stores the fields that
//! every organisation has.

use crate::{Entity, Org};

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// `PersonCore` is the component that stores the three fields every person has.
///
/// | Field        | Type            | Stores                                 |
/// |--------------|-----------------|----------------------------------------|
/// | `person_key` | [`PersonKey`]   | the person's key in the database       |
/// | `org_id`     | [`Entity<Org>`] | the organisation the person belongs to |
/// | `name`       | `String`        | the person's name                      |
///
/// The fields are private. A caller sets all three at once through [`PersonCore::new`], then reads
/// each one through the method of the same name, such as [`PersonCore::name`]. A caller changes
/// `org_id` and `name` afterwards through [`PersonCore::set_org_id`] and [`PersonCore::set_name`].
/// `person_key` keeps the value that `new` gave it.
///
/// Only code inside `slicket-core` can create a `PersonKey`. As such, only code inside
/// `slicket-core` can create a `PersonCore`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonCore {
    person_key: PersonKey,
    org_id: Entity<Org>,
    name: String,
}

impl PersonCore {
    /// Creates a `PersonCore` from the three fields that every person has.
    pub fn new(person_key: PersonKey, org_id: Entity<Org>, name: String) -> Self {
        Self {
            person_key,
            org_id,
            name,
        }
    }

    /// Returns the `PersonKey` of this person.
    pub fn person_key(&self) -> PersonKey {
        self.person_key
    }

    /// Returns the entity of the organisation this person belongs to.
    pub fn org_id(&self) -> Entity<Org> {
        self.org_id
    }

    /// Returns the name of this person.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the entity of the organisation this person belongs to.
    pub fn set_org_id(&mut self, org_id: Entity<Org>) {
        self.org_id = org_id;
    }

    /// Sets the name of this person.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

/// `OrgCore` is the component that stores the two fields every organisation has.
///
/// | Field     | Type       | Stores                                 |
/// |-----------|------------|----------------------------------------|
/// | `org_key` | [`OrgKey`] | the organisation's key in the database |
/// | `name`    | `String`   | the organisation's name                |
///
/// The fields are private. A caller sets both at once through [`OrgCore::new`], then reads each
/// one through the method of the same name, such as [`OrgCore::name`]. A caller changes `name`
/// afterwards through [`OrgCore::set_name`]. `org_key` keeps the value that `new` gave it.
///
/// Only code inside `slicket-core` can create an `OrgKey`. As such, only code inside
/// `slicket-core` can create an `OrgCore`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrgCore {
    org_key: OrgKey,
    name: String,
}

impl OrgCore {
    /// Creates an `OrgCore` from the two fields that every organisation has.
    pub fn new(org_key: OrgKey, name: String) -> Self {
        Self { org_key, name }
    }

    /// Returns the `OrgKey` of this organisation.
    pub fn org_key(&self) -> OrgKey {
        self.org_key
    }

    /// Returns the name of this organisation.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of this organisation.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

/// An `OrgKey` is the key that identifies an organisation in the database.
///
/// An `OrgKey` and an [`Entity<Org>`] both identify an organisation, but they are separate values.
/// The `Entity<Org>` is the organisation's index in the ECS, while the `OrgKey` is the
/// organisation's key in the database.
///
/// Only code inside `slicket-core` can create an `OrgKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a Postgres `INTEGER`, which is signed, and numbers keys
/// from 1 upwards. An `i32` therefore allows 2,147,483,647 organisations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrgKey(pub(crate) i32);

impl OrgKey {
    /// Returns the number that identifies this organisation.
    pub fn get(self) -> i32 {
        self.0
    }
}

/// A `PersonKey` is the key that identifies a person in the database.
///
/// A `PersonKey` and an [`Entity<Person>`] both identify a person, but they are separate values.
/// The `Entity<Person>` is the person's index in the ECS, while the `PersonKey` is the person's key
/// in the database.
///
/// Only code inside `slicket-core` can create a `PersonKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a Postgres `INTEGER`, which is signed, and numbers keys
/// from 1 upwards. An `i32` therefore allows 2,147,483,647 people.
///
/// [`Entity<Person>`]: crate::Entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonKey(pub(crate) i32);

impl PersonKey {
    /// Returns the number that identifies this person.
    pub fn get(self) -> i32 {
        self.0
    }
}

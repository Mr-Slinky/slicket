//! Components that describe a person and the organisation they belong to.
//!
//! [`PersonCore`] stores the fields that every person has, and [`OrgCore`] stores the fields that
//! every organisation has. A person's email address is an [`Email`], which [`Email::new`] checks
//! before it creates one.

use crate::{Entity, Key, Org, Person};
use std::fmt;
use std::fmt::{Display, Formatter};

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// `PersonCore` is the component that stores the four fields every person has.
///
/// | Field        | Type            | Stores                                 |
/// |--------------|-----------------|----------------------------------------|
/// | `person_key` | [`PersonKey`]   | the person's key in the database       |
/// | `org_id`     | [`Entity<Org>`] | the organisation the person belongs to |
/// | `name`       | `String`        | the person's name                      |
/// | `email`      | [`Email`]       | the person's email address             |
///
/// The fields are private. A caller sets all four at once through [`PersonCore::new`], then reads
/// each one through the method of the same name, such as [`PersonCore::name`]. A caller changes
/// `name` afterwards through [`PersonCore::set_name`]. The other three fields keep the values that
/// `new` gave them.
///
/// Only code inside `slicket-core` can create a `PersonKey`. As such, only code inside
/// `slicket-core` can create a `PersonCore`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonCore {
    person_key: PersonKey,
    org_id: Entity<Org>,
    name: String,
    email: Email,
}

impl PersonCore {
    /// Creates a `PersonCore` from the four fields that every person has.
    pub fn new(person_key: PersonKey, org_id: Entity<Org>, name: String, email: Email) -> Self {
        Self {
            person_key,
            org_id,
            name,
            email,
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

    /// Returns the email address of this person.
    pub fn email(&self) -> &Email {
        &self.email
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
/// decision: the database stores the key as a PostgreSQL `INTEGER`, which is signed, and numbers
/// keys from 1 upwards. An `i32` therefore allows 2,147,483,647 organisations.
pub type OrgKey = Key<Org>;

/// A `PersonKey` is the key that identifies a person in the database.
///
/// A `PersonKey` and an [`Entity<Person>`] both identify a person, but they are separate values.
/// The `Entity<Person>` is the person's index in the ECS, while the `PersonKey` is the person's key
/// in the database.
///
/// Only code inside `slicket-core` can create a `PersonKey`. The number is an `i32`. This is a
/// decision: the database stores the key as a PostgreSQL `INTEGER`, which is signed, and numbers
/// keys from 1 upwards. An `i32` therefore allows 2,147,483,647 people.
///
/// [`Entity<Person>`]: crate::Entity
pub type PersonKey = Key<Person>;

/// An `Email` is a person's email address, checked when it is created.
///
/// The field is private, meaning [`Email::new`] is the only way to create an `Email`. As a result,
/// every `Email` has passed the checks that `new` runs. A caller reads the address back through
/// [`Email::as_str`].
///
/// The checks catch common typing mistakes. Only a message sent to the address proves that the
/// address receives mail.
///
/// ```
/// use slicket_core::person::{Email, EmailError};
///
/// let email = Email::new("  jane@example.com ").unwrap();
/// assert_eq!(email.as_str(), "jane@example.com");
///
/// assert_eq!(Email::new("jane.example.com"), Err(EmailError::MissingAt));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Creates an `Email` from `email` once it passes every check.
    ///
    /// `new` first trims whitespace from both ends of `email`. The checks then run on the trimmed
    /// address, in the order below, and `new` returns the variant of the first check that fails:
    ///
    /// | Check                                           | Fails with                         |
    /// |-------------------------------------------------|------------------------------------|
    /// | The address is at most 254 bytes long           | [`EmailError::TooLong`]            |
    /// | The address is not empty                        | [`EmailError::Empty`]              |
    /// | The address contains an `@`                     | [`EmailError::MissingAt`]          |
    /// | Text comes before the last `@`                  | [`EmailError::EmptyLocalPart`]     |
    /// | Text comes after the last `@`                   | [`EmailError::EmptyDomainPart`]    |
    /// | The address contains no whitespace              | [`EmailError::HasWhitespace`]      |
    /// | The domain has a dot, with text around each one | [`EmailError::DomainWithoutDot`]   |
    ///
    /// When every check passes, the returned `Email` stores the trimmed address.
    ///
    /// `new` splits the address at its last `@`. Therefore, `a@b@example.com` passes, with `a@b` as
    /// the part before the `@`.
    pub fn new(email: &str) -> Result<Self, EmailError> {
        let email = email.trim();

        if email.len() > 254 {
            return Err(EmailError::TooLong);
        }

        if email.is_empty() {
            return Err(EmailError::Empty);
        }

        let Some((local, domain)) = email.rsplit_once('@') else {
            return Err(EmailError::MissingAt);
        };

        if local.is_empty() {
            return Err(EmailError::EmptyLocalPart);
        }

        if domain.is_empty() {
            return Err(EmailError::EmptyDomainPart);
        }

        if email.contains(char::is_whitespace) {
            return Err(EmailError::HasWhitespace);
        }

        if !domain.contains('.') || domain.split('.').any(str::is_empty) {
            return Err(EmailError::DomainWithoutDot);
        }

        Ok(Self(email.to_owned()))
    }

    /// Returns the email address as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// An `EmailError` is the reason [`Email::new`] rejected an address.
///
/// Each variant identifies one check that `new` runs. The `Display` implementation returns a
/// message a person can read, such as "the email address has no @". `EmailError` also implements
/// [`std::error::Error`], meaning the `?` operator can convert it into a `Box<dyn Error>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailError {
    /// The address is empty once trimmed.
    Empty,
    /// The address is longer than 254 bytes once trimmed.
    TooLong,
    /// The address contains no `@`.
    MissingAt,
    /// The address has nothing before its last `@`.
    EmptyLocalPart,
    /// The address has nothing after its last `@`.
    EmptyDomainPart,
    /// The domain has no dot, or has an empty part between two dots or at either end, as in
    /// `example..com` or `example.com.`.
    DomainWithoutDot,
    /// The address contains whitespace between its first and last characters.
    HasWhitespace,
}

impl Display for EmailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::Empty => "the email address is empty",
            Self::TooLong => "the email address is longer than 254 bytes",
            Self::MissingAt => "the email address has no @",
            Self::EmptyLocalPart => "the email address is empty before the @",
            Self::EmptyDomainPart => "the email address is empty after the @",
            Self::DomainWithoutDot => "the domain of the email address has no dot `.`",
            Self::HasWhitespace => "the email address contains whitespace",
        };
        f.write_str(reason)
    }
}

impl std::error::Error for EmailError {}

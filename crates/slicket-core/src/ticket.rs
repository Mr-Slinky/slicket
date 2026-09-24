use crate::world::{Entity, Person};

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

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

    pub fn ticket_key(&self) -> TicketKey {
        self.ticket_key
    }

    pub fn status_key(&self) -> StatusKey {
        self.status_key
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn raised_by(&self) -> Entity<Person> {
        self.raised_by
    }

    pub fn created_at(&self) -> UnixEpochSeconds {
        self.created_at
    }

    pub fn set_status_key(&mut self, status_key: StatusKey) {
        self.status_key = status_key;
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TicketKey(pub(crate) u32);

impl TicketKey {
    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusKey(pub(crate) u16);

impl StatusKey {
    pub fn get(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(pub(crate) u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnixEpochSeconds(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClosedAt(pub UnixEpochSeconds);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description(pub String);

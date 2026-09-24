use crate::{Entity, Org};

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonCore {
    person_key: PersonKey,
    org_id: Entity<Org>,
    name: String,
}

impl PersonCore {
    pub fn new(person_key: PersonKey, org_id: Entity<Org>, name: String) -> Self {
        Self {
            person_key,
            org_id,
            name,
        }
    }

    pub fn person_key(&self) -> PersonKey {
        self.person_key
    }

    pub fn org_id(&self) -> Entity<Org> {
        self.org_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_org_id(&mut self, org_id: Entity<Org>) {
        self.org_id = org_id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrgCore {
    org_key: OrgKey,
    name: String,
}

impl OrgCore {
    pub fn new(org_key: OrgKey, name: String) -> Self {
        Self { org_key, name }
    }

    pub fn org_key(&self) -> OrgKey {
        self.org_key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrgKey(pub(crate) u32);

impl OrgKey {
    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonKey(pub(crate) u32);

impl PersonKey {
    pub fn get(self) -> u32 {
        self.0
    }
}

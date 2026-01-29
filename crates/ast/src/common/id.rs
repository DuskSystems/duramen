use alloc::string::String;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Id(String);

impl Id {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnreservedId(Id);

impl UnreservedId {
    #[must_use]
    pub const fn new(id: Id) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn as_id(&self) -> &Id {
        &self.0
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn into_id(self) -> Id {
        self.0
    }
}

impl AsRef<str> for UnreservedId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<UnreservedId> for Id {
    fn from(value: UnreservedId) -> Self {
        value.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AnyId(String);

impl AnyId {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for AnyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Id> for AnyId {
    fn from(value: Id) -> Self {
        Self(value.0)
    }
}

impl From<UnreservedId> for AnyId {
    fn from(value: UnreservedId) -> Self {
        Self(value.0.0)
    }
}

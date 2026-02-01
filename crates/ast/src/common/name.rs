use alloc::vec::Vec;

use super::id::Id;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Name {
    path: Vec<Id>,
    basename: Id,
}

impl Name {
    #[must_use]
    pub const fn qualified(path: Vec<Id>, basename: Id) -> Self {
        Self { path, basename }
    }

    #[must_use]
    pub const fn unqualified(basename: Id) -> Self {
        Self {
            path: Vec::new(),
            basename,
        }
    }

    #[must_use]
    pub fn path(&self) -> &[Id] {
        &self.path
    }

    #[must_use]
    pub const fn basename(&self) -> &Id {
        &self.basename
    }

    #[must_use]
    pub const fn is_unqualified(&self) -> bool {
        self.path.is_empty()
    }

    #[must_use]
    pub fn into_parts(self) -> (Vec<Id>, Id) {
        (self.path, self.basename)
    }
}

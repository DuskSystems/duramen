use alloc::sync::Arc;
use alloc::vec::Vec;

use super::id::Id;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Name {
    path: Arc<Vec<Id>>,
    basename: Id,
}

impl Name {
    #[must_use]
    pub fn new(path: Vec<Id>, basename: Id) -> Self {
        Self {
            path: Arc::new(path),
            basename,
        }
    }

    #[must_use]
    pub fn unqualified(basename: Id) -> Self {
        Self {
            path: Arc::new(Vec::new()),
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
    pub fn is_unqualified(&self) -> bool {
        self.path.is_empty()
    }

    #[must_use]
    pub fn into_parts(self) -> (Arc<Vec<Id>>, Id) {
        (self.path, self.basename)
    }
}

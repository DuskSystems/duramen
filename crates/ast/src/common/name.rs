use alloc::vec::Vec;
use core::fmt;

use crate::common::Identifier;

/// A qualified path like `Namespace::Type`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Name<'a> {
    path: Vec<Identifier<'a>>,
    basename: Identifier<'a>,
}

impl<'a> Name<'a> {
    /// Creates a name with explicit namespace path and basename.
    #[must_use]
    pub const fn new(path: Vec<Identifier<'a>>, basename: Identifier<'a>) -> Self {
        Self { path, basename }
    }

    /// Creates an unqualified name with a single segment.
    #[must_use]
    pub const fn unqualified(basename: Identifier<'a>) -> Self {
        Self {
            path: Vec::new(),
            basename,
        }
    }

    /// Returns the namespace path segments (excluding the basename).
    #[must_use]
    pub fn path(&self) -> &[Identifier<'a>] {
        &self.path
    }

    /// Returns the basename (final segment).
    #[must_use]
    pub const fn basename(&self) -> Identifier<'a> {
        self.basename
    }
}

impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.path {
            fmt::Display::fmt(segment, f)?;
            f.write_str("::")?;
        }

        fmt::Display::fmt(&self.basename, f)
    }
}

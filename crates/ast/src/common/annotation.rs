use alloc::collections::BTreeMap;
use alloc::string::String;

use super::id::AnyId;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Annotation {
    value: Option<String>,
}

impl Annotation {
    #[must_use]
    pub const fn with_value(value: String) -> Self {
        Self { value: Some(value) }
    }

    #[must_use]
    pub const fn without_value() -> Self {
        Self { value: None }
    }

    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    #[must_use]
    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[must_use]
    pub fn into_value(self) -> Option<String> {
        self.value
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Annotations(BTreeMap<AnyId, Annotation>);

impl Annotations {
    #[must_use]
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    #[must_use]
    pub const fn from_map(map: BTreeMap<AnyId, Annotation>) -> Self {
        Self(map)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn get(&self, key: &AnyId) -> Option<&Annotation> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AnyId, &Annotation)> {
        self.0.iter()
    }

    pub fn insert(&mut self, key: AnyId, annotation: Annotation) {
        self.0.insert(key, annotation);
    }

    #[must_use]
    pub fn into_map(self) -> BTreeMap<AnyId, Annotation> {
        self.0
    }
}

impl FromIterator<(AnyId, Annotation)> for Annotations {
    fn from_iter<T: IntoIterator<Item = (AnyId, Annotation)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Annotations {
    type IntoIter = alloc::collections::btree_map::IntoIter<AnyId, Annotation>;
    type Item = (AnyId, Annotation);

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Annotations {
    type IntoIter = alloc::collections::btree_map::Iter<'a, AnyId, Annotation>;
    type Item = (&'a AnyId, &'a Annotation);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

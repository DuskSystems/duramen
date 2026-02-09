#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub(crate) use rustc_hash::FxBuildHasher;

pub(crate) type IndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;
pub(crate) type IndexSet<T> = indexmap::IndexSet<T, FxBuildHasher>;
pub(crate) type IndexSet1<T> = mitsein::index_set1::IndexSet1<T, FxBuildHasher>;

mod error;
pub use error::Error;

mod common;
pub use common::*;

mod policy;
pub use policy::*;

mod schema;
pub use schema::*;

//! Inspired by:
//! <https://matklad.github.io/2025/12/28/parsing-advances.html>.

#[cfg(debug_assertions)]
use alloc::vec::Vec;
use core::fmt;

/// Tracks parser position to avoid infinite loops in debug builds.
pub struct Advance {
    #[cfg(debug_assertions)]
    positions: Vec<usize>,
}

impl Advance {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            positions: Vec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, position: usize) {
        #[cfg(debug_assertions)]
        self.positions.push(position);

        #[cfg(not(debug_assertions))]
        let _ = position;
    }

    #[inline]
    pub fn pop<T: fmt::Debug>(&mut self, position: usize, token: T) {
        #[cfg(debug_assertions)]
        #[expect(clippy::panic, reason = "Debug assertion")]
        {
            let Some(start) = self.positions.pop() else {
                panic!("`pop` called without prior `push`");
            };

            assert!(
                position > start,
                "parser did not advance: stuck at position {start} (token {token:?})"
            );
        }

        #[cfg(not(debug_assertions))]
        {
            let _ = position;
            drop(token);
        }
    }
}

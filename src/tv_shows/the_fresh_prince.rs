//! The Fresh Prince of Bel-Air generator (compatibility alias).
//!
//! This module previously duplicated [`crate::tv_shows::fresh_prince`] with
//! different (unbacked) locale keys. It now re-exports the canonical
//! implementation so both paths resolve real locale data.

pub use crate::tv_shows::fresh_prince::{character, quote};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports_are_usable() {
        assert!(!character().is_empty());
        assert!(!quote().is_empty());
    }
}

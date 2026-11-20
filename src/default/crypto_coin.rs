//! Crypto coin generator (compatibility re-export of [`crate::crypto`]).

/// Generate a random cryptocurrency coin name.
pub use crate::crypto::coin;

/// Generate a random cryptocurrency symbol (also called an acronym).
pub use crate::crypto::symbol;

/// Alias kept for backwards compatibility with the old `crypto_coin` API.
pub use crate::crypto::symbol as acronym;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports_are_usable() {
        assert!(!coin().is_empty());
        assert!(!acronym().is_empty());
        assert!(!symbol().is_empty());
    }
}

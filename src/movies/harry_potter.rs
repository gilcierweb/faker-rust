//! Harry Potter generator - generates random Harry Potter characters

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Harry Potter character
pub fn character() -> String {
    fetch_locale_with_context("harry_potter.characters", "en", Some("harry_potter"))
        .map(|v| sample_with_resolve(&v, Some("harry_potter")))
        .unwrap_or_else(|| sample(FALLBACK_HARRY_POTTER_CHARACTERS).to_string())
}

const FALLBACK_HARRY_POTTER_CHARACTERS: &[&str] = &[
    "Harry Potter",
    "Hermione Granger",
    "Ron Weasley",
    "Albus Dumbledore",
    "Severus Snape",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }
}

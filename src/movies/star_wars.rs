//! Star Wars generator - generates random Star Wars characters

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Star Wars pilot call sign
pub fn call_sign() -> String {
    fetch_locale_with_context("star_wars.call_sign", "en", Some("star_wars"))
        .map(|v| sample_with_resolve(&v, Some("star_wars")))
        .unwrap_or_else(|| "Red Leader".to_string())
}

/// Generate a random Star Wars character
pub fn character() -> String {
    fetch_locale_with_context("star_wars.characters", "en", Some("star_wars"))
        .map(|v| sample_with_resolve(&v, Some("star_wars")))
        .unwrap_or_else(|| sample(FALLBACK_STAR_WARS_CHARACTERS).to_string())
}

const FALLBACK_STAR_WARS_CHARACTERS: &[&str] = &[
    "Luke Skywalker",
    "Darth Vader",
    "Han Solo",
    "Princess Leia",
    "Obi-Wan Kenobi",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_call_sign() {
        for _ in 0..50 {
            let sign = call_sign();
            assert!(!sign.is_empty());
            assert!(!sign.contains("#{"), "unresolved placeholder: {sign}");
        }
    }
}

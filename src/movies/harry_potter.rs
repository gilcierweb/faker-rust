//! Harry Potter generator - generates random Harry Potter characters

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Harry Potter character
pub fn character() -> String {
    fetch_locale_with_context(
        "harry_potter.characters",
        &crate::config::FakerConfig::locale(),
        Some("harry_potter"),
    )
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

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `harry_potter.books`.
pub fn book() -> String {
    parity_pick("harry_potter.books")
}

/// Gem-compatible generator: `harry_potter.houses`.
pub fn house() -> String {
    parity_pick("harry_potter.houses")
}

/// Gem-compatible generator: `harry_potter.locations`.
pub fn location() -> String {
    parity_pick("harry_potter.locations")
}

/// Gem-compatible generator: `harry_potter.quotes`.
pub fn quote() -> String {
    parity_pick("harry_potter.quotes")
}

/// Gem-compatible generator: `harry_potter.spells`.
pub fn spell() -> String {
    parity_pick("harry_potter.spells")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }
}

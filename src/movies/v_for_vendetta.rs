//! V for Vendetta movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random V for Vendetta character
pub fn character() -> String {
    fetch_locale("v_for_vendetta.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random V for Vendetta quote
pub fn quote() -> String {
    fetch_locale("v_for_vendetta.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "V",
    "Evey Hammond",
    "Adam Sutler",
    "Inspector Finch",
    "Chancellor Adam Sutler",
    "Gordon Deitrich",
    "Dominic",
    "Mr. Creedy",
    "Valerie",
    "Ruth",
    "Lewis Prothero",
    "Delia Surridge",
    "Dr. Delia Surridge",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Remember, remember, the Fifth of November.",
    "Beneath this mask there is more than flesh. Beneath this mask there is an idea, and ideas are bulletproof.",
    "The only verdict is vengeance; a vendetta.",
    "Voilà! In view, a humble vaudevillian veteran, cast vicariously as both victim and villain.",
    "People should not be afraid of their governments. Governments should be afraid of their people.",
    "God is in the rain.",
    "A revolution without dancing is a revolution not worth having!",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

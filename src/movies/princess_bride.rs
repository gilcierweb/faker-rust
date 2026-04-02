//! The Princess Bride movie generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Princess Bride character
pub fn character() -> String {
    fetch_locale("princess_bride.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Princess Bride quote
pub fn quote() -> String {
    fetch_locale("princess_bride.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Westley",
    "Buttercup",
    "Inigo Montoya",
    "Fezzik",
    "Vizzini",
    "Prince Humperdinck",
    "Count Rugen",
    "Miracle Max",
    "Valerie",
    "The Grandfather",
    "The Grandson",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Hello. My name is Inigo Montoya. You killed my father. Prepare to die.",
    "Inconceivable!",
    "As you wish.",
    "Have fun storming the castle!",
    "There's a big difference between mostly dead and all dead.",
    "Anybody want a peanut?",
    "Death cannot stop true love. All it can do is delay it for a while.",
    "Life is pain, Highness. Anyone who says differently is selling something.",
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

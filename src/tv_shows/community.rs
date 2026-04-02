//! Community TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Community character
pub fn character() -> String {
    fetch_locale("community.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Community quote
pub fn quote() -> String {
    fetch_locale("community.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Jeff Winger",
    "Britta Perry",
    "Abed Nadir",
    "Shirley Bennett",
    "Annie Edison",
    "Troy Barnes",
    "Pierce Hawthorne",
    "Ben Chang",
    "Dean Craig Pelton",
    "Ian Duncan",
    "Buzz Hickey",
    "Frankie Dart",
    "Elroy Patashnik",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Six seasons and a movie!",
    "Streets ahead.",
    "Pop pop!",
    "Cool cool cool.",
    "I am a genius!",
    "Don't eat the crab dip! Yay-ye!",
    "Troy and Abed in the morning!",
    "Annie's pretty young. We try not to sexualize her.",
    "The dean is a genius!",
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

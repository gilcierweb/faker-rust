//! Friends TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Friends character
pub fn character() -> String {
    fetch_locale("friends.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Friends quote
pub fn quote() -> String {
    fetch_locale("friends.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Friends location
pub fn location() -> String {
    fetch_locale("friends.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Rachel Green",
    "Monica Geller",
    "Phoebe Buffay",
    "Joey Tribbiani",
    "Chandler Bing",
    "Ross Geller",
    "Gunther",
    "Janice",
    "Mike Hannigan",
    "David",
    "Tag Jones",
    "Paul Stevens",
];

const FALLBACK_QUOTES: &[&str] = &[
    "We were on a break!",
    "How you doin'?",
    "Unagi.",
    "I don't even have a 'pla'.",
    "Could this BE any more...?",
    "Oh. My. God.",
    "Pivot!",
    "I'm not great at the advice. Can I interest you in a sarcastic comment?",
    "Welcome to the real world. It sucks. You're gonna love it.",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Central Perk",
    "Monica's Apartment",
    "Joey and Chandler's Apartment",
    "Ross's Apartment",
    "Phoebe's Apartment",
    "Rachel's Apartment",
    "Coffee House",
    "New York City",
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

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }
}

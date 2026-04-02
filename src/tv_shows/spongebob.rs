//! SpongeBob SquarePants TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random SpongeBob character
pub fn character() -> String {
    fetch_locale("spongebob.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random SpongeBob quote
pub fn quote() -> String {
    fetch_locale("spongebob.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random SpongeBob location
pub fn location() -> String {
    fetch_locale("spongebob.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "SpongeBob SquarePants",
    "Patrick Star",
    "Squidward Tentacles",
    "Mr. Krabs",
    "Sandy Cheeks",
    "Plankton",
    "Karen",
    "Gary",
    "Mrs. Puff",
    "Pearl Krabs",
    "Larry the Lobster",
    "Mermaid Man",
    "Barnacle Boy",
    "Flying Dutchman",
    "Patchy the Pirate",
    "Potty the Parrot",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I'm ready! I'm ready!",
    "F is for friends who do stuff together!",
    "I'm a Goofy Goober, yeah!",
    "Aye-aye, captain!",
    "I'm ugly and I'm proud!",
    "Good morning, Krusty Krew!",
    "Hooooooooly mackerel!",
    "I'm ready, I'm ready, I'm ready!",
    "No, this is Patrick!",
    "Is mayonnaise an instrument?",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Bikini Bottom",
    "Krusty Krab",
    "Chum Bucket",
    "Jellyfish Fields",
    "Sandy's Treedome",
    "Conch Street",
    "Mrs. Puff's Boating School",
    "Rock Bottom",
    "Goo Lagoon",
    "The Flying Dutchman's Ship",
    "Atlantis",
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

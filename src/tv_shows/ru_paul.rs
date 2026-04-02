//! RuPaul's Drag Race TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random RuPaul quote
pub fn quote() -> String {
    fetch_locale("rupaul.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random RuPaul contestant
pub fn contestant() -> String {
    fetch_locale("rupaul.contestants", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CONTESTANTS).to_string())
}

/// Generate a random RuPaul challenge
pub fn challenge() -> String {
    fetch_locale("rupaul.challenges", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHALLENGES).to_string())
}

// Fallback data
const FALLBACK_QUOTES: &[&str] = &[
    "If you can't love yourself, how in the hell you gonna love somebody else?",
    "Shantay, you stay!",
    "Sashay away...",
    "Now, sashay away!",
    "Can I get an amen?",
    "Covergirl, put that bass in your walk!",
    "Don't f*** it up!",
    "Good luck, and don't f*** it up!",
];

const FALLBACK_CONTESTANTS: &[&str] = &[
    "Bianca Del Rio",
    "Adore Delano",
    "Alaska",
    "Jinkx Monsoon",
    "Bob the Drag Queen",
    "Trixie Mattel",
    "Katya",
    "Aquaria",
    "Sasha Velour",
    "Violet Chachki",
    "Raja",
    "Chad Michaels",
];

const FALLBACK_CHALLENGES: &[&str] = &[
    "Snatch Game",
    "Rusical",
    "Ball Challenge",
    "Makeover Challenge",
    "Roast",
    "Acting Challenge",
    "Sewing Challenge",
    "Lip Sync for Your Life",
    "Design Challenge",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_contestant() {
        assert!(!contestant().is_empty());
    }

    #[test]
    fn test_challenge() {
        assert!(!challenge().is_empty());
    }
}

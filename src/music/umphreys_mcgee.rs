//! Umphrey's McGee music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Umphrey's McGee song
pub fn song() -> String {
    fetch_locale("umphreys_mcgee.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Umphrey's McGee member
pub fn member() -> String {
    fetch_locale("umphreys_mcgee.members", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "In the Kitchen",
    "Der Bluten Kat",
    "Miss Tinkle's Overture",
    "Ringo",
    "Wife Soup",
    "Prowler",
    "Conduit",
    "1348",
    "Got Your Milk",
    "Zonkeys",
    "The Bottom Half",
];

const FALLBACK_MEMBERS: &[&str] = &[
    "Brendan Bayliss",
    "Jake Cinninger",
    "Kris Myers",
    "Andy Farag",
    "Ryan Stasik",
    "Joel Cummins",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song() {
        assert!(!song().is_empty());
    }

    #[test]
    fn test_member() {
        assert!(!member().is_empty());
    }
}

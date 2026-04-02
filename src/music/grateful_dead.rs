//! Grateful Dead music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Grateful Dead song
pub fn song() -> String {
    fetch_locale("grateful_dead.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Grateful Dead member
pub fn member() -> String {
    fetch_locale("grateful_dead.members", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "Truckin'",
    "Sugar Magnolia",
    "Uncle John's Band",
    "Casey Jones",
    "Touch of Grey",
    "Scarlet Begonias",
    "Fire on the Mountain",
    "Ripple",
    "Dark Star",
    "China Cat Sunflower",
    "Friend of the Devil",
    "Bertha",
    "Eyes of the World",
    "Terrapin Station",
];

const FALLBACK_MEMBERS: &[&str] = &[
    "Jerry Garcia",
    "Bob Weir",
    "Phil Lesh",
    "Bill Kreutzmann",
    "Mickey Hart",
    "Ron 'Pigpen' McKernan",
    "Brent Mydland",
    "Vince Welnick",
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

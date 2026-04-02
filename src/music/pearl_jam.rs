//! Pearl Jam music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Pearl Jam song
pub fn song() -> String {
    fetch_locale("pearl_jam.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Pearl Jam member
pub fn member() -> String {
    fetch_locale("pearl_jam.members", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "Alive",
    "Even Flow",
    "Jeremy",
    "Black",
    "Better Man",
    "Yellow Ledbetter",
    "Elderly Woman Behind the Counter in a Small Town",
    "Corduroy",
    "Daughter",
    "Rearviewmirror",
    "Do the Evolution",
    "Last Kiss",
    "Given to Fly",
];

const FALLBACK_MEMBERS: &[&str] = &[
    "Eddie Vedder",
    "Mike McCready",
    "Stone Gossard",
    "Jeff Ament",
    "Matt Cameron",
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

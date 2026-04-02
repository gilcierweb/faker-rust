//! Smashing Pumpkins music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Smashing Pumpkins song
pub fn song() -> String {
    fetch_locale("smashing_pumpkins.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Smashing Pumpkins album
pub fn album() -> String {
    fetch_locale("smashing_pumpkins.albums", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ALBUMS).to_string())
}

/// Generate a random Smashing Pumpkins member
pub fn member() -> String {
    fetch_locale("smashing_pumpkins.members", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "1979",
    "Bullet with Butterfly Wings",
    "Tonight, Tonight",
    "Disarm",
    "Cherub Rock",
    "Today",
    "Zero",
    "Mayonaise",
    "Today",
    "Ava Adore",
    "Perfect",
    "The Everlasting Gaze",
];

const FALLBACK_ALBUMS: &[&str] = &[
    "Siamese Dream",
    "Mellon Collie and the Infinite Sadness",
    "Adore",
    "Machina/The Machines of God",
    "Gish",
    "Zeitgeist",
    "Oceania",
];

const FALLBACK_MEMBERS: &[&str] = &[
    "Billy Corgan",
    "James Iha",
    "D'arcy Wretzky",
    "Jimmy Chamberlin",
    "Melissa Auf der Maur",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song() {
        assert!(!song().is_empty());
    }

    #[test]
    fn test_album() {
        assert!(!album().is_empty());
    }

    #[test]
    fn test_member() {
        assert!(!member().is_empty());
    }
}

//! Phish music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Phish song
pub fn song() -> String {
    fetch_locale("phish.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Phish member
pub fn member() -> String {
    fetch_locale("phish.members", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MEMBERS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "You Enjoy Myself",
    "Run Like an Antelope",
    "Tweezer",
    "Harry Hood",
    "Down with Disease",
    "Bathtub Gin",
    "Stash",
    "Fluffhead",
    "Chalk Dust Torture",
    "Free",
    "Sample in a Jar",
    "Bouncing Around the Room",
    "Julius",
];

const FALLBACK_MEMBERS: &[&str] = &[
    "Trey Anastasio",
    "Page McConnell",
    "Mike Gordon",
    "Jon Fishman",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `phish.albums`.
pub fn album() -> String {
    parity_pick("phish.albums")
}

/// Gem-compatible generator: `phish.musicians`.
pub fn musician() -> String {
    parity_pick("phish.musicians")
}

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

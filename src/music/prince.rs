//! Prince music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Prince song
pub fn song() -> String {
    fetch_locale("prince.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

/// Generate a random Prince album
pub fn album() -> String {
    fetch_locale("prince.albums", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ALBUMS).to_string())
}

/// Generate a random Prince side project/band
pub fn band() -> String {
    fetch_locale("prince.bands", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BANDS).to_string())
}

// Fallback data
const FALLBACK_SONGS: &[&str] = &[
    "Purple Rain",
    "When Doves Cry",
    "Kiss",
    "1999",
    "Little Red Corvette",
    "Raspberry Beret",
    "Let's Go Crazy",
    "U Got the Look",
    "Sign o' the Times",
    "Cream",
    "Diamonds and Pearls",
    "The Most Beautiful Girl in the World",
];

const FALLBACK_ALBUMS: &[&str] = &[
    "Purple Rain",
    "1999",
    "Sign o' the Times",
    "Parade",
    "Dirty Mind",
    "Controversy",
    "Around the World in a Day",
    "Lovesexy",
    "Batman",
    "Diamonds and Pearls",
];

const FALLBACK_BANDS: &[&str] = &[
    "The Revolution",
    "The New Power Generation",
    "3rdeyegirl",
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
    fn test_band() {
        assert!(!band().is_empty());
    }
}

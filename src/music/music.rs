//! Music generator - generates random music-related data

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random musical artist/band
pub fn band() -> String {
    fetch_locale_with_context("music.bands", "en", Some("music"))
        .map(|v| sample_with_resolve(&v, Some("music")))
        .unwrap_or_else(|| sample(FALLBACK_ARTISTS).to_string())
}

/// Generate a random album title
pub fn album() -> String {
    fetch_locale_with_context("music.albums", "en", Some("music"))
        .map(|v| sample_with_resolve(&v, Some("music")))
        .unwrap_or_else(|| sample(FALLBACK_ALBUMS).to_string())
}

/// Generate a random music genre
pub fn genre() -> String {
    fetch_locale_with_context("music.genres", "en", Some("music"))
        .map(|v| sample_with_resolve(&v, Some("music")))
        .unwrap_or_else(|| sample(FALLBACK_GENRES).to_string())
}

/// Generate a random musical instrument
pub fn instrument() -> String {
    fetch_locale_with_context("music.instruments", "en", Some("music"))
        .map(|v| sample_with_resolve(&v, Some("music")))
        .unwrap_or_else(|| sample(FALLBACK_INSTRUMENTS).to_string())
}

// Fallback data
const FALLBACK_ARTISTS: &[&str] = &[
    "The Beatles",
    "Led Zeppelin",
    "Pink Floyd",
    "Queen",
    "AC/DC",
];

const FALLBACK_ALBUMS: &[&str] = &[
    "Abbey Road",
    "The Dark Side of the Moon",
    "Nevermind",
];

const FALLBACK_GENRES: &[&str] = &[
    "Rock",
    "Pop",
    "Jazz",
    "Classical",
];

const FALLBACK_INSTRUMENTS: &[&str] = &[
    "Guitar",
    "Piano",
    "Drums",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_band() {
        assert!(!band().is_empty());
    }

    #[test]
    fn test_album() {
        assert!(!album().is_empty());
    }
}

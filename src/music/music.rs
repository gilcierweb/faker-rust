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

const FALLBACK_ALBUMS: &[&str] = &["Abbey Road", "The Dark Side of the Moon", "Nevermind"];

const FALLBACK_GENRES: &[&str] = &["Rock", "Pop", "Jazz", "Classical"];

const FALLBACK_INSTRUMENTS: &[&str] = &["Guitar", "Piano", "Drums"];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `music.mambo_no_5`.
pub fn mambo_no_5() -> String {
    parity_pick("music.mambo_no_5")
}
// --- gem parity generators (algorithmic) ---
const NOTE_LETTERS: &[&str] = &["C", "D", "E", "F", "G", "A", "B"];
const ACCIDENTAL_SIGNS: &[&str] = &["b", "#", ""];
const CHORD_TYPES: &[&str] = &[
    "", "maj", "6", "maj7", "m", "m7", "-7", "7", "dom7", "dim", "dim7", "m7b5",
];

/// Gem-compatible generator: musical key using letter notation.
pub fn key() -> String {
    format!(
        "{}{}",
        crate::base::sample(NOTE_LETTERS),
        crate::base::sample(ACCIDENTAL_SIGNS)
    )
}
/// Gem-compatible generator: chord built from a musical key.
pub fn chord() -> String {
    format!("{}{}", key(), crate::base::sample(CHORD_TYPES))
}

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

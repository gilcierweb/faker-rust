//! Music generator - generates random music data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random music instrument
pub fn instrument() -> String {
    fetch_locale("music.instruments", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Guitar".to_string())
}

/// Generate a random band name
pub fn band() -> String {
    fetch_locale("music.bands", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Beatles".to_string())
}

/// Generate a random album title
pub fn album() -> String {
    fetch_locale("music.albums", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Thriller".to_string())
}

/// Generate a random music genre
pub fn genre() -> String {
    fetch_locale("music.genres", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Rock".to_string())
}

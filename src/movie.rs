//! Movie generator - generates random movie titles

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random movie title
pub fn title() -> String {
    fetch_locale("movie.title", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Godfather".to_string())
}

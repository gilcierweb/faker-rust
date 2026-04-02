//! Movie generator - generates random movie titles and quotes

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random movie title
pub fn title() -> String {
    fetch_locale_with_context("movie.title", "en", Some("movie"))
        .map(|v| sample_with_resolve(&v, Some("movie")))
        .unwrap_or_else(|| sample(FALLBACK_TITLES).to_string())
}

/// Generate a random movie quote
pub fn quote() -> String {
    fetch_locale_with_context("movie.quote", "en", Some("movie"))
        .map(|v| sample_with_resolve(&v, Some("movie")))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_TITLES: &[&str] = &[
    "The Godfather",
    "The Shawshank Redemption",
    "The Dark Knight",
    "Pulp Fiction",
    "Star Wars",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Frankly, my dear, I don’t give a damn.",
    "I'm going to make him an offer he can't refuse.",
    "May the Force be with you.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        assert!(!title().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

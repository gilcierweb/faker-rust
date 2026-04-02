//! Theater generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random theater name
pub fn name() -> String {
    fetch_locale("theater.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random play/musical title
pub fn play() -> String {
    fetch_locale("theater.plays", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLAYS).to_string())
}

/// Generate a random theater genre
pub fn genre() -> String {
    fetch_locale("theater.genres", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GENRES).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "The Globe Theatre", "Broadway Theatre", "West End Theatre",
    "Royal Shakespeare Theatre", "Lyceum Theatre", "Apollo Theatre",
    "Victoria Palace Theatre", "The Old Vic", "National Theatre",
    "Royal Opera House", "Carnegie Hall", "Sydney Opera House",
];

const FALLBACK_PLAYS: &[&str] = &[
    "Hamlet", "Romeo and Juliet", "Macbeth", "The Phantom of the Opera",
    "Les Misérables", "The Lion King", "Wicked", "Hamilton", "Cats",
    "The Crucible", "A Streetcar Named Desire", "Death of a Salesman",
];

const FALLBACK_GENRES: &[&str] = &[
    "Tragedy", "Comedy", "Drama", "Musical", "Opera", "Ballet",
    "Farce", "Satire", "Melodrama", "Historical", "Contemporary",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_play() {
        assert!(!play().is_empty());
    }

    #[test]
    fn test_genre() {
        assert!(!genre().is_empty());
    }
}

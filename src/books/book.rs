//! Book generator - generates random book titles, authors, genres, and publishers

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random book title
pub fn title() -> String {
    fetch_locale_with_context("book.title", "en", Some("book"))
        .map(|v| sample_with_resolve(&v, Some("book")))
        .unwrap_or_else(|| sample(FALLBACK_TITLES).to_string())
}

/// Generate a random book author
pub fn author() -> String {
    fetch_locale_with_context("book.author", "en", Some("book"))
        .map(|v| sample_with_resolve(&v, Some("book")))
        .unwrap_or_else(|| crate::name::name())
}

/// Generate a random book genre
pub fn genre() -> String {
    fetch_locale_with_context("book.genre", "en", Some("book"))
        .map(|v| sample_with_resolve(&v, Some("book")))
        .unwrap_or_else(|| sample(FALLBACK_GENRES).to_string())
}

/// Generate a random book publisher
pub fn publisher() -> String {
    fetch_locale_with_context("book.publisher", "en", Some("book"))
        .map(|v| sample_with_resolve(&v, Some("book")))
        .unwrap_or_else(|| sample(FALLBACK_PUBLISHERS).to_string())
}

// Fallback data
const FALLBACK_TITLES: &[&str] = &[
    "The Great Gatsby",
    "To Kill a Mockingbird",
    "1984",
    "The Catcher in the Rye",
    "The Hobbit",
];

const FALLBACK_GENRES: &[&str] = &["Classic", "Fantasy", "Science Fiction", "Horror", "Mystery"];

const FALLBACK_PUBLISHERS: &[&str] = &[
    "Penguin Books",
    "HarperCollins",
    "Simon & Schuster",
    "Hachette Book Group",
    "Macmillan Publishers",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        assert!(!title().is_empty());
    }

    #[test]
    fn test_author() {
        assert!(!author().is_empty());
    }

    #[test]
    fn test_genre() {
        assert!(!genre().is_empty());
    }

    #[test]
    fn test_publisher() {
        assert!(!publisher().is_empty());
    }
}

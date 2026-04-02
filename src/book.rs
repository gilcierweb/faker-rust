//! Book generator - generates random book titles and authors

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random book title
pub fn title() -> String {
    fetch_locale("book.title", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Great Gatsby".to_string())
}

/// Generate a random book author
pub fn author() -> String {
    fetch_locale("book.author", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "J.K. Rowling".to_string())
}

/// Generate a random genre
pub fn genre() -> String {
    sample(&[
        "Fiction".to_string(),
        "Non-Fiction".to_string(),
        "Science Fiction".to_string(),
        "Mystery".to_string(),
        "Thriller".to_string(),
        "Romance".to_string(),
        "Horror".to_string(),
        "Biography".to_string(),
        "History".to_string(),
    ])
}

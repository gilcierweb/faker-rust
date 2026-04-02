//! Avatar generator - generates random avatar characters, quotes, and image URLs

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Avatar character (from the movie)
pub fn character() -> String {
    fetch_locale_with_context("avatar.characters", "en", Some("avatar"))
        .map(|v| sample_with_resolve(&v, Some("avatar")))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Avatar quote
pub fn quote() -> String {
    fetch_locale_with_context("avatar.quotes", "en", Some("avatar"))
        .map(|v| sample_with_resolve(&v, Some("avatar")))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random avatar image URL using RoboHash
pub fn image(slug: Option<&str>, size: Option<&str>, format: Option<&str>) -> String {
    let slug = slug.unwrap_or("avatar");
    let size = size.unwrap_or("300x300");
    let format = format.unwrap_or("png");
    format!("https://robohash.org/{}.{}?size={}", slug, format, size)
}

const FALLBACK_CHARACTERS: &[&str] = &["Neytiri", "Jake Sully", "Miles Quaritch"];
const FALLBACK_QUOTES: &[&str] = &["I see you.", "This is our land!"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_image() {
        let url = image(None, None, None);
        assert!(url.contains("robohash.org"));
    }
}

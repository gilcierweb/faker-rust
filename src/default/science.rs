//! Science generator - generates random chemical elements, symbols, and scientists

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random chemical element (e.g. "Hydrogen")
pub fn element() -> String {
    fetch_locale_with_context("science.element", "en", Some("science"))
        .map(|v| sample_with_resolve(&v, Some("science")))
        .unwrap_or_else(|| sample(FALLBACK_ELEMENTS).to_string())
}

/// Generate a random chemical element symbol (e.g. "H")
pub fn element_symbol() -> String {
    fetch_locale_with_context("science.element_symbol", "en", Some("science"))
        .map(|v| sample_with_resolve(&v, Some("science")))
        .unwrap_or_else(|| sample(FALLBACK_SYMBOLS).to_string())
}

/// Generate a random scientist name (e.g. "Albert Einstein")
pub fn scientist() -> String {
    fetch_locale_with_context("science.scientist", "en", Some("science"))
        .map(|v| sample_with_resolve(&v, Some("science")))
        .unwrap_or_else(|| sample(FALLBACK_SCIENTISTS).to_string())
}

// Fallback data
const FALLBACK_ELEMENTS: &[&str] = &["Hydrogen", "Helium", "Lithium", "Beryllium", "Boron"];
const FALLBACK_SYMBOLS: &[&str] = &["H", "He", "Li", "Be", "B"];
const FALLBACK_SCIENTISTS: &[&str] = &[
    "Isaac Newton",
    "Albert Einstein",
    "Marie Curie",
    "Charles Darwin",
    "Nikola Tesla",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() {
        assert!(!element().is_empty());
    }

    #[test]
    fn test_scientist() {
        assert!(!scientist().is_empty());
    }
}

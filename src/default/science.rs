//! Science generator - generates random chemical elements, symbols, and scientists

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random chemical element (e.g. "Hydrogen")
pub fn element() -> String {
    fetch_locale_with_context(
        "science.element",
        &crate::config::FakerConfig::locale(),
        Some("science"),
    )
    .map(|v| sample_with_resolve(&v, Some("science")))
    .unwrap_or_else(|| sample(FALLBACK_ELEMENTS).to_string())
}

/// Generate a random chemical element symbol (e.g. "H")
pub fn element_symbol() -> String {
    fetch_locale_with_context(
        "science.element_symbol",
        &crate::config::FakerConfig::locale(),
        Some("science"),
    )
    .map(|v| sample_with_resolve(&v, Some("science")))
    .unwrap_or_else(|| sample(FALLBACK_SYMBOLS).to_string())
}

/// Generate a random scientist name (e.g. "Albert Einstein")
pub fn scientist() -> String {
    fetch_locale_with_context(
        "science.scientist",
        &crate::config::FakerConfig::locale(),
        Some("science"),
    )
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
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: state of matter of a chemical element.
pub fn element_state() -> String {
    crate::locale::fetch_locale(
        "science.element_state",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::sample(&v))
    .unwrap_or_default()
}
/// Gem-compatible generator: element subcategory.
pub fn element_subcategory() -> String {
    crate::locale::fetch_locale(
        "science.element_subcategory",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::sample(&v))
    .unwrap_or_default()
}
/// Gem-compatible generator: scientific modifier word.
pub fn modifier() -> String {
    crate::locale::fetch_locale("science.modifier", &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}
/// Gem-compatible generator: scientific tool, optionally without a modifier.
pub fn tool(simple: bool) -> String {
    let base = crate::locale::fetch_locale("science.tool", &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default();
    if simple {
        return base;
    }
    format!("{} {}", modifier(), base)
}
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: science branch name, optionally filtered.
pub fn science(branches: &[&str]) -> String {
    let all =
        crate::locale::fetch_locale_keys("science.branch", &crate::config::FakerConfig::locale());
    let selected: Vec<&String> = if branches.is_empty() {
        all.iter().collect()
    } else {
        all.iter()
            .filter(|k| branches.contains(&k.as_str()))
            .collect()
    };
    let source = crate::base::sample(&selected);
    crate::locale::fetch_locale(
        &format!("science.branch.{source}"),
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::sample(&v))
    .unwrap_or_default()
}

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

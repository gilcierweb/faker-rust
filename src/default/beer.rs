//! Beer generator - generates random beer names, styles, hops, malts, and yeasts

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random beer name
pub fn name() -> String {
    fetch_locale_with_context("beer.name", "en", Some("beer"))
        .map(|v| sample_with_resolve(&v, Some("beer")))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random beer style
pub fn style() -> String {
    fetch_locale_with_context("beer.style", "en", Some("beer"))
        .map(|v| sample_with_resolve(&v, Some("beer")))
        .unwrap_or_else(|| sample(FALLBACK_STYLES).to_string())
}

/// Generate a random beer hop
pub fn hop() -> String {
    fetch_locale_with_context("beer.hop", "en", Some("beer"))
        .map(|v| sample_with_resolve(&v, Some("beer")))
        .unwrap_or_else(|| sample(FALLBACK_HOPS).to_string())
}

/// Generate a random beer malt
pub fn malt() -> String {
    fetch_locale_with_context("beer.malt", "en", Some("beer"))
        .map(|v| sample_with_resolve(&v, Some("beer")))
        .unwrap_or_else(|| sample(FALLBACK_MALTS).to_string())
}

/// Generate a random beer yeast
pub fn yeast() -> String {
    fetch_locale_with_context("beer.yeast", "en", Some("beer"))
        .map(|v| sample_with_resolve(&v, Some("beer")))
        .unwrap_or_else(|| sample(FALLBACK_YEASTS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &["Guinness", "Heineken", "Budweiser", "Corona", "Stella Artois"];
const FALLBACK_STYLES: &[&str] = &["Lager", "IPA", "Pilsner", "Stout", "Porter"];
const FALLBACK_HOPS: &[&str] = &["Cascade", "Centennial", "Chinook", "Citra", "Columbus"];
const FALLBACK_MALTS: &[&str] = &["Pale", "Caramel", "Chocolate", "Roasted Barley", "Munich"];
const FALLBACK_YEASTS: &[&str] = &["Top fermenting", "Bottom fermenting", "Wild", "Lambic"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_style() {
        assert!(!style().is_empty());
    }
}

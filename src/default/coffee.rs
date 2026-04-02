//! Coffee generator - generates random coffee blends, origins, varieties, and notes

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random coffee blend name
pub fn blend_name() -> String {
    fetch_locale_with_context("coffee.blend_name", "en", Some("coffee"))
        .map(|v| sample_with_resolve(&v, Some("coffee")))
        .unwrap_or_else(|| sample(FALLBACK_BLEND_NAMES).to_string())
}

/// Generate a random coffee country of origin
pub fn country() -> String {
    fetch_locale_with_context("coffee.country", "en", Some("coffee"))
        .map(|v| sample_with_resolve(&v, Some("coffee")))
        .unwrap_or_else(|| sample(FALLBACK_COUNTRIES).to_string())
}

/// Generate a random coffee variety
pub fn variety() -> String {
    fetch_locale_with_context("coffee.variety", "en", Some("coffee"))
        .map(|v| sample_with_resolve(&v, Some("coffee")))
        .unwrap_or_else(|| sample(FALLBACK_VARIETIES).to_string())
}

/// Generate a random coffee note
pub fn notes() -> String {
    fetch_locale_with_context("coffee.notes", "en", Some("coffee"))
        .map(|v| sample_with_resolve(&v, Some("coffee")))
        .unwrap_or_else(|| sample(FALLBACK_NOTES).to_string())
}

// Fallback data
const FALLBACK_BLEND_NAMES: &[&str] = &["Summer Solstice", "Winter Wonderland", "Midnight Brew"];
const FALLBACK_COUNTRIES: &[&str] = &["Ethiopia", "Colombia", "Vietnam", "Brazil", "Indonesia"];
const FALLBACK_VARIETIES: &[&str] = &["Arabica", "Robusta", "Caturra", "Bourbon", "Typica"];
const FALLBACK_NOTES: &[&str] = &["fruity", "chocolatey", "nutty", "caramel", "acidic"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_name() {
        assert!(!blend_name().is_empty());
    }

    #[test]
    fn test_country() {
        assert!(!country().is_empty());
    }
}

//! Cannabis generator - generates random cannabis-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random cannabis strain
pub fn strain() -> String {
    fetch_locale("cannabis.strains", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "OG Kush".to_string())
}

/// Generate a random cannabinoid
pub fn cannabinoid() -> String {
    fetch_locale(
        "cannabis.cannabinoids",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "THC".to_string())
}

/// Generate a random terpene
pub fn terpene() -> String {
    fetch_locale("cannabis.terpenes", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Myrcene".to_string())
}

/// Generate a random medical use
pub fn medical_use() -> String {
    fetch_locale(
        "cannabis.medical_uses",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Pain relief".to_string())
}

/// Generate a random health benefit
pub fn health_benefit() -> String {
    fetch_locale(
        "cannabis.health_benefits",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Stress relief".to_string())
}

/// Generate a random category
pub fn category() -> String {
    fetch_locale("cannabis.categories", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Flower".to_string())
}

/// Generate a random type (Sativa, Indica, Hybrid)
pub fn type_name() -> String {
    fetch_locale("cannabis.types", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hybrid".to_string())
}

/// Generate a random cannabis brand
pub fn brand() -> String {
    fetch_locale("cannabis.brands", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Cookies".to_string())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `cannabis.buzzwords`.
pub fn buzzword() -> String {
    parity_pick("cannabis.buzzwords")
}

/// Gem-compatible generator: `cannabis.cannabinoid_abbreviations`.
pub fn cannabinoid_abbreviation() -> String {
    parity_pick("cannabis.cannabinoid_abbreviations")
}

/// Gem-compatible generator: `cannabis.types`.
pub fn r#type() -> String {
    parity_pick("cannabis.types")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strain() {
        let s = strain();
        assert!(!s.is_empty());
    }
}

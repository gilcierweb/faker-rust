//! Cannabis generator - generates random cannabis-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random cannabis strain
pub fn strain() -> String {
    fetch_locale("cannabis.strains", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "OG Kush".to_string())
}

/// Generate a random cannabinoid
pub fn cannabinoid() -> String {
    fetch_locale("cannabis.cannabinoids", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "THC".to_string())
}

/// Generate a random terpene
pub fn terpene() -> String {
    fetch_locale("cannabis.terpenes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Myrcene".to_string())
}

/// Generate a random medical use
pub fn medical_use() -> String {
    fetch_locale("cannabis.medical_uses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Pain relief".to_string())
}

/// Generate a random health benefit
pub fn health_benefit() -> String {
    fetch_locale("cannabis.health_benefits", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Stress relief".to_string())
}

/// Generate a random category
pub fn category() -> String {
    fetch_locale("cannabis.categories", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Flower".to_string())
}

/// Generate a random type (Sativa, Indica, Hybrid)
pub fn type_name() -> String {
    fetch_locale("cannabis.types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hybrid".to_string())
}

/// Generate a random cannabis brand
pub fn brand() -> String {
    fetch_locale("cannabis.brands", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Cookies".to_string())
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

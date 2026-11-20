//! Nation generator - generates names of nations, nationalities, and languages

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random nation nationality
pub fn nationality() -> String {
    fetch_locale("nation.nationality", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Brazilian".to_string())
}

/// Generate a random nation language
pub fn language() -> String {
    fetch_locale("nation.language", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Portuguese".to_string())
}

/// Generate a random nation capital city
pub fn capital_city() -> String {
    fetch_locale("nation.capital_city", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Brasilia".to_string())
}

/// Generate a random nation flag emoji
pub fn flag() -> String {
    fetch_locale("nation.flag", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "🇧🇷".to_string())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `team.sport`.
pub fn national_sport() -> String {
    parity_pick("team.sport")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nation_generation() {
        assert!(!nationality().is_empty());
        assert!(!language().is_empty());
        assert!(!capital_city().is_empty());
        assert!(!flag().is_empty());
    }
}

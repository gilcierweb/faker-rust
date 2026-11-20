//! Compass generator - generates random compass-related data

use crate::base::sample;
use crate::locale::{fetch_locale, sample_with_resolve};

/// Generate a random compass direction (north, northeast, etc.)
pub fn direction() -> String {
    fetch_locale("compass.direction", "en")
        .map(|v| sample_with_resolve(&v, Some("compass")))
        .unwrap_or_else(|| "north".to_string())
}

/// Generate a random compass abbreviation (N, NE, NNE, etc.)
pub fn abbreviation() -> String {
    fetch_locale("compass.abbreviation", "en")
        .map(|v| sample_with_resolve(&v, Some("compass")))
        .unwrap_or_else(|| "N".to_string())
}

/// Generate a random compass azimuth (0, 45, 22.5, etc.)
pub fn azimuth() -> String {
    fetch_locale("compass.azimuth", "en")
        .map(|v| sample_with_resolve(&v, Some("compass")))
        .unwrap_or_else(|| "0".to_string())
}

/// Generate a random cardinal direction
pub fn cardinal() -> String {
    fetch_locale("compass.cardinal.word", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "north".to_string())
}

/// Generate a random ordinal direction
pub fn ordinal() -> String {
    fetch_locale("compass.ordinal.word", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "northeast".to_string())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `compass.cardinal.abbreviation`.
pub fn cardinal_abbreviation() -> String {
    parity_pick("compass.cardinal.abbreviation")
}

/// Gem-compatible generator: `compass.cardinal.azimuth`.
pub fn cardinal_azimuth() -> String {
    parity_pick("compass.cardinal.azimuth")
}

/// Gem-compatible generator: `compass.half-wind.word`.
pub fn half_wind() -> String {
    parity_pick("compass.half-wind.word")
}

/// Gem-compatible generator: `compass.half-wind.abbreviation`.
pub fn half_wind_abbreviation() -> String {
    parity_pick("compass.half-wind.abbreviation")
}

/// Gem-compatible generator: `compass.half-wind.azimuth`.
pub fn half_wind_azimuth() -> String {
    parity_pick("compass.half-wind.azimuth")
}

/// Gem-compatible generator: `compass.ordinal.abbreviation`.
pub fn ordinal_abbreviation() -> String {
    parity_pick("compass.ordinal.abbreviation")
}

/// Gem-compatible generator: `compass.ordinal.azimuth`.
pub fn ordinal_azimuth() -> String {
    parity_pick("compass.ordinal.azimuth")
}

/// Gem-compatible generator: `compass.quarter-wind.word`.
pub fn quarter_wind() -> String {
    parity_pick("compass.quarter-wind.word")
}

/// Gem-compatible generator: `compass.quarter-wind.abbreviation`.
pub fn quarter_wind_abbreviation() -> String {
    parity_pick("compass.quarter-wind.abbreviation")
}

/// Gem-compatible generator: `compass.quarter-wind.azimuth`.
pub fn quarter_wind_azimuth() -> String {
    parity_pick("compass.quarter-wind.azimuth")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        let d = direction();
        assert!(!d.is_empty());
    }
}

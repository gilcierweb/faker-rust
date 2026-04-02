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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        let d = direction();
        assert!(!d.is_empty());
    }
}

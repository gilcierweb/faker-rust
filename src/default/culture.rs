//! Culture generator - generates names of planets, ships, and civilizations from the Culture series

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random planet from the Culture series
pub fn planet() -> String {
    fetch_locale("culture_series.planets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Vavatch".to_string())
}

/// Generate a random ship from the Culture series
pub fn ship() -> String {
    fetch_locale("culture_series.ships", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Of Course I Still Love You".to_string())
}

/// Generate a random civilization from the Culture series
pub fn civilization() -> String {
    fetch_locale("culture_series.civilizations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Culture".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_culture_generation() {
        assert!(!planet().is_empty());
        assert!(!ship().is_empty());
        assert!(!civilization().is_empty());
    }
}

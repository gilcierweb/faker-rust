//! Parks and Recreation generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Parks and Recreation character
pub fn character() -> String {
    fetch_locale("parks_and_rec.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Leslie Knope".to_string())
}

/// Generate a random Parks and Recreation city
pub fn city() -> String {
    fetch_locale("parks_and_rec.cities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Pawnee".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parks_and_rec_generation() {
        assert!(!character().is_empty());
        assert!(!city().is_empty());
    }
}

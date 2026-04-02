//! Bird generator - generates bird names, anatomy, and descriptors

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random bird anatomy part
pub fn anatomy() -> String {
    fetch_locale("creature.bird.anatomy", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "beak".to_string())
}

/// Generate a random bird color
pub fn colors() -> String {
    fetch_locale("creature.bird.colors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "black".to_string())
}

/// Generate a random bird adjective
pub fn adjectives() -> String {
    fetch_locale("creature.bird.adjectives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "common".to_string())
}

/// Generate a random bird common family name
pub fn common_family_name() -> String {
    fetch_locale("creature.bird.common_family_name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hawk".to_string())
}

/// Generate a plausible bird common name
pub fn common_name() -> String {
    fetch_locale("creature.bird.plausible_common_names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Common Black Hawk".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bird_generation() {
        assert!(!anatomy().is_empty());
        assert!(!colors().is_empty());
        assert!(!adjectives().is_empty());
        assert!(!common_family_name().is_empty());
        assert!(!common_name().is_empty());
    }
}

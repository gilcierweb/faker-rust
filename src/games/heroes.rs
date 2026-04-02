//! Heroes of Might and Magic generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random hero name from Heroes of Might and Magic
pub fn name() -> String {
    fetch_locale("heroes.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Catherine Ironfist".to_string())
}

/// Generate a random hero specialty
pub fn specialty() -> String {
    fetch_locale("heroes.specialties", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Archery".to_string())
}

/// Generate a random hero class (using 'klass' to avoid keyword conflict)
pub fn klass() -> String {
    fetch_locale("heroes.klasses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Knight".to_string())
}

/// Generate a random artifact name
pub fn artifact() -> String {
    fetch_locale("heroes.artifacts", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Orb of Inhibition".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heroes_generation() {
        assert!(!name().is_empty());
        assert!(!specialty().is_empty());
        assert!(!klass().is_empty());
        assert!(!artifact().is_empty());
    }
}

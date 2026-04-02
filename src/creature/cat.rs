//! Cat generator - generates cat names, breeds, and registries

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random cat name
pub fn name() -> String {
    fetch_locale("creature.cat.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Felix".to_string())
}

/// Generate a random cat breed
pub fn breed() -> String {
    fetch_locale("creature.cat.breed", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Abyssinian".to_string())
}

/// Generate a random cat registry
pub fn registry() -> String {
    fetch_locale("creature.cat.registry", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "TICA".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_breed() {
        assert!(!breed().is_empty());
    }

    #[test]
    fn test_registry() {
        assert!(!registry().is_empty());
    }
}

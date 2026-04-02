//! The Venture Bros generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Venture Bros character
pub fn character() -> String {
    fetch_locale("venture_bros.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hank Venture".to_string())
}

/// Generate a random Venture Bros organization
pub fn organization() -> String {
    fetch_locale("venture_bros.organization", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Guild of Calamity Intent".to_string())
}

/// Generate a random Venture Bros vehicle
pub fn vehicle() -> String {
    fetch_locale("venture_bros.vehicle", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The X-1".to_string())
}

/// Generate a random Venture Bros quote
pub fn quote() -> String {
    fetch_locale("venture_bros.quote", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Go Team Venture!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_venture_bros_generation() {
        assert!(!character().is_empty());
        assert!(!organization().is_empty());
        assert!(!vehicle().is_empty());
        assert!(!quote().is_empty());
    }
}

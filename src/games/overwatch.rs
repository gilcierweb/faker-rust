//! Overwatch generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Overwatch hero
pub fn hero() -> String {
    fetch_locale("overwatch.heroes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Tracer".to_string())
}

/// Generate a random Overwatch location
pub fn location() -> String {
    fetch_locale("overwatch.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hanamura".to_string())
}

/// Generate a random Overwatch quote
pub fn quote() -> String {
    fetch_locale("overwatch.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Cheers, love! The cavalry's here!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overwatch_generation() {
        assert!(!hero().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
    }
}

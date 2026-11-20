//! Stranger Things generator

use crate::base::sample;
use crate::locale::fetch_locale_with_context;

// The bundled data lives in `stranger_thing.yml` (singular file stem) while
// the locale key is plural, so look it up with an explicit context.
const CONTEXT: &str = "stranger_thing";

/// Generate a random Stranger Things character
pub fn character() -> String {
    fetch_locale_with_context(
        "stranger_things.character",
        &crate::config::FakerConfig::locale(),
        Some(CONTEXT),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Eleven".to_string())
}

/// Generate a random Stranger Things quote
pub fn quote() -> String {
    fetch_locale_with_context(
        "stranger_things.quote",
        &crate::config::FakerConfig::locale(),
        Some(CONTEXT),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Friends don't lie.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stranger_things_generation() {
        assert!(!character().is_empty());
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_stranger_things_resolve_locale_data() {
        // Data-backed values must vary; a single fallback would not.
        let chars: std::collections::HashSet<_> = (0..50).map(|_| character()).collect();
        let quotes: std::collections::HashSet<_> = (0..50).map(|_| quote()).collect();
        assert!(chars.len() > 1, "characters did not come from locale data");
        assert!(quotes.len() > 1, "quotes did not come from locale data");
    }
}

//! The Fresh Prince of Bel-Air generator

use crate::base::sample;
use crate::locale::fetch_locale_with_context;

// The bundled data lives in `fresh_prince_of_bel_air.yml` while its locale
// key is `the_fresh_prince_of_bel_air`, so the file stem is passed as context.
const CONTEXT: &str = "fresh_prince_of_bel_air";
const KEY: &str = "the_fresh_prince_of_bel_air";

/// Generate a random Fresh Prince character
pub fn character() -> String {
    fetch_locale_with_context(
        &format!("{KEY}.characters"),
        &crate::config::FakerConfig::locale(),
        Some(CONTEXT),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Will Smith".to_string())
}

/// Generate a random Fresh Prince actor
pub fn actor() -> String {
    fetch_locale_with_context(
        &format!("{KEY}.actors"),
        &crate::config::FakerConfig::locale(),
        Some(CONTEXT),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Will Smith".to_string())
}

/// Generate a random Fresh Prince quote
pub fn quote() -> String {
    fetch_locale_with_context(
        &format!("{KEY}.quotes"),
        &crate::config::FakerConfig::locale(),
        Some(CONTEXT),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| {
        "Now this is a story all about how my life got flipped-turned upside down.".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_prince_generation() {
        assert!(!character().is_empty());
        assert!(!actor().is_empty());
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_fresh_prince_resolve_locale_data() {
        let chars: std::collections::HashSet<_> = (0..50).map(|_| character()).collect();
        assert!(chars.len() > 1, "characters did not come from locale data");
    }
}

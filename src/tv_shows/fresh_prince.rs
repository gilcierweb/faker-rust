//! The Fresh Prince of Bel-Air generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Fresh Prince character
pub fn character() -> String {
    fetch_locale("the_fresh_prince_of_bel_air.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Will Smith".to_string())
}

/// Generate a random Fresh Prince actor
pub fn actor() -> String {
    fetch_locale("the_fresh_prince_of_bel_air.actors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Will Smith".to_string())
}

/// Generate a random Fresh Prince quote
pub fn quote() -> String {
    fetch_locale("the_fresh_prince_of_bel_air.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Now this is a story all about how my life got flipped-turned upside down.".to_string())
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
}

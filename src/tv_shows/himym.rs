//! How I Met Your Mother generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random How I Met Your Mother character
pub fn character() -> String {
    fetch_locale("how_i_met_your_mother.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ted Mosby".to_string())
}

/// Generate a random How I Met Your Mother catch phrase
pub fn catch_phrase() -> String {
    fetch_locale("how_i_met_your_mother.catch_phrase", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Legen-wait for it-dary!".to_string())
}

/// Generate a random How I Met Your Mother high five
pub fn high_five() -> String {
    fetch_locale("how_i_met_your_mother.high_five", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Self-five".to_string())
}

/// Generate a random How I Met Your Mother quote
pub fn quote() -> String {
    fetch_locale("how_i_met_your_mother.quote", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Suit up!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_himym_generation() {
        assert!(!character().is_empty());
        assert!(!catch_phrase().is_empty());
        assert!(!high_five().is_empty());
        assert!(!quote().is_empty());
    }
}

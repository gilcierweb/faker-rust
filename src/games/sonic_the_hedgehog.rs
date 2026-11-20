//! Sonic the Hedgehog generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Sonic the Hedgehog zone
pub fn zone() -> String {
    fetch_locale(
        "games.sonic_the_hedgehog.zone",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Green Hill Zone".to_string())
}

/// Generate a random Sonic the Hedgehog character
pub fn character() -> String {
    fetch_locale(
        "games.sonic_the_hedgehog.character",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Sonic the Hedgehog".to_string())
}

/// Generate a random Sonic the Hedgehog game title
pub fn game() -> String {
    fetch_locale(
        "games.sonic_the_hedgehog.game",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Sonic the Hedgehog 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sonic_generation() {
        assert!(!zone().is_empty());
        assert!(!character().is_empty());
        assert!(!game().is_empty());
    }
}

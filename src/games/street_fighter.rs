//! Street Fighter generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Street Fighter character
pub fn character() -> String {
    fetch_locale("street_fighter.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ryu".to_string())
}

/// Generate a random Street Fighter stage
pub fn stage() -> String {
    fetch_locale("street_fighter.stages", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Suzaku Castle".to_string())
}

/// Generate a random Street Fighter quote
pub fn quote() -> String {
    fetch_locale("street_fighter.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "You must defeat Sheng Long to stand a chance.".to_string())
}

/// Generate a random Street Fighter move
pub fn move_name() -> String {
    fetch_locale("street_fighter.moves", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hadoken".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_street_fighter_generation() {
        assert!(!character().is_empty());
        assert!(!stage().is_empty());
        assert!(!quote().is_empty());
        assert!(!move_name().is_empty());
    }
}

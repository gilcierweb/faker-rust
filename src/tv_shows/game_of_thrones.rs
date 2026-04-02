//! Game of Thrones generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Game of Thrones character
pub fn character() -> String {
    fetch_locale("game_of_thrones.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Jon Snow".to_string())
}

/// Generate a random Game of Thrones house
pub fn house() -> String {
    fetch_locale("game_of_thrones.houses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Stark".to_string())
}

/// Generate a random Game of Thrones city
pub fn city() -> String {
    fetch_locale("game_of_thrones.cities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Winterfell".to_string())
}

/// Generate a random Game of Thrones quote
pub fn quote() -> String {
    fetch_locale("game_of_thrones.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Winter is coming.".to_string())
}

/// Generate a random Game of Thrones dragon
pub fn dragon() -> String {
    fetch_locale("game_of_thrones.dragons", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Drogon".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_got_generation() {
        assert!(!character().is_empty());
        assert!(!house().is_empty());
        assert!(!city().is_empty());
        assert!(!quote().is_empty());
        assert!(!dragon().is_empty());
    }
}

//! Heroes of the Storm generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random battleground from Heroes of the Storm
pub fn battleground() -> String {
    fetch_locale("heroes_of_the_storm.battlegrounds", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Cursed Hollow".to_string())
}

/// Generate a random hero class from Heroes of the Storm
pub fn class_name() -> String {
    fetch_locale("heroes_of_the_storm.class_names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Assassin".to_string())
}

/// Generate a random hero name from Heroes of the Storm
pub fn hero() -> String {
    fetch_locale("heroes_of_the_storm.heroes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Arthas".to_string())
}

/// Generate a random quote from Heroes of the Storm
pub fn quote() -> String {
    fetch_locale("heroes_of_the_storm.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "For the Alliance!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hots_generation() {
        assert!(!battleground().is_empty());
        assert!(!class_name().is_empty());
        assert!(!hero().is_empty());
        assert!(!quote().is_empty());
    }
}

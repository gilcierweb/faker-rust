//! The Elder Scrolls generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Elder Scrolls race
pub fn race() -> String {
    fetch_locale("elder_scrolls.race", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Nord".to_string())
}

/// Generate a random Elder Scrolls creature
pub fn creature() -> String {
    fetch_locale("elder_scrolls.creature", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Mudcrab".to_string())
}

/// Generate a random Elder Scrolls region
pub fn region() -> String {
    fetch_locale("elder_scrolls.region", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Skyrim".to_string())
}

/// Generate a random Elder Scrolls dragon
pub fn dragon() -> String {
    fetch_locale("elder_scrolls.dragon", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Alduin".to_string())
}

/// Generate a random Elder Scrolls city
pub fn city() -> String {
    fetch_locale("elder_scrolls.city", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Whiterun".to_string())
}

/// Generate a random Elder Scrolls first name
pub fn first_name() -> String {
    fetch_locale("elder_scrolls.first_name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ulfric".to_string())
}

/// Generate a random Elder Scrolls last name
pub fn last_name() -> String {
    fetch_locale("elder_scrolls.last_name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Stormcloak".to_string())
}

/// Generate a random Elder Scrolls full name
pub fn name() -> String {
    format!("{} {}", first_name(), last_name())
}

/// Generate a random Elder Scrolls weapon
pub fn weapon() -> String {
    fetch_locale("elder_scrolls.weapon", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Iron Sword".to_string())
}

/// Generate a random Elder Scrolls jewelry
pub fn jewelry() -> String {
    fetch_locale("elder_scrolls.jewelry", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Amulet of Mara".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elder_scrolls_generation() {
        assert!(!race().is_empty());
        assert!(!creature().is_empty());
        assert!(!region().is_empty());
        assert!(!dragon().is_empty());
        assert!(!city().is_empty());
        assert!(!first_name().is_empty());
        assert!(!last_name().is_empty());
        assert!(!name().is_empty());
        assert!(!weapon().is_empty());
        assert!(!jewelry().is_empty());
    }
}

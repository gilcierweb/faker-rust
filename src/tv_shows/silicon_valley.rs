//! Silicon Valley generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Silicon Valley character
pub fn character() -> String {
    fetch_locale("silicon_valley.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Richard Hendricks".to_string())
}

/// Generate a random Silicon Valley company
pub fn company() -> String {
    fetch_locale("silicon_valley.companies", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Pied Piper".to_string())
}

/// Generate a random Silicon Valley quote
pub fn quote() -> String {
    fetch_locale("silicon_valley.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "I'm a pivot!".to_string())
}

/// Generate a random Silicon Valley app
pub fn app() -> String {
    fetch_locale("silicon_valley.apps", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Not Hotdog".to_string())
}

/// Generate a random Silicon Valley invention
pub fn invention() -> String {
    fetch_locale("silicon_valley.inventions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Box".to_string())
}

/// Generate a random Silicon Valley motto
pub fn motto() -> String {
    fetch_locale("silicon_valley.mottos", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Making the world a better place".to_string())
}

/// Generate a random Silicon Valley URL
pub fn url() -> String {
    fetch_locale("silicon_valley.urls", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "http://www.piedpiper.com".to_string())
}

/// Generate a random Silicon Valley email
pub fn email() -> String {
    fetch_locale("silicon_valley.email", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "richard@piedpiper.com".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silicon_valley_generation() {
        assert!(!character().is_empty());
        assert!(!company().is_empty());
        assert!(!quote().is_empty());
        assert!(!app().is_empty());
        assert!(!invention().is_empty());
        assert!(!motto().is_empty());
        assert!(!url().is_empty());
        assert!(!email().is_empty());
    }
}

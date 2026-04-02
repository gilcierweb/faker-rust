//! Twin Peaks generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Twin Peaks character
pub fn character() -> String {
    fetch_locale("twin_peaks.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Dale Cooper".to_string())
}

/// Generate a random Twin Peaks location
pub fn location() -> String {
    fetch_locale("twin_peaks.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Black Lodge".to_string())
}

/// Generate a random Twin Peaks quote
pub fn quote() -> String {
    fetch_locale("twin_peaks.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Damn fine coffee!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twin_peaks_generation() {
        assert!(!character().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
    }
}

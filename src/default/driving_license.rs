//! Driving License generator - generates driving license numbers for various regions

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random USA driving license number for a specific state
pub fn usa(state: &str) -> String {
    let key = format!("driving_license.{}", state);
    fetch_locale(&key, "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "123456789".to_string())
}

/// Generate a random Alabama driving license number
pub fn alabama() -> String {
    usa("alabama")
}

/// Generate a random California driving license number
pub fn california() -> String {
    usa("california")
}

/// Generate a random Florida driving license number
pub fn florida() -> String {
    usa("florida")
}

/// Generate a random New York driving license number
pub fn new_york() -> String {
    usa("new_york")
}

/// Generate a random Texas driving license number
pub fn texas() -> String {
    usa("texas")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driving_license_generation() {
        assert!(!alabama().is_empty());
        assert!(!california().is_empty());
        assert!(!florida().is_empty());
        assert!(!new_york().is_empty());
        assert!(!texas().is_empty());
    }
}

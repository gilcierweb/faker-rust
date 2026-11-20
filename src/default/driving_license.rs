//! Driving License generator - generates driving license numbers for various regions

use crate::base::sample;
use crate::config::FakerConfig;
use crate::locale::fetch_locale;

/// Generate a random UK driving licence number in DVLA format
pub fn uk() -> String {
    let config = FakerConfig::current();
    let surname_part = sample(&["SMITH", "JONES", "BROWN", "TAYLOR", "WILSON"]);
    let year = config.rand_range(1950, 2005);
    let month = config.rand_range(1, 12);
    let day = config.rand_range(1, 29);
    let initials = sample(&["AB", "CD", "EF", "GH", "IJ"]);

    format!(
        "{}{:02}{:02}{:02}{}9AA",
        &surname_part[..5.min(surname_part.len())],
        year % 100,
        month,
        day,
        initials
    )
}

/// Generate a random USA driving license number for a specific state
pub fn usa(state: &str) -> String {
    let key = format!("driving_licence.usa.{}", state);
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
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: US driving licence number for a state.
pub fn usa_driving_licence(state: &str) -> String {
    let normalized = state.trim().to_lowercase().replace(' ', "_");
    crate::locale::fetch_locale(&format!("driving_licence.usa.{normalized}"), "en")
        .map(|v| crate::base::bothify(&crate::base::sample(&v)))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uk_uses_canonical_module() {
        let licence = uk();
        assert_eq!(licence.len(), 16, "bad licence: {licence}");
        assert!(licence.ends_with("9AA"), "bad suffix: {licence}");
        assert!(
            licence[..5].chars().all(|c| c.is_ascii_uppercase()),
            "bad surname prefix: {licence}"
        );
        assert!(
            licence[5..11].chars().all(|c| c.is_ascii_digit()),
            "bad numeric fields: {licence}"
        );
        assert!(
            licence[11..13].chars().all(|c| c.is_ascii_uppercase()),
            "bad initials: {licence}"
        );
    }

    #[test]
    fn test_driving_license_generation() {
        assert!(!alabama().is_empty());
        assert!(!california().is_empty());
        assert!(!florida().is_empty());
        assert!(!new_york().is_empty());
        assert!(!texas().is_empty());
    }
}

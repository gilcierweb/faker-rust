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
// --- gem parity generators (algorithmic) ---
fn pad_uppercase(value: &str, len: usize) -> String {
    let letters: String = value
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    format!("{letters:<len$}")[..len].to_string()
}

/// Gem-compatible generator: British driving licence number.
pub fn british_driving_licence(
    last_name: &str,
    initials: &str,
    gender: &str,
    date_of_birth: &str,
) -> String {
    let parts: Vec<u32> = date_of_birth
        .split('-')
        .filter_map(|p| p.parse().ok())
        .collect();
    let (year, month, day) = (parts[0], parts[1], parts[2]);
    let gender_marker = if gender.eq_ignore_ascii_case("female") {
        50
    } else {
        0
    };
    let config = crate::config::FakerConfig::current();
    let letters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let checksum = format!(
        "{}{}{}",
        config.rand_u32(10),
        crate::base::sample(&letters),
        crate::base::sample(&letters)
    );
    format!(
        "{}{}{:02}{:02}{}{}",
        pad_uppercase(last_name, 5),
        (year / 10) % 10,
        month + gender_marker,
        day,
        year % 10,
        pad_uppercase(initials, 2),
    ) + &checksum
}

/// Gem-compatible generator: Northern Irish driving licence number.
pub fn northern_irish_driving_licence() -> String {
    let config = crate::config::FakerConfig::current();
    format!("{:08}", config.rand_u32(100_000_000))
}

/// Gem-compatible generator: UK driving licence number in either format.
pub fn uk_driving_licence() -> String {
    let config = crate::config::FakerConfig::current();
    if config.rand_f64() < 0.07 {
        northern_irish_driving_licence()
    } else {
        british_driving_licence("SMITH", "JS", "M", "1990-01-01")
    }
}

/// Error raised when an unsupported state code is supplied.
///
/// The gem defines `InvalidStatePassed` as a `StandardError` subclass with a
/// custom initializer; Rust adapts it to a plain error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidStatePassed {
    message: String,
}

impl InvalidStatePassed {
    /// Gem-compatible constructor with the gem's default message.
    pub fn initialize(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    /// Message carried by the error.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for InvalidStatePassed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for InvalidStatePassed {}

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

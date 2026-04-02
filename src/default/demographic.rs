//! Demographic generator - generates data related to population demographics

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random race
pub fn race() -> String {
    fetch_locale("demographic.race", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Caucasian".to_string())
}

/// Generate a random educational status
pub fn educational_status() -> String {
    fetch_locale("demographic.educational_status", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bachelor's Degree".to_string())
}

/// Generate a random demonym
pub fn demonym() -> String {
    fetch_locale("demographic.demonym", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Brazilian".to_string())
}

/// Generate a random marital status
pub fn marital_status() -> String {
    fetch_locale("demographic.marital_status", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Single".to_string())
}

/// Generate a random sex
pub fn sex() -> String {
    fetch_locale("demographic.sex", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Male".to_string())
}

/// Generate a random height (e.g., in meters)
pub fn height() -> String {
    fetch_locale("demographic.height", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "1.75".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demographic_generation() {
        assert!(!race().is_empty());
        assert!(!educational_status().is_empty());
        assert!(!demonym().is_empty());
        assert!(!marital_status().is_empty());
        assert!(!sex().is_empty());
        assert!(!height().is_empty());
    }
}

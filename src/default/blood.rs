//! Blood generator - generates random blood type and Rh factor

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random blood type (A, B, AB, O)
pub fn type_name() -> String {
    fetch_locale("blood.type", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "O".to_string())
}

/// Generate a random Rh factor (+, -)
pub fn rh_factor() -> String {
    fetch_locale("blood.rh_factor", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "+".to_string())
}

/// Generate a full blood type (e.g., O+)
pub fn group() -> String {
    format!("{}{}", type_name(), rh_factor())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `blood.type`.
pub fn r#type() -> String {
    parity_pick("blood.type")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        let g = group();
        assert!(!g.is_empty());
        assert!(g.contains('+') || g.contains('-'));
    }
}

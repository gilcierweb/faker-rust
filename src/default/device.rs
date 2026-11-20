//! Device generator - generates random device info

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random device model name
pub fn model_name() -> String {
    fetch_locale("device.model_name", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "iPhone X".to_string())
}

/// Generate a random device platform
pub fn platform() -> String {
    fetch_locale("device.platform", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "iOS".to_string())
}

/// Generate a random device manufacturer
pub fn manufacturer() -> String {
    fetch_locale("device.manufacturer", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Apple".to_string())
}

/// Generate a random device serial number
pub fn serial() -> String {
    sample(&[
        "ABC123456789".to_string(),
        "XYZ987654321".to_string(),
        "DEV123456789".to_string(),
        "SN12345678".to_string(),
    ])
}
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: build number between 1 and 500.
pub fn build_number() -> u32 {
    crate::config::FakerConfig::current().rand_range(1, 501)
}
/// Gem-compatible generator: version number between 1 and 1000.
pub fn version() -> u32 {
    crate::config::FakerConfig::current().rand_range(1, 1001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serial_is_alphanumeric_and_nonempty() {
        for _ in 0..20 {
            let s = serial();
            assert!(!s.is_empty());
            assert!(s.chars().all(|c| c.is_ascii_alphanumeric()), "serial {s}");
        }
    }
}

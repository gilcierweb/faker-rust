//! Device generator - generates random device info

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random device model name
pub fn model_name() -> String {
    fetch_locale("device.model_name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "iPhone X".to_string())
}

/// Generate a random device platform
pub fn platform() -> String {
    fetch_locale("device.platform", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "iOS".to_string())
}

/// Generate a random device manufacturer
pub fn manufacturer() -> String {
    fetch_locale("device.manufacturer", "en")
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

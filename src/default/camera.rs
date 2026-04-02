//! Camera generator - generates random camera-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random camera brand
pub fn brand() -> String {
    fetch_locale("camera.brand", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Canon".to_string())
}

/// Generate a random camera model
pub fn model() -> String {
    fetch_locale("camera.model", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "EOS 5D Mark IV".to_string())
}

/// Generate a random camera brand with model
pub fn brand_with_model() -> String {
    fetch_locale("camera.brand_with_model", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| format!("{} {}", brand(), model()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brand_with_model() {
        let b = brand_with_model();
        assert!(!b.is_empty());
    }
}

//! Color generator - generates random color names

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random color name
pub fn name() -> String {
    fetch_locale("color.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "red".to_string())
}

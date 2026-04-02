//! Hobby generator - generates random hobby activities

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random hobby activity
pub fn activity() -> String {
    fetch_locale("hobby.activity", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Reading".to_string())
}

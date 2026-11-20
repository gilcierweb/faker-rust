//! Rajnikanth quotes generator.

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Rajnikanth joke.
pub fn joke() -> String {
    fetch_locale("rajnikanth.joke", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_default()
}

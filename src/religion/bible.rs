//! Bible data generator.

use crate::base::sample;
use crate::locale::fetch_locale;

fn pick(key: &str) -> String {
    fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_default()
}

/// Generate a random Bible character.
pub fn character() -> String {
    pick("bible.character")
}

/// Generate a random Bible location.
pub fn location() -> String {
    pick("bible.location")
}

/// Generate a random Bible quote.
pub fn quote() -> String {
    pick("bible.quote")
}

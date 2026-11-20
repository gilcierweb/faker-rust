//! Chiquito quotes generator.

use crate::base::sample;
use crate::locale::fetch_locale;

fn pick(key: &str) -> String {
    fetch_locale(key, "en")
        .map(|v| sample(&v))
        .unwrap_or_default()
}

/// Generate a random Chiquito expression.
pub fn expression() -> String {
    pick("chiquito.expressions")
}

/// Generate a random Chiquito term.
pub fn term() -> String {
    pick("chiquito.terms")
}

/// Generate a random Chiquito joke.
pub fn joke() -> String {
    pick("chiquito.jokes")
}

/// Generate a random Chiquito sentence.
pub fn sentence() -> String {
    pick("chiquito.sentences")
}

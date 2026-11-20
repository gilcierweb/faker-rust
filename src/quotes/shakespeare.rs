//! Shakespeare quotes generator.

use crate::base::sample;
use crate::locale::fetch_locale;

fn pick(key: &str) -> String {
    fetch_locale(key, "en")
        .map(|v| sample(&v))
        .unwrap_or_default()
}

/// Generate a random quote from Hamlet.
pub fn hamlet_quote() -> String {
    pick("shakespeare.hamlet")
}

/// Generate a random quote from As You Like It.
pub fn as_you_like_it_quote() -> String {
    pick("shakespeare.as_you_like_it")
}

/// Generate a random quote from King Richard III.
pub fn king_richard_iii_quote() -> String {
    pick("shakespeare.king_richard_iii")
}

/// Generate a random quote from Romeo and Juliet.
pub fn romeo_and_juliet_quote() -> String {
    pick("shakespeare.romeo_and_juliet")
}

//! Shakespeare quotes generator.

use crate::base::sample;
use crate::locale::fetch_locale;

/// Gem-compatible alias for [`as_you_like_it_quote`].
pub use self::as_you_like_it_quote as as_you_like_it;
/// Gem-compatible alias for [`hamlet_quote`].
pub use self::hamlet_quote as hamlet;
/// Gem-compatible alias for [`king_richard_iii_quote`].
pub use self::king_richard_iii_quote as king_richard_iii;
/// Gem-compatible alias for [`romeo_and_juliet_quote`].
pub use self::romeo_and_juliet_quote as romeo_and_juliet;

fn pick(key: &str) -> String {
    fetch_locale(key, &crate::config::FakerConfig::locale())
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

//! Chuck Norris generator - generates random Chuck Norris facts

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Chuck Norris fact
pub fn fact() -> String {
    fetch_locale_with_context("chuck_norris.fact", "en", Some("chuck_norris"))
        .map(|v| sample_with_resolve(&v, Some("chuck_norris")))
        .unwrap_or_else(|| sample(FALLBACK_FACTS).to_string())
}

// Fallback data
const FALLBACK_FACTS: &[&str] = &[
    "Chuck Norris counted to infinity. Twice.",
    "Chuck Norris can slam a revolving door.",
    "Chuck Norris doesn't wear a watch. He decides what time it is.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        assert!(!fact().is_empty());
    }
}

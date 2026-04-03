//! App generator - generates random app names, versions, and authors

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random app name
pub fn name() -> String {
    fetch_locale_with_context("app.name", "en", Some("app"))
        .map(|v| sample_with_resolve(&v, Some("app")))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random app version
pub fn version() -> String {
    fetch_locale_with_context("app.version", "en", Some("app"))
        .map(|v| sample_with_resolve(&v, Some("app")))
        .unwrap_or_else(|| sample(FALLBACK_VERSIONS).to_string())
}

/// Generate a random app author
pub fn author() -> String {
    fetch_locale_with_context("app.author", "en", Some("app"))
        .map(|v| sample_with_resolve(&v, Some("app")))
        .unwrap_or_else(crate::name::name)
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &["Redhold", "Treeflex", "Tri-tip", "Greenlam"];
const FALLBACK_VERSIONS: &[&str] = &["0.1.0", "1.0.0", "2.1.3", "0.9.9"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}

//! Code generator - generates random ISBN and EAN codes

use crate::base::bothify;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random ASIN code
pub fn asin() -> String {
    fetch_locale_with_context("code.asin", "en", Some("code"))
        .map(|v| sample_with_resolve(&v, Some("code")))
        .unwrap_or_else(|| bothify("B000#######"))
}

/// Generate a random ISBN-10 code
pub fn isbn() -> String {
    fetch_locale_with_context("code.isbn", "en", Some("code"))
        .map(|v| sample_with_resolve(&v, Some("code")))
        .unwrap_or_else(|| bothify("#########-#"))
}

/// Generate a random EAN code
pub fn ean() -> String {
    fetch_locale_with_context("code.ean", "en", Some("code"))
        .map(|v| sample_with_resolve(&v, Some("code")))
        .unwrap_or_else(|| bothify("#############"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isbn() {
        assert!(!isbn().is_empty());
    }

    #[test]
    fn test_ean() {
        assert!(!ean().is_empty());
    }
}

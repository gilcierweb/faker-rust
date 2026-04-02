//! University generator - generates random university names

use crate::base::sample;
use crate::locale::{fetch_locale, sample_with_resolve};

/// Generate a random university name
pub fn name() -> String {
    fetch_locale("university.name", "en")
        .map(|v| sample_with_resolve(&v, Some("university")))
        .unwrap_or_else(|| "Harvard University".to_string())
}

/// Generate a random university prefix
pub fn prefix() -> String {
    fetch_locale("university.prefix", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "University".to_string())
}

/// Generate a random university suffix
pub fn suffix() -> String {
    fetch_locale("university.suffix", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "University".to_string())
}

/// Generate a random greek alphabet letter
pub fn greek_alphabet() -> String {
    fetch_locale("university.greek_alphabet", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Alpha".to_string())
}

/// Generate a random greek organization name
pub fn greek_organization() -> String {
    let prefix = greek_alphabet();
    let suffix = sample(&[
        "Alpha".to_string(),
        "Beta".to_string(),
        "Gamma".to_string(),
        "Delta".to_string(),
        "Epsilon".to_string(),
        "Zeta".to_string(),
        "Theta".to_string(),
        "Kappa".to_string(),
        "Lambda".to_string(),
        "Omega".to_string(),
    ]);
    format!("{} {}", prefix, suffix)
}

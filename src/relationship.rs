//! Relationship generator - generates random relationship terms

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a direct familial relationship term
pub fn direct() -> String {
    fetch_locale("relationship.familial.direct", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Father".to_string())
}

/// Generate an extended familial relationship term
pub fn extended() -> String {
    fetch_locale("relationship.familial.extended", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Grandfather".to_string())
}

/// Generate an in-law relationship term
pub fn in_law() -> String {
    fetch_locale("relationship.in_law", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Father-in-law".to_string())
}

/// Generate a spouse relationship term
pub fn spouse() -> String {
    fetch_locale("relationship.spouse", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Husband".to_string())
}

/// Generate a parent relationship term
pub fn parent() -> String {
    fetch_locale("relationship.parent", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Father".to_string())
}

/// Generate a sibling relationship term
pub fn sibling() -> String {
    fetch_locale("relationship.sibling", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sister".to_string())
}

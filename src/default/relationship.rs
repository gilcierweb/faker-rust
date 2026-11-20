//! Relationship generator - generates random relationship terms

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a direct familial relationship term
pub fn direct() -> String {
    fetch_locale(
        "relationship.familial.direct",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Father".to_string())
}

/// Generate an extended familial relationship term
pub fn extended() -> String {
    fetch_locale(
        "relationship.familial.extended",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Grandfather".to_string())
}

/// Generate an in-law relationship term
pub fn in_law() -> String {
    fetch_locale("relationship.in_law", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Father-in-law".to_string())
}

/// Generate a spouse relationship term
pub fn spouse() -> String {
    fetch_locale("relationship.spouse", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Husband".to_string())
}

/// Generate a parent relationship term
pub fn parent() -> String {
    fetch_locale("relationship.parent", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Father".to_string())
}

/// Generate a sibling relationship term
pub fn sibling() -> String {
    fetch_locale(
        "relationship.sibling",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Sister".to_string())
}
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: familial relationship from a named group.
pub fn familial(connection: Option<&str>) -> String {
    let groups = crate::locale::fetch_locale_keys(
        "relationship.familial",
        &crate::config::FakerConfig::locale(),
    );
    let selected = match connection {
        Some(name) => name.to_lowercase(),
        None => crate::base::sample(&groups),
    };
    crate::locale::fetch_locale(
        &format!("relationship.familial.{selected}"),
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| crate::base::sample(&v))
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_terms_are_nonempty() {
        for term in [
            direct(),
            extended(),
            in_law(),
            spouse(),
            parent(),
            sibling(),
        ] {
            assert!(!term.is_empty());
            assert!(!term.contains("#{"), "unresolved: {term}");
        }
    }
}

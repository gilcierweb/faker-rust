//! Doctor Who generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Doctor Who character
pub fn character() -> String {
    fetch_locale("dr_who.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Doctor".to_string())
}

/// Generate a random Doctor from Doctor Who
pub fn the_doctor() -> String {
    fetch_locale("dr_who.the_doctors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Tenth Doctor".to_string())
}

/// Generate a random Doctor Who actor
pub fn actor() -> String {
    fetch_locale("dr_who.actors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "David Tennant".to_string())
}

/// Generate a random Doctor Who catch phrase
pub fn catch_phrase() -> String {
    fetch_locale("dr_who.catch_phrases", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Allons-y!".to_string())
}

/// Generate a random Doctor Who quote
pub fn quote() -> String {
    fetch_locale("dr_who.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| {
            "We're all stories, in the end. Just make it a good one, eh?".to_string()
        })
}

/// Generate a random Doctor Who villain
pub fn villain() -> String {
    fetch_locale("dr_who.villains", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Dalek".to_string())
}

/// Generate a random Doctor Who species
pub fn species() -> String {
    fetch_locale("dr_who.species", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Time Lord".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dr_who_generation() {
        assert!(!character().is_empty());
        assert!(!the_doctor().is_empty());
        assert!(!actor().is_empty());
        assert!(!catch_phrase().is_empty());
        assert!(!quote().is_empty());
        assert!(!villain().is_empty());
        assert!(!species().is_empty());
    }
}

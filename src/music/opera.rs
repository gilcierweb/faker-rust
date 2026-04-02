//! Opera music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random opera name
pub fn name() -> String {
    fetch_locale("opera.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random opera composer
pub fn composer() -> String {
    fetch_locale("opera.composers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COMPOSERS).to_string())
}

/// Generate a random opera language
pub fn language() -> String {
    fetch_locale("opera.languages", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LANGUAGES).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "La Traviata",
    "La Bohème",
    "Carmen",
    "The Marriage of Figaro",
    "Tosca",
    "Aida",
    "Madama Butterfly",
    "The Barber of Seville",
    "Rigoletto",
    "The Magic Flute",
    "Don Giovanni",
    "Turandot",
    "Nabucco",
    "Falstaff",
];

const FALLBACK_COMPOSERS: &[&str] = &[
    "Giuseppe Verdi",
    "Wolfgang Amadeus Mozart",
    "Giacomo Puccini",
    "Georges Bizet",
    "Richard Wagner",
    "Gioachino Rossini",
    "Ludwig van Beethoven",
    "Gaetano Donizetti",
];

const FALLBACK_LANGUAGES: &[&str] = &["Italian", "German", "French", "English", "Russian"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_composer() {
        assert!(!composer().is_empty());
    }

    #[test]
    fn test_language() {
        assert!(!language().is_empty());
    }
}

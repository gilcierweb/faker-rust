//! Opera music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random opera name
pub fn name() -> String {
    fetch_locale("opera.names", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random opera composer
pub fn composer() -> String {
    fetch_locale("opera.composers", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COMPOSERS).to_string())
}

/// Generate a random opera language
pub fn language() -> String {
    fetch_locale("opera.languages", &crate::config::FakerConfig::locale())
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

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `opera.german.by_ludwig_van_beethoven`.
pub fn beethoven() -> String {
    parity_pick("opera.german.by_ludwig_van_beethoven")
}

/// Gem-compatible generator: `opera.italian.by_vincenzo_bellini`.
pub fn bellini() -> String {
    parity_pick("opera.italian.by_vincenzo_bellini")
}

/// Gem-compatible generator: `opera.german.by_alban_berg`.
pub fn berg() -> String {
    parity_pick("opera.german.by_alban_berg")
}

/// Gem-compatible generator: `opera.french.by_hector_berlioz`.
pub fn berlioz() -> String {
    parity_pick("opera.french.by_hector_berlioz")
}

/// Gem-compatible generator: `opera.french.by_georges_bizet`.
pub fn bizet() -> String {
    parity_pick("opera.french.by_georges_bizet")
}

/// Gem-compatible generator: `opera.italian.by_gaetano_donizetti`.
pub fn donizetti() -> String {
    parity_pick("opera.italian.by_gaetano_donizetti")
}

/// Gem-compatible generator: `opera.italian.by_christoph_willibald_gluck`.
pub fn gluck() -> String {
    parity_pick("opera.italian.by_christoph_willibald_gluck")
}

/// Gem-compatible generator: `opera.french.by_christoph_willibald_gluck`.
pub fn gluck_french() -> String {
    parity_pick("opera.french.by_christoph_willibald_gluck")
}

/// Gem-compatible generator: `opera.italian.by_christoph_willibald_gluck`.
pub fn gluck_italian() -> String {
    parity_pick("opera.italian.by_christoph_willibald_gluck")
}

/// Gem-compatible generator: `opera.french.by_charles_gounod`.
pub fn gounod() -> String {
    parity_pick("opera.french.by_charles_gounod")
}

/// Gem-compatible generator: `opera.italian.by_wolfgang_amadeus_mozart`.
pub fn mozart() -> String {
    parity_pick("opera.italian.by_wolfgang_amadeus_mozart")
}

/// Gem-compatible generator: `opera.german.by_wolfgang_amadeus_mozart`.
pub fn mozart_german() -> String {
    parity_pick("opera.german.by_wolfgang_amadeus_mozart")
}

/// Gem-compatible generator: `opera.italian.by_wolfgang_amadeus_mozart`.
pub fn mozart_italian() -> String {
    parity_pick("opera.italian.by_wolfgang_amadeus_mozart")
}

/// Gem-compatible generator: `opera.french.by_maurice_ravel`.
pub fn ravel() -> String {
    parity_pick("opera.french.by_maurice_ravel")
}

/// Gem-compatible generator: `opera.italian.by_gioacchino_rossini`.
pub fn rossini() -> String {
    parity_pick("opera.italian.by_gioacchino_rossini")
}

/// Gem-compatible generator: `opera.french.by_camille_saint_saens`.
pub fn saint_saens() -> String {
    parity_pick("opera.french.by_camille_saint_saens")
}

/// Gem-compatible generator: `opera.german.by_franz_schubert`.
pub fn schubert() -> String {
    parity_pick("opera.german.by_franz_schubert")
}

/// Gem-compatible generator: `opera.german.by_robert_schumann`.
pub fn schumann() -> String {
    parity_pick("opera.german.by_robert_schumann")
}

/// Gem-compatible generator: `opera.german.by_richard_strauss`.
pub fn strauss() -> String {
    parity_pick("opera.german.by_richard_strauss")
}

/// Gem-compatible generator: `opera.italian.by_giuseppe_verdi`.
pub fn verdi() -> String {
    parity_pick("opera.italian.by_giuseppe_verdi")
}

/// Gem-compatible generator: `opera.german.by_richard_wagner`.
pub fn wagner() -> String {
    parity_pick("opera.german.by_richard_wagner")
}

/// Gem-compatible generator: `opera.german.by_carl_maria_von_weber`.
pub fn weber() -> String {
    parity_pick("opera.german.by_carl_maria_von_weber")
}

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

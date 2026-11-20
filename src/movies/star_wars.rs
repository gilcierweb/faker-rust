//! Star Wars generator - generates random Star Wars characters

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Star Wars pilot call sign
pub fn call_sign() -> String {
    fetch_locale_with_context("star_wars.call_sign", "en", Some("star_wars"))
        .map(|v| sample_with_resolve(&v, Some("star_wars")))
        .unwrap_or_else(|| "Red Leader".to_string())
}

/// Generate a random Star Wars character
pub fn character() -> String {
    fetch_locale_with_context("star_wars.characters", "en", Some("star_wars"))
        .map(|v| sample_with_resolve(&v, Some("star_wars")))
        .unwrap_or_else(|| sample(FALLBACK_STAR_WARS_CHARACTERS).to_string())
}

const FALLBACK_STAR_WARS_CHARACTERS: &[&str] = &[
    "Luke Skywalker",
    "Darth Vader",
    "Han Solo",
    "Princess Leia",
    "Obi-Wan Kenobi",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

// --- gem parity `fetch_all` equivalents (all locale values) ---
fn parity_all(key: &str) -> Vec<String> {
    crate::locale::fetch_locale(key, "en").unwrap_or_default()
}

/// Gem-compatible generator: all pilot call numbers.
pub fn call_numbers() -> Vec<String> {
    parity_all("star_wars.call_numbers")
}

/// Gem-compatible generator: all call squadrons.
pub fn call_squadrons() -> Vec<String> {
    parity_all("star_wars.call_squadrons")
}

/// Gem-compatible generator: all characters.
pub fn characters() -> Vec<String> {
    parity_all("star_wars.characters")
}

/// Gem-compatible generator: all droids.
pub fn droids() -> Vec<String> {
    parity_all("star_wars.droids")
}

/// Gem-compatible generator: all planets.
pub fn planets() -> Vec<String> {
    parity_all("star_wars.planets")
}

/// Gem-compatible generator: all species.
pub fn species() -> Vec<String> {
    parity_all("star_wars.species")
}

/// Gem-compatible generator: all vehicles.
pub fn vehicles() -> Vec<String> {
    parity_all("star_wars.vehicles")
}

/// Gem-compatible generator: all Wookiee words.
pub fn wookiee_words() -> Vec<String> {
    parity_all("star_wars.wookiee_words")
}

/// Gem-compatible generator: `star_wars.call_numbers`.
pub fn call_number() -> String {
    parity_pick("star_wars.call_numbers")
}

/// Gem-compatible generator: `star_wars.call_squadrons`.
pub fn call_squadron() -> String {
    parity_pick("star_wars.call_squadrons")
}

/// Gem-compatible generator: `star_wars.droids`.
pub fn droid() -> String {
    parity_pick("star_wars.droids")
}

/// Gem-compatible generator: `star_wars.planets`.
pub fn planet() -> String {
    parity_pick("star_wars.planets")
}

/// Gem-compatible generator: `star_wars.species`.
pub fn specie() -> String {
    parity_pick("star_wars.species")
}

/// Gem-compatible generator: `star_wars.vehicles`.
pub fn vehicle() -> String {
    parity_pick("star_wars.vehicles")
}
// --- gem parity generators (locale-backed) ---
/// Gem-compatible generator: Wookiee sentence.
pub fn wookiee_sentence() -> String {
    let words = crate::locale::fetch_locale("star_wars.wookiee_words", "en")
        .unwrap_or_else(|| vec!["wyaaaaaa".to_string()]);
    let config = crate::config::FakerConfig::current();
    let mut sentence = crate::base::sample(&words);
    if let Some(first) = sentence.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    for _ in 0..config.rand_u32(11) {
        sentence.push(' ');
        sentence.push_str(&crate::base::sample(&words));
    }
    sentence
}
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: Star Wars quote, optionally from a character.
pub fn quote(character: Option<&str>) -> String {
    let characters = crate::locale::fetch_locale_keys("star_wars.quotes", "en");
    let selected = match character {
        Some(name) => name.to_lowercase(),
        None => crate::base::sample(&characters),
    };
    crate::locale::fetch_locale(&format!("star_wars.quotes.{selected}"), "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_call_sign() {
        for _ in 0..50 {
            let sign = call_sign();
            assert!(!sign.is_empty());
            assert!(!sign.contains("#{"), "unresolved placeholder: {sign}");
        }
    }
}

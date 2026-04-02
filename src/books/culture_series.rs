//! Culture Series (Iain M. Banks) book generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Culture Series book title
pub fn book() -> String {
    fetch_locale("culture_series.books", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BOOKS).to_string())
}

/// Generate a random Culture Series ship name
pub fn ship() -> String {
    fetch_locale("culture_series.ships", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SHIPS).to_string())
}

/// Generate a random Culture Series character
pub fn character() -> String {
    fetch_locale("culture_series.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Culture Series civilization
pub fn civ() -> String {
    fetch_locale("culture_series.civs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CIVS).to_string())
}

// Fallback data
const FALLBACK_BOOKS: &[&str] = &[
    "Consider Phlebas",
    "The Player of Games",
    "Use of Weapons",
    "Excession",
    "Look to Windward",
    "Matter",
    "Surface Detail",
    "The Hydrogen Sonata",
    "The State of the Art",
];

const FALLBACK_SHIPS: &[&str] = &[
    "Sleeper Service",
    "Mistake Not...",
    "Fate Amenable To Change",
    "Anticipation Of A New Lover's Arrival",
    "Shoot Them Later",
    "Gangster Of Love",
    "Frank Exchange Of Views",
    "Nervous Energy",
    "Irregular Apocalypse",
    "Limiting Factor",
];

const FALLBACK_CHARACTERS: &[&str] = &[
    "The Mind of the Masaq' Orbital",
    "Byr Genar-Hofoen",
    "Zakalwe",
    "Diziet Sma",
    "Bora Horza Gobuchul",
    "Balveda",
    "Kraiklyn",
    "Jernau Morat Gurgeh",
    "Chamlis Amalk-ney",
];

const FALLBACK_CIVS: &[&str] = &[
    "The Culture",
    "The Idirans",
    "The Affront",
    "The Morthanveld",
    "The Azad",
    "The Gzilt",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book() {
        assert!(!book().is_empty());
    }

    #[test]
    fn test_ship() {
        assert!(!ship().is_empty());
    }

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_civ() {
        assert!(!civ().is_empty());
    }
}

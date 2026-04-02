//! BoJack Horseman TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random BoJack Horseman character
pub fn character() -> String {
    fetch_locale("bojack_horseman.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random BoJack Horseman quote
pub fn quote() -> String {
    fetch_locale("bojack_horseman.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Hollywoo location
pub fn location() -> String {
    fetch_locale("bojack_horseman.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "BoJack Horseman",
    "Diane Nguyen",
    "Princess Carolyn",
    "Todd Chavez",
    "Mr. Peanutbutter",
    "Hollyhock",
    "Beatrice Horseman",
    "Herb Kazzaz",
    "Kelsey Jannings",
    "Judah Mannowdog",
    "Rutabaga Rabbitowitz",
    "Stefani Stilton",
    "Sextina Aquafina",
    "Vincent Adultman",
    "Charlotte Moore",
    "Sarah Lynn",
    "Secretariat",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I'm BoJack the Horse, don't act like you don't know.",
    "You are everything that's wrong with you.",
    "It gets easier. Every day it gets a little easier.",
    "Suck a dick, dumb shits!",
    "I'm responsible for my own happiness? I can't even be responsible for my own breakfast!",
    "Life is a bitch and then you die, right?",
    "Sometimes life's a bitch and then you keep living.",
    "You need to be better.",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Hollywoo",
    "Horsin' Around Set",
    "VIM Office",
    "BoJack's House",
    "Mr. Peanutbutter's House",
    "Pacific Ocean City",
    "Griffith Observatory",
    "Underground",  
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }
}

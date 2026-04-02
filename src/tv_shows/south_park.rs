//! South Park TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random South Park character
pub fn character() -> String {
    fetch_locale("south_park.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random South Park quote
pub fn quote() -> String {
    fetch_locale("south_park.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Stan Marsh",
    "Kyle Broflovski",
    "Eric Cartman",
    "Kenny McCormick",
    "Butters Stotch",
    "Randy Marsh",
    "Sharon Marsh",
    "Shelly Marsh",
    "Gerald Broflovski",
    "Sheila Broflovski",
    "Ike Broflovski",
    "Wendy Testaburger",
    "Chef",
    "Mr. Garrison",
    "Mr. Mackey",
    "Principal Victoria",
    "PC Principal",
    "Towelie",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Oh my God! They killed Kenny!",
    "Respect my authoritah!",
    "Screw you guys, I'm going home.",
    "Mmph mmph mmph!",
    "I'm not fat, I'm big-boned!",
    "You bastards!",
    "Screw you guys.",
    "Well, I'll be damned.",
    "Hot topic!",
    "I learned something today...",
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
}

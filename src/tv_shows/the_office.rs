//! The Office TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random The Office character
pub fn character() -> String {
    fetch_locale("the_office.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random The Office quote
pub fn quote() -> String {
    fetch_locale("the_office.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Michael Scott",
    "Dwight Schrute",
    "Jim Halpert",
    "Pam Beesly",
    "Ryan Howard",
    "Andy Bernard",
    "Stanley Hudson",
    "Kevin Malone",
    "Meredith Palmer",
    "Angela Martin",
    "Oscar Martinez",
    "Phyllis Vance",
    "Creed Bratton",
    "Toby Flenderson",
    "Darryl Philbin",
    "Kelly Kapoor",
    "Erin Hannon",
    "Jan Levinson",
    "David Wallace",
    "Holly Flax",
];

const FALLBACK_QUOTES: &[&str] = &[
    "That's what she said!",
    "I'm not superstitious, but I am a little stitious.",
    "Would I rather be feared or loved? Easy. Both.",
    "I have flaws. What are they? I sing in the shower.",
    "Identity theft is not a joke, Jim!",
    "Bears. Beets. Battlestar Galactica.",
    "I'm dead inside.",
    "Threat level midnight!",
    "Ain't no party like a Scranton party!",
    "Just poopin'. You know how I be.",
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

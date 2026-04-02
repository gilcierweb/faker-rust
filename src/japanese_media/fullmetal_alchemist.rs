//! Fullmetal Alchemist Brotherhood anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Fullmetal Alchemist character
pub fn character() -> String {
    fetch_locale("fullmetal_alchemist.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Fullmetal Alchemist quote
pub fn quote() -> String {
    fetch_locale("fullmetal_alchemist.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random alchemical ability
pub fn ability() -> String {
    fetch_locale("fullmetal_alchemist.abilities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ABILITIES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Edward Elric",
    "Alphonse Elric",
    "Winry Rockbell",
    "Roy Mustang",
    "Riza Hawkeye",
    "Scar",
    "Ling Yao",
    "Lan Fan",
    "Olivier Mira Armstrong",
    "Alex Louis Armstrong",
    "Maes Hughes",
    "Izumi Curtis",
    "Van Hohenheim",
    "Father",
    "Envy",
    "Gluttony",
    "Greed",
    "Lust",
    "Pride",
    "Sloth",
    "Wrath",
];

const FALLBACK_QUOTES: &[&str] = &[
    "A lesson without pain is meaningless.",
    "Human kind cannot obtain anything without first giving something in return.",
    "The world's not perfect, but it's there for us trying the best it can.",
    "I will never give up.",
    "Stand up and walk. Keep moving forward.",
    "You've got two good legs. So use them.",
    "I am a firm believer in survival of the fittest.",
];

const FALLBACK_ABILITIES: &[&str] = &[
    "Fire Alchemy",
    "Water Alchemy",
    "Earth Alchemy",
    "Wind Alchemy",
    "Metal Alchemy",
    "Light Alchemy",
    "Shadow Alchemy",
    "Human Transmutation",
    "Automail Combat",
    "Xing Alchemy",
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
    fn test_ability() {
        assert!(!ability().is_empty());
    }
}

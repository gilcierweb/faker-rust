//! Studio Ghibli anime generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Studio Ghibli movie
pub fn movie() -> String {
    fetch_locale("studio_ghibli.movies", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MOVIES).to_string())
}

/// Generate a random Studio Ghibli character
pub fn character() -> String {
    fetch_locale("studio_ghibli.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Studio Ghibli quote
pub fn quote() -> String {
    fetch_locale("studio_ghibli.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_MOVIES: &[&str] = &[
    "Spirited Away",
    "My Neighbor Totoro",
    "Princess Mononoke",
    "Howl's Moving Castle",
    "Kiki's Delivery Service",
    "Nausicaä of the Valley of the Wind",
    "Castle in the Sky",
    "Ponyo",
    "The Wind Rises",
    "Grave of the Fireflies",
    "Arrietty",
    "Whisper of the Heart",
    "The Cat Returns",
    "Tales from Earthsea",
    "From Up on Poppy Hill",
    "The Tale of the Princess Kaguya",
    "When Marnie Was There",
    "Earwig and the Witch",
];

const FALLBACK_CHARACTERS: &[&str] = &[
    "Chihiro",
    "No-Face",
    "Haku",
    "Yubaba",
    "Totoro",
    "Satsuki",
    "Mei",
    "Catbus",
    "Ashitaka",
    "San",
    "Lady Eboshi",
    "Howl",
    "Sophie",
    "Calcifer",
    "Kiki",
    "Jiji",
    "Nausicaä",
    "Sheeta",
    "Pazu",
    "Ponyo",
    "Sosuke",
    "Jiro",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Once you've met someone you never really forget them.",
    "Always believe in yourself.",
    "It's not really important what color your dress is. It's how you wear it.",
    "The brave do not live forever, but the cautious do not live at all.",
    "You cannot alter your fate. However, you can rise to meet it.",
    "Life is suffering. It is hard. The world is cursed. But still, you find reasons to keep living.",
    "They say that the best blaze burns brightest when circumstances are at their worst.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie() {
        assert!(!movie().is_empty());
    }

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

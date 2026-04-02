//! H.P. Lovecraft book generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Lovecraft creature/deity
pub fn creature() -> String {
    fetch_locale("lovecraft.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATURES).to_string())
}

/// Generate a random Lovecraft location
pub fn location() -> String {
    fetch_locale("lovecraft.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Lovecraft book/tome
pub fn book() -> String {
    fetch_locale("lovecraft.books", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BOOKS).to_string())
}

/// Generate a random Lovecraft quote
pub fn quote() -> String {
    fetch_locale("lovecraft.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CREATURES: &[&str] = &[
    "Cthulhu",
    "Azathoth",
    "Nyarlathotep",
    "Shub-Niggurath",
    "Yog-Sothoth",
    "Dagon",
    "Hastur",
    "Yithian",
    "Deep One",
    "Mi-Go",
    "Elder Thing",
    "Shoggoth",
    "Night-gaunt",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "R'lyeh",
    "Arkham",
    "Innsmouth",
    "Dunwich",
    "Kingsport",
    "Miskatonic University",
    "The Mountains of Madness",
    "Plateau of Leng",
    "Dreamlands",
];

const FALLBACK_BOOKS: &[&str] = &[
    "Necronomicon",
    "De Vermis Mysteriis",
    "Book of Eibon",
    "Cultes des Goules",
    "Unaussprechlichen Kulten",
    "Pnakotic Manuscripts",
    "Eltdown Shards",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn.",
    "That is not dead which can eternal lie, and with strange aeons even death may die.",
    "The oldest and strongest emotion of mankind is fear, and the oldest and strongest kind of fear is fear of the unknown.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creature() {
        assert!(!creature().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_book() {
        assert!(!book().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

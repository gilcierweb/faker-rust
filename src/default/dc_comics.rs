//! DC Comics generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random DC Comics character
pub fn hero() -> String {
    fetch_locale("dc_comics.heroes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HEROES).to_string())
}

/// Generate a random DC Comics villain
pub fn villain() -> String {
    fetch_locale("dc_comics.villains", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_VILLAINS).to_string())
}

/// Generate a random DC Comics title
pub fn title() -> String {
    fetch_locale("dc_comics.titles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TITLES).to_string())
}

// Fallback data
const FALLBACK_HEROES: &[&str] = &[
    "Superman", "Batman", "Wonder Woman", "The Flash", "Green Lantern",
    "Aquaman", "Cyborg", "Shazam", "Green Arrow", "Nightwing", "Batgirl",
    "Supergirl", "Martian Manhunter", "Hawkman", "Hawkgirl", "Zatanna",
    "Constantine", "Swamp Thing", "Doctor Fate", "The Atom", "Blue Beetle",
];

const FALLBACK_VILLAINS: &[&str] = &[
    "Joker", "Lex Luthor", "Darkseid", "Brainiac", "Doomsday",
    "Deathstroke", "Harley Quinn", "Catwoman", "Poison Ivy", "Bane",
    "Penguin", "Riddler", "Two-Face", "Scarecrow", "Ra's al Ghul",
    "Reverse-Flash", "Sinestro", "Black Manta", "General Zod", "Lobo",
];

const FALLBACK_TITLES: &[&str] = &[
    "Action Comics", "Detective Comics", "Batman", "Superman", "Wonder Woman",
    "The Flash", "Green Lantern", "Justice League", "Teen Titans", "Suicide Squad",
    "Aquaman", "Shazam", "Swamp Thing", "Watchmen", "V for Vendetta",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hero() {
        assert!(!hero().is_empty());
    }

    #[test]
    fn test_villain() {
        assert!(!villain().is_empty());
    }

    #[test]
    fn test_title() {
        assert!(!title().is_empty());
    }
}

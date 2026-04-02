//! Naruto anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Naruto character
pub fn character() -> String {
    fetch_locale("naruto.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Naruto village
pub fn village() -> String {
    fetch_locale("naruto.villages", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_VILLAGES).to_string())
}

/// Generate a random Naruto jutsu/technique
pub fn jutsu() -> String {
    fetch_locale("naruto.jutsus", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_JUTSUS).to_string())
}

/// Generate a random Naruto eye technique
pub fn eye() -> String {
    fetch_locale("naruto.eyes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_EYES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Naruto Uzumaki",
    "Sasuke Uchiha",
    "Sakura Haruno",
    "Kakashi Hatake",
    "Itachi Uchiha",
    "Gaara",
    "Jiraiya",
    "Orochimaru",
    "Tsunade",
    "Rock Lee",
    "Neji Hyuga",
    "Hinata Hyuga",
    "Shikamaru Nara",
    "Choji Akimichi",
    "Ino Yamanaka",
    "Might Guy",
    "Minato Namikaze",
    "Kushina Uzumaki",
    "Madara Uchiha",
    "Hashirama Senju",
];

const FALLBACK_VILLAGES: &[&str] = &[
    "Konohagakure",
    "Sunagakure",
    "Kirigakure",
    "Kumogakure",
    "Iwagakure",
    "Otogakure",
    "Amegakure",
    "Kusagakure",
];

const FALLBACK_JUTSUS: &[&str] = &[
    "Rasengan",
    "Chidori",
    "Shadow Clone Jutsu",
    "Summoning Jutsu",
    "Fireball Jutsu",
    "Amaterasu",
    "Susanoo",
    "Tsukuyomi",
    "Rasenshuriken",
    "Flying Raijin",
    "Eight Gates",
    "Byakugan",
    "Gentle Fist",
];

const FALLBACK_EYES: &[&str] = &[
    "Sharingan",
    "Mangekyo Sharingan",
    "Rinnegan",
    "Byakugan",
    "Tenseigan",
    "Rinne Sharingan",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_village() {
        assert!(!village().is_empty());
    }

    #[test]
    fn test_jutsu() {
        assert!(!jutsu().is_empty());
    }

    #[test]
    fn test_eye() {
        assert!(!eye().is_empty());
    }
}

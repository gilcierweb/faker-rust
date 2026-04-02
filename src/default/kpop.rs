//! K-pop generator - generates K-pop related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random K-pop group/band name
pub fn group() -> String {
    fetch_locale("kpop.groups", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GROUPS).to_string())
}

/// Generate a random K-pop solo artist
pub fn solo() -> String {
    fetch_locale("kpop.solos", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SOLOS).to_string())
}

/// Generate a random K-pop song name
pub fn song() -> String {
    fetch_locale("kpop.songs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SONGS).to_string())
}

// Fallback data
const FALLBACK_GROUPS: &[&str] = &[
    "BTS", "BLACKPINK", "TWICE", "EXO", "Red Velvet", "NCT", "Stray Kids",
    "SEVENTEEN", "GOT7", "ITZY", "aespa", "ENHYPEN", "TOMORROW X TOGETHER",
    "IVE", "LE SSERAFIM", "NewJeans", "(G)I-DLE", "ATEEZ", "THE BOYZ",
    "CRAVITY", "TREASURE", "NMIXX", "Kep1er", "STAYC", "fromis_9",
];

const FALLBACK_SOLOS: &[&str] = &[
    "IU", "Taeyeon", "Sunmi", "Chung Ha", "Taemin", "Baekhyun", "Kai",
    "Jackson Wang", "Jessi", "Somi", "Kang Daniel", "Park Jimin",
    "Jungkook", "V", "RM", "J-Hope", "Suga", "Jin", "Lisa", "Jennie",
    "Rosé", "Jisoo", "Hwa Sa", "HyunA", "DAWN", "PSY",
];

const FALLBACK_SONGS: &[&str] = &[
    "Dynamite", "Butter", "Boy With Luv", "DNA", "DDU-DU DDU-DU",
    "Kill This Love", "How You Like That", "Pink Venom", "Shut Down",
    "Feel Special", "Fancy", "What is Love?", "Cheer Up", "TT",
    "Love Shot", "Growl", "Ko Ko Bop", "Monster", "Psycho", "Bad Boy",
    "Red Flavor", "Next Level", "Savage", "Anti-Fragile", "After LIKE",
    "Love Dive", "Attention", "Hype Boy", "Tomboy", "Nxde", "Queencard",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        assert!(!group().is_empty());
    }

    #[test]
    fn test_solo() {
        assert!(!solo().is_empty());
    }

    #[test]
    fn test_song() {
        assert!(!song().is_empty());
    }
}

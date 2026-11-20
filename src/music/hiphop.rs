//! Hip-Hop music generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Hip-Hop artist
pub fn artist() -> String {
    fetch_locale("music.hiphop.artist", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ARTISTS).to_string())
}

/// Generate a random Hip-Hop group
pub fn groups() -> String {
    fetch_locale("music.hiphop.groups", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GROUPS).to_string())
}

/// Generate a random Hip-Hop subgenre
pub fn subgenre() -> String {
    fetch_locale(
        "music.hiphop.subgenres",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_SUBGENRES).to_string())
}

// Fallback data
const FALLBACK_ARTISTS: &[&str] = &[
    "Tupac",
    "The Notorious B.I.G.",
    "Jay-Z",
    "Nas",
    "Eminem",
    "Snoop Dogg",
    "Dr. Dre",
    "Ice Cube",
    "Kendrick Lamar",
    "J. Cole",
    "Drake",
    "Kanye West",
    "Travis Scott",
    "Killer Mike",
];

const FALLBACK_GROUPS: &[&str] = &[
    "Run-DMC",
    "N.W.A",
    "Public Enemy",
    "A Tribe Called Quest",
    "Wu-Tang Clan",
    "OutKast",
    "Beastie Boys",
    "De La Soul",
    "The Roots",
    "Migos",
];

const FALLBACK_SUBGENRES: &[&str] = &[
    "East Coast",
    "West Coast",
    "Southern",
    "Midwest",
    "Gangsta Rap",
    "Conscious Hip Hop",
    "Trap",
    "Boom Bap",
    "Alternative Hip Hop",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `music.hiphop.subgenres`.
pub fn subgenres() -> String {
    parity_pick("music.hiphop.subgenres")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist() {
        assert!(!artist().is_empty());
    }

    #[test]
    fn test_groups() {
        assert!(!groups().is_empty());
    }

    #[test]
    fn test_subgenre() {
        assert!(!subgenre().is_empty());
    }
}

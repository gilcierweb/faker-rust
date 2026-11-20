//! Volleyball generator - generates volleyball data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random volleyball team
pub fn team() -> String {
    fetch_locale("volleyball.team", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TEAMS).to_string())
}

/// Generate a random volleyball player
pub fn player() -> String {
    fetch_locale("volleyball.player", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLAYERS).to_string())
}

/// Generate a random volleyball position
pub fn position() -> String {
    fetch_locale(
        "volleyball.positions",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_POSITIONS).to_string())
}

// Fallback data
const FALLBACK_TEAMS: &[&str] = &[
    "Brazil",
    "Italy",
    "Poland",
    "Russia",
    "United States",
    "Argentina",
    "France",
    "Japan",
    "Serbia",
    "Cuba",
    "Iran",
    "Germany",
    "Netherlands",
    "Turkey",
    "China",
];

const FALLBACK_PLAYERS: &[&str] = &[
    "Giba",
    "Ivan Zaytsev",
    "Wilfredo León",
    "Earvin N'Gapeth",
    "Maxim Mikhaylov",
    "Bruno Rezende",
    "Wallace de Souza",
    "Saeid Marouf",
    "Lucas Saatkamp",
    "Michał Kubiak",
    "Bartosz Kurek",
    "Yūki Ishikawa",
    "Matt Anderson",
    "Aaron Russell",
];

const FALLBACK_POSITIONS: &[&str] = &[
    "Setter",
    "Outside Hitter",
    "Opposite Hitter",
    "Middle Blocker",
    "Libero",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `volleyball.coach`.
pub fn coach() -> String {
    parity_pick("volleyball.coach")
}

/// Gem-compatible generator: `volleyball.formation`.
pub fn formation() -> String {
    parity_pick("volleyball.formation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team() {
        assert!(!team().is_empty());
    }

    #[test]
    fn test_player() {
        assert!(!player().is_empty());
    }

    #[test]
    fn test_position() {
        assert!(!position().is_empty());
    }
}

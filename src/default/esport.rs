//! Esport generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random esports player
pub fn player() -> String {
    fetch_locale("esport.players", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLAYERS).to_string())
}

/// Generate a random esports team
pub fn team() -> String {
    fetch_locale("esport.teams", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TEAMS).to_string())
}

/// Generate a random esports event
pub fn event() -> String {
    fetch_locale("esport.events", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_EVENTS).to_string())
}

/// Generate a random esports league
pub fn league() -> String {
    fetch_locale("esport.leagues", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LEAGUES).to_string())
}

/// Generate a random esports game
pub fn game() -> String {
    fetch_locale("esport.games", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_GAMES).to_string())
}

// Fallback data
const FALLBACK_PLAYERS: &[&str] = &[
    "Faker", "s1mple", "Zywoo", "Niko", "Dev1ce", "Coldzera", "KennyS",
    "GuardiaN", "olofmeister", "Get_RiGhT", "Flusha", "PashaBiceps",
    "Dendi", "Arteezy", "SumaiL", "S1mple", "Shroud", "Ninja",
];

const FALLBACK_TEAMS: &[&str] = &[
    "Fnatic", "SK Gaming", "Astralis", "Natus Vincere", "FaZe Clan",
    "Cloud9", "TSM", "Team Liquid", "Evil Geniuses", "OG",
    "Team Secret", "Virtus.pro", "G2 Esports", "100 Thieves",
];

const FALLBACK_EVENTS: &[&str] = &[
    "The International", "Worlds", "ESL One", "BLAST Premier", "PGL Major",
    "DreamHack", "IEM", "LCS Finals", "LEC Finals", "Capcom Cup",
];

const FALLBACK_LEAGUES: &[&str] = &[
    "LCK", "LCS", "LEC", "LPL", "PCS", "VCS", "LJL", "LCL",
    "LMS", "TCL", "CBLOL", "LLA",
];

const FALLBACK_GAMES: &[&str] = &[
    "League of Legends", "Dota 2", "CS:GO", "Valorant", "Overwatch",
    "Fortnite", "PUBG", "Rainbow Six Siege", "Rocket League", "Street Fighter",
    "Tekken", "Super Smash Bros", "StarCraft II", "Hearthstone",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player() {
        assert!(!player().is_empty());
    }

    #[test]
    fn test_team() {
        assert!(!team().is_empty());
    }

    #[test]
    fn test_event() {
        assert!(!event().is_empty());
    }

    #[test]
    fn test_league() {
        assert!(!league().is_empty());
    }

    #[test]
    fn test_game() {
        assert!(!game().is_empty());
    }
}

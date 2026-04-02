//! Football (Soccer) generator - generates football data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random football team
pub fn team() -> String {
    fetch_locale("football.teams", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TEAMS).to_string())
}

/// Generate a random football player
pub fn player() -> String {
    fetch_locale("football.players", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLAYERS).to_string())
}

/// Generate a random football position
pub fn position() -> String {
    fetch_locale("football.positions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_POSITIONS).to_string())
}

/// Generate a random football competition/league
pub fn competition() -> String {
    fetch_locale("football.competitions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COMPETITIONS).to_string())
}

// Fallback data
const FALLBACK_TEAMS: &[&str] = &[
    "Real Madrid",
    "Barcelona",
    "Manchester United",
    "Liverpool",
    "Bayern Munich",
    "Juventus",
    "Paris Saint-Germain",
    "Chelsea",
    "Arsenal",
    "Manchester City",
    "AC Milan",
    "Inter Milan",
    "Ajax",
    "Porto",
    "Boca Juniors",
    "River Plate",
    "Flamengo",
    "Santos",
];

const FALLBACK_PLAYERS: &[&str] = &[
    "Lionel Messi",
    "Cristiano Ronaldo",
    "Neymar",
    "Kylian Mbappé",
    "Robert Lewandowski",
    "Kevin De Bruyne",
    "Mohamed Salah",
    "Virgil van Dijk",
    "Sergio Ramos",
    "Manuel Neuer",
    "Luka Modrić",
    "Karim Benzema",
    "Sadio Mané",
    "Erling Haaland",
];

const FALLBACK_POSITIONS: &[&str] = &[
    "Goalkeeper",
    "Defender",
    "Midfielder",
    "Forward",
    "Striker",
    "Winger",
    "Center Back",
    "Full Back",
    "Defensive Midfielder",
    "Attacking Midfielder",
];

const FALLBACK_COMPETITIONS: &[&str] = &[
    "FIFA World Cup",
    "UEFA Champions League",
    "Premier League",
    "La Liga",
    "Serie A",
    "Bundesliga",
    "Ligue 1",
    "Copa Libertadores",
    "UEFA Europa League",
    "FA Cup",
];

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

    #[test]
    fn test_competition() {
        assert!(!competition().is_empty());
    }
}

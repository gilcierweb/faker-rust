//! Basketball generator - generates basketball data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random basketball team
pub fn team() -> String {
    fetch_locale("basketball.teams", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TEAMS).to_string())
}

/// Generate a random basketball player
pub fn player() -> String {
    fetch_locale("basketball.players", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLAYERS).to_string())
}

/// Generate a random basketball position
pub fn position() -> String {
    fetch_locale("basketball.positions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_POSITIONS).to_string())
}

/// Generate a random basketball coach
pub fn coach() -> String {
    fetch_locale("basketball.coaches", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COACHES).to_string())
}

// Fallback data
const FALLBACK_TEAMS: &[&str] = &[
    "Los Angeles Lakers",
    "Boston Celtics",
    "Chicago Bulls",
    "Golden State Warriors",
    "Miami Heat",
    "New York Knicks",
    "San Antonio Spurs",
    "Dallas Mavericks",
    "Houston Rockets",
    "Toronto Raptors",
    "Milwaukee Bucks",
    "Phoenix Suns",
    "Brooklyn Nets",
    "Philadelphia 76ers",
    "Denver Nuggets",
];

const FALLBACK_PLAYERS: &[&str] = &[
    "Michael Jordan",
    "LeBron James",
    "Kobe Bryant",
    "Shaquille O'Neal",
    "Magic Johnson",
    "Larry Bird",
    "Kareem Abdul-Jabbar",
    "Tim Duncan",
    "Kevin Durant",
    "Stephen Curry",
    "Giannis Antetokounmpo",
    "Luka Dončić",
    "Nikola Jokić",
    "Joel Embiid",
    "Kawhi Leonard",
];

const FALLBACK_POSITIONS: &[&str] = &[
    "Point Guard",
    "Shooting Guard",
    "Small Forward",
    "Power Forward",
    "Center",
];

const FALLBACK_COACHES: &[&str] = &[
    "Phil Jackson",
    "Gregg Popovich",
    "Pat Riley",
    "Red Auerbach",
    "Steve Kerr",
    "Erik Spoelstra",
    "Doc Rivers",
    "Mike Budenholzer",
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
    fn test_coach() {
        assert!(!coach().is_empty());
    }
}

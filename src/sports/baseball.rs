//! Baseball generator - generates baseball data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random baseball team
pub fn team() -> String {
    fetch_locale("baseball.teams", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TEAMS).to_string())
}

/// Generate a random baseball player
pub fn player() -> String {
    fetch_locale("baseball.players", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLAYERS).to_string())
}

/// Generate a random baseball position
pub fn position() -> String {
    fetch_locale("baseball.positions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_POSITIONS).to_string())
}

/// Generate a random baseball stadium
pub fn stadium() -> String {
    fetch_locale("baseball.stadiums", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STADIUMS).to_string())
}

// Fallback data
const FALLBACK_TEAMS: &[&str] = &[
    "New York Yankees",
    "Boston Red Sox",
    "Los Angeles Dodgers",
    "Chicago Cubs",
    "San Francisco Giants",
    "St. Louis Cardinals",
    "New York Mets",
    "Houston Astros",
    "Atlanta Braves",
    "Philadelphia Phillies",
    "Toronto Blue Jays",
    "Chicago White Sox",
    "Cleveland Guardians",
    "Detroit Tigers",
    "Texas Rangers",
];

const FALLBACK_PLAYERS: &[&str] = &[
    "Babe Ruth",
    "Hank Aaron",
    "Jackie Robinson",
    "Derek Jeter",
    "Mike Trout",
    "Shohei Ohtani",
    "Aaron Judge",
    "Mookie Betts",
    "Freddie Freeman",
    "Juan Soto",
    "Ronald Acuña Jr.",
    "Fernando Tatis Jr.",
    "Bryce Harper",
    "Manny Machado",
];

const FALLBACK_POSITIONS: &[&str] = &[
    "Pitcher",
    "Catcher",
    "First Baseman",
    "Second Baseman",
    "Third Baseman",
    "Shortstop",
    "Left Fielder",
    "Center Fielder",
    "Right Fielder",
    "Designated Hitter",
];

const FALLBACK_STADIUMS: &[&str] = &[
    "Yankee Stadium",
    "Fenway Park",
    "Wrigley Field",
    "Dodger Stadium",
    "Oracle Park",
    "Busch Stadium",
    "Citi Field",
    "Truist Park",
    "Minute Maid Park",
    "T-Mobile Park",
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
    fn test_stadium() {
        assert!(!stadium().is_empty());
    }
}

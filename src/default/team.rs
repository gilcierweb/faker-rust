//! Team generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random team name
pub fn name() -> String {
    fetch_locale("team.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random team mascot
pub fn mascot() -> String {
    fetch_locale("team.mascots", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MASCOTS).to_string())
}

/// Generate a random team sport
pub fn sport() -> String {
    fetch_locale("team.sports", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SPORTS).to_string())
}

/// Generate a random team state
pub fn state() -> String {
    fetch_locale("team.states", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STATES).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Thunder", "Lightning", "Storm", "Wolves", "Bears", "Eagles", "Hawks",
    "Lions", "Tigers", "Panthers", "Sharks", "Dragons", "Knights", "Warriors",
    "Titans", "Giants", "Raiders", "Chargers", "Jets", "Rockets", "Stars",
];

const FALLBACK_MASCOTS: &[&str] = &[
    "Wildcats", "Bulldogs", "Tigers", "Eagles", "Lions", "Bears", "Sharks",
    "Dragons", "Knights", "Vikings", "Pirates", "Cowboys", "Spartans",
    "Trojans", "Rangers", "Rebels", "Patriots", "Braves", "Chiefs",
];

const FALLBACK_SPORTS: &[&str] = &[
    "Football", "Basketball", "Baseball", "Hockey", "Soccer", "Volleyball",
    "Tennis", "Golf", "Swimming", "Track", "Wrestling", "Lacrosse", "Rugby",
];

const FALLBACK_STATES: &[&str] = &[
    "New York", "California", "Texas", "Florida", "Illinois", "Pennsylvania",
    "Ohio", "Georgia", "North Carolina", "Michigan", "New Jersey", "Virginia",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_mascot() {
        assert!(!mascot().is_empty());
    }

    #[test]
    fn test_sport() {
        assert!(!sport().is_empty());
    }

    #[test]
    fn test_state() {
        assert!(!state().is_empty());
    }
}

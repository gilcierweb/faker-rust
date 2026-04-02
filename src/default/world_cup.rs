//! World Cup generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random FIFA World Cup year
pub fn year() -> String {
    fetch_locale("world_cup.years", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_YEARS).to_string())
}

/// Generate a random World Cup host
pub fn host() -> String {
    fetch_locale("world_cup.hosts", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HOSTS).to_string())
}

/// Generate a random World Cup winner
pub fn winner() -> String {
    fetch_locale("world_cup.winners", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_WINNERS).to_string())
}

/// Generate a random World Cup stadium
pub fn stadium() -> String {
    fetch_locale("world_cup.stadiums", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STADIUMS).to_string())
}

/// Generate a random World Cup team
pub fn team() -> String {
    let teams = [
        "Brazil", "Germany", "Italy", "Argentina", "France", "Spain",
        "England", "Netherlands", "Uruguay", "Portugal", "Belgium",
        "Croatia", "Mexico", "USA", "Japan", "South Korea",
    ];
    sample(&teams).to_string()
}

// Fallback data
const FALLBACK_YEARS: &[&str] = &[
    "2022", "2018", "2014", "2010", "2006", "2002", "1998", "1994",
    "1990", "1986", "1982", "1978", "1974", "1970", "1966", "1962",
];

const FALLBACK_HOSTS: &[&str] = &[
    "Qatar", "Russia", "Brazil", "South Africa", "Germany", "Japan/Korea",
    "France", "USA", "Italy", "Mexico", "Argentina", "Spain",
];

const FALLBACK_WINNERS: &[&str] = &[
    "Argentina", "France", "Germany", "Brazil", "Spain", "Italy",
    "England", "Uruguay", "France", "Argentina", "Germany",
];

const FALLBACK_STADIUMS: &[&str] = &[
    "Lusail Stadium", "Al Bayt Stadium", "Khalifa International Stadium",
    "Al Janoub Stadium", "Education City Stadium", "Stadium 974",
    "Al Thumama Stadium", "Ahmad Bin Ali Stadium",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year() {
        assert!(!year().is_empty());
    }

    #[test]
    fn test_host() {
        assert!(!host().is_empty());
    }

    #[test]
    fn test_winner() {
        assert!(!winner().is_empty());
    }

    #[test]
    fn test_stadium() {
        assert!(!stadium().is_empty());
    }

    #[test]
    fn test_team() {
        assert!(!team().is_empty());
    }
}

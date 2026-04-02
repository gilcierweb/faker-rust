//! World Cup generator - generates World Cup data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random FIFA World Cup year
pub fn year() -> String {
    fetch_locale("world_cup.years", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_YEARS).to_string())
}

/// Generate a random FIFA World Cup host
pub fn host() -> String {
    fetch_locale("world_cup.hosts", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HOSTS).to_string())
}

/// Generate a random FIFA World Cup winner
pub fn winner() -> String {
    fetch_locale("world_cup.winners", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_WINNERS).to_string())
}

/// Generate a random FIFA World Cup stadium
pub fn stadium() -> String {
    fetch_locale("world_cup.stadiums", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STADIUMS).to_string())
}

// Fallback data
const FALLBACK_YEARS: &[&str] = &[
    "1930", "1934", "1938", "1950", "1954", "1958", "1962", "1966",
    "1970", "1974", "1978", "1982", "1986", "1990", "1994", "1998",
    "2002", "2006", "2010", "2014", "2018", "2022", "2026",
];

const FALLBACK_HOSTS: &[&str] = &[
    "Uruguay",
    "Italy",
    "France",
    "Brazil",
    "Switzerland",
    "Sweden",
    "Chile",
    "England",
    "Mexico",
    "West Germany",
    "Argentina",
    "Spain",
    "United States",
    "South Korea & Japan",
    "Germany",
    "South Africa",
    "Russia",
    "Qatar",
];

const FALLBACK_WINNERS: &[&str] = &[
    "Uruguay",
    "Italy",
    "Germany",
    "Brazil",
    "England",
    "Argentina",
    "France",
    "Spain",
    "Argentina",
];

const FALLBACK_STADIUMS: &[&str] = &[
    "Maracanã",
    "Wembley",
    "Estadio Azteca",
    "Camp Nou",
    "Santiago Bernabéu",
    "San Siro",
    "Allianz Arena",
    "Lusail Stadium",
    "Rose Bowl",
    "Olympiastadion",
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
}

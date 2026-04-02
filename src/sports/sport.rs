//! Sports generator - generates various sports data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random sport name
pub fn name() -> String {
    fetch_locale("sport.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SPORTS).to_string())
}

/// Generate a random summer olympic sport
pub fn summer_olympics() -> String {
    fetch_locale("sport.summer_olympics", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SUMMER_OLYMPICS).to_string())
}

/// Generate a random winter olympic sport
pub fn winter_olympics() -> String {
    fetch_locale("sport.winter_olympics", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_WINTER_OLYMPICS).to_string())
}

// Fallback data
const FALLBACK_SPORTS: &[&str] = &[
    "Football",
    "Basketball",
    "Baseball",
    "Soccer",
    "Tennis",
    "Golf",
    "Swimming",
    "Running",
    "Cycling",
    "Boxing",
    "Hockey",
    "Volleyball",
    "Rugby",
    "Cricket",
];

const FALLBACK_SUMMER_OLYMPICS: &[&str] = &[
    "Athletics",
    "Swimming",
    "Gymnastics",
    "Tennis",
    "Basketball",
    "Football",
    "Volleyball",
    "Cycling",
    "Boxing",
    "Wrestling",
    "Rowing",
    "Sailing",
];

const FALLBACK_WINTER_OLYMPICS: &[&str] = &[
    "Skiing",
    "Snowboarding",
    "Ice Hockey",
    "Figure Skating",
    "Speed Skating",
    "Bobsleigh",
    "Luge",
    "Skeleton",
    "Curling",
    "Biathlon",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_summer_olympics() {
        assert!(!summer_olympics().is_empty());
    }

    #[test]
    fn test_winter_olympics() {
        assert!(!winter_olympics().is_empty());
    }
}

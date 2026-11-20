//! Sports generator - generates various sports data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random sport name
pub fn name() -> String {
    fetch_locale("sport.names", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SPORTS).to_string())
}

/// Generate a random summer olympic sport
pub fn summer_olympics() -> String {
    fetch_locale(
        "sport.summer_olympics",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_SUMMER_OLYMPICS).to_string())
}

/// Generate a random winter olympic sport
pub fn winter_olympics() -> String {
    fetch_locale(
        "sport.winter_olympics",
        &crate::config::FakerConfig::locale(),
    )
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

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `sport.ancient_olympics`.
pub fn ancient_olympics_sport() -> String {
    parity_pick("sport.ancient_olympics")
}

/// Gem-compatible generator: `sport.summer_olympics`.
pub fn summer_olympics_sport() -> String {
    parity_pick("sport.summer_olympics")
}

/// Gem-compatible generator: `sport.summer_paralympics`.
pub fn summer_paralympics_sport() -> String {
    parity_pick("sport.summer_paralympics")
}

/// Gem-compatible generator: `sport.unusual`.
pub fn unusual_sport() -> String {
    parity_pick("sport.unusual")
}

/// Gem-compatible generator: `sport.winter_olympics`.
pub fn winter_olympics_sport() -> String {
    parity_pick("sport.winter_olympics")
}

/// Gem-compatible generator: `sport.winter_paralympics`.
pub fn winter_paralympics_sport() -> String {
    parity_pick("sport.winter_paralympics")
}
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: sport name with optional ancient/unusual sources.
pub fn sport() -> String {
    crate::base::sample(&[
        crate::sports::sport::summer_olympics_sport(),
        crate::sports::sport::winter_olympics_sport(),
    ])
    .to_string()
}

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

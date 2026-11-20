//! Space generator - generates random planets, moons, galaxies, and constellations

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random planet
pub fn planet() -> String {
    fetch_locale_with_context(
        "space.planet",
        &crate::config::FakerConfig::locale(),
        Some("space"),
    )
    .map(|v| sample_with_resolve(&v, Some("space")))
    .unwrap_or_else(|| sample(FALLBACK_PLANETS).to_string())
}

/// Generate a random moon
pub fn moon() -> String {
    fetch_locale_with_context(
        "space.moon",
        &crate::config::FakerConfig::locale(),
        Some("space"),
    )
    .map(|v| sample_with_resolve(&v, Some("space")))
    .unwrap_or_else(|| sample(FALLBACK_MOONS).to_string())
}

/// Generate a random galaxy
pub fn galaxy() -> String {
    fetch_locale_with_context(
        "space.galaxy",
        &crate::config::FakerConfig::locale(),
        Some("space"),
    )
    .map(|v| sample_with_resolve(&v, Some("space")))
    .unwrap_or_else(|| sample(FALLBACK_GALAXIES).to_string())
}

/// Generate a random constellation
pub fn constellation() -> String {
    fetch_locale_with_context(
        "space.constellation",
        &crate::config::FakerConfig::locale(),
        Some("space"),
    )
    .map(|v| sample_with_resolve(&v, Some("space")))
    .unwrap_or_else(|| sample(FALLBACK_CONSTELLATIONS).to_string())
}

// Fallback data
const FALLBACK_PLANETS: &[&str] = &[
    "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
];
const FALLBACK_MOONS: &[&str] = &[
    "Moon", "Phobos", "Deimos", "Io", "Europa", "Ganymede", "Callisto",
];
const FALLBACK_GALAXIES: &[&str] = &["Andromeda", "Milky Way", "Triangulum", "Sombrero", "Cigar"];
const FALLBACK_CONSTELLATIONS: &[&str] = &[
    "Orion",
    "Ursa Major",
    "Ursa Minor",
    "Canis Major",
    "Cassiopeia",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `space.agency`.
pub fn agency() -> String {
    parity_pick("space.agency")
}

/// Gem-compatible generator: `space.agency_abv`.
pub fn agency_abv() -> String {
    parity_pick("space.agency_abv")
}

/// Gem-compatible generator: `space.company`.
pub fn company() -> String {
    parity_pick("space.company")
}

/// Gem-compatible generator: `space.distance_measurement`.
pub fn distance_measurement() -> String {
    parity_pick("space.distance_measurement")
}

/// Gem-compatible generator: `space.launch_vehicle`.
pub fn launch_vehicle() -> String {
    parity_pick("space.launch_vehicle")
}

/// Gem-compatible generator: `space.meteorite`.
pub fn meteorite() -> String {
    parity_pick("space.meteorite")
}

/// Gem-compatible generator: `space.nasa_space_craft`.
pub fn nasa_space_craft() -> String {
    parity_pick("space.nasa_space_craft")
}

/// Gem-compatible generator: `space.nebula`.
pub fn nebula() -> String {
    parity_pick("space.nebula")
}

/// Gem-compatible generator: `space.star`.
pub fn star() -> String {
    parity_pick("space.star")
}

/// Gem-compatible generator: `space.star_cluster`.
pub fn star_cluster() -> String {
    parity_pick("space.star_cluster")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet() {
        assert!(!planet().is_empty());
    }

    #[test]
    fn test_moon() {
        assert!(!moon().is_empty());
    }
}

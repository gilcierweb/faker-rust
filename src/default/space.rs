//! Space generator - generates random planets, moons, galaxies, and constellations

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random planet
pub fn planet() -> String {
    fetch_locale_with_context("space.planet", "en", Some("space"))
        .map(|v| sample_with_resolve(&v, Some("space")))
        .unwrap_or_else(|| sample(FALLBACK_PLANETS).to_string())
}

/// Generate a random moon
pub fn moon() -> String {
    fetch_locale_with_context("space.moon", "en", Some("space"))
        .map(|v| sample_with_resolve(&v, Some("space")))
        .unwrap_or_else(|| sample(FALLBACK_MOONS).to_string())
}

/// Generate a random galaxy
pub fn galaxy() -> String {
    fetch_locale_with_context("space.galaxy", "en", Some("space"))
        .map(|v| sample_with_resolve(&v, Some("space")))
        .unwrap_or_else(|| sample(FALLBACK_GALAXIES).to_string())
}

/// Generate a random constellation
pub fn constellation() -> String {
    fetch_locale_with_context("space.constellation", "en", Some("space"))
        .map(|v| sample_with_resolve(&v, Some("space")))
        .unwrap_or_else(|| sample(FALLBACK_CONSTELLATIONS).to_string())
}

// Fallback data
const FALLBACK_PLANETS: &[&str] = &["Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"];
const FALLBACK_MOONS: &[&str] = &["Moon", "Phobos", "Deimos", "Io", "Europa", "Ganymede", "Callisto"];
const FALLBACK_GALAXIES: &[&str] = &["Andromeda", "Milky Way", "Triangulum", "Sombrero", "Cigar"];
const FALLBACK_CONSTELLATIONS: &[&str] = &["Orion", "Ursa Major", "Ursa Minor", "Canis Major", "Cassiopeia"];

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

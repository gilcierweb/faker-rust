//! Drone generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random drone manufacturer
pub fn manufacturer() -> String {
    fetch_locale("drone.manufacturers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MANUFACTURERS).to_string())
}

/// Generate a random drone name
pub fn name() -> String {
    fetch_locale("drone.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random drone category
pub fn category() -> String {
    fetch_locale("drone.categories", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CATEGORIES).to_string())
}

// Fallback data
const FALLBACK_MANUFACTURERS: &[&str] = &[
    "DJI", "Parrot", "Autel Robotics", "Skydio", "Yuneec", "Holy Stone",
    "Ryze", "Hubsan", "Syma", "Potensic", "Snaptain", "DEERC",
];

const FALLBACK_NAMES: &[&str] = &[
    "Mavic Air", "Mavic Pro", "Phantom", "Inspire", "Mini", "Spark",
    "Anafi", "Bebop", "EVO", "X-Star", "Typhoon", "Tello",
];

const FALLBACK_CATEGORIES: &[&str] = &[
    "Consumer", "Professional", "Racing", "Toy", "Industrial", "Military",
    "Agricultural", "Delivery", "Surveillance", "Photography",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manufacturer() {
        assert!(!manufacturer().is_empty());
    }

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_category() {
        assert!(!category().is_empty());
    }
}

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
    "DJI",
    "Parrot",
    "Autel Robotics",
    "Skydio",
    "Yuneec",
    "Holy Stone",
    "Ryze",
    "Hubsan",
    "Syma",
    "Potensic",
    "Snaptain",
    "DEERC",
];

const FALLBACK_NAMES: &[&str] = &[
    "Mavic Air",
    "Mavic Pro",
    "Phantom",
    "Inspire",
    "Mini",
    "Spark",
    "Anafi",
    "Bebop",
    "EVO",
    "X-Star",
    "Typhoon",
    "Tello",
];

const FALLBACK_CATEGORIES: &[&str] = &[
    "Consumer",
    "Professional",
    "Racing",
    "Toy",
    "Industrial",
    "Military",
    "Agricultural",
    "Delivery",
    "Surveillance",
    "Photography",
];

// --- gem parity generators (locale-backed) ---

fn pick(key: &str) -> String {
    fetch_locale(key, "en")
        .map(|v| sample(&v))
        .unwrap_or_default()
}

fn parsed(key: &str) -> String {
    crate::base::numerify(&pick(key))
}

/// Gem-compatible generator: drone weight.
pub fn weight() -> String {
    parsed("drone.weight")
}

/// Gem-compatible generator: maximum ascent speed.
pub fn max_ascent_speed() -> String {
    parsed("drone.max_ascent_speed")
}

/// Gem-compatible generator: maximum descent speed.
pub fn max_descent_speed() -> String {
    parsed("drone.max_descent_speed")
}

/// Gem-compatible generator: flight time.
pub fn flight_time() -> String {
    parsed("drone.flight_time")
}

/// Gem-compatible generator: maximum altitude.
pub fn max_altitude() -> String {
    parsed("drone.max_altitude")
}

/// Gem-compatible generator: maximum flight distance.
pub fn max_flight_distance() -> String {
    parsed("drone.max_flight_distance")
}

/// Gem-compatible generator: maximum speed.
pub fn max_speed() -> String {
    parsed("drone.max_speed")
}

/// Gem-compatible generator: maximum wind resistance.
pub fn max_wind_resistance() -> String {
    parsed("drone.max_wind_resistance")
}

/// Gem-compatible generator: maximum angular velocity.
pub fn max_angular_velocity() -> String {
    parsed("drone.max_angular_velocity")
}

/// Gem-compatible generator: maximum tilt angle.
pub fn max_tilt_angle() -> String {
    parsed("drone.max_tilt_angle")
}

/// Gem-compatible generator: operating temperature.
pub fn operating_temperature() -> String {
    parsed("drone.operating_temperature")
}

/// Gem-compatible generator: battery capacity.
pub fn battery_capacity() -> String {
    parsed("drone.battery_capacity")
}

/// Gem-compatible generator: battery voltage.
pub fn battery_voltage() -> String {
    parsed("drone.battery_voltage")
}

/// Gem-compatible generator: battery type.
pub fn battery_type() -> String {
    pick("drone.battery_type")
}

/// Gem-compatible generator: battery weight.
pub fn battery_weight() -> String {
    parsed("drone.battery_weight")
}

/// Gem-compatible generator: charging temperature.
pub fn charging_temperature() -> String {
    parsed("drone.charging_temperature")
}

/// Gem-compatible generator: maximum charging power.
pub fn max_charging_power() -> String {
    parsed("drone.max_charging_power")
}

/// Gem-compatible generator: ISO range.
pub fn iso() -> String {
    pick("drone.iso")
}

/// Gem-compatible generator: maximum camera resolution.
pub fn max_resolution() -> String {
    parsed("drone.max_resolution")
}

/// Gem-compatible generator: photo format.
pub fn photo_format() -> String {
    pick("drone.photo_format")
}

/// Gem-compatible generator: video format.
pub fn video_format() -> String {
    pick("drone.video_format")
}

/// Gem-compatible generator: shutter speed range.
pub fn shutter_speed_range() -> String {
    format!(
        "{}-{}{}",
        pick("drone.max_shutter_speed"),
        pick("drone.min_shutter_speed"),
        pick("drone.shutter_speed_units")
    )
}

/// Gem-compatible generator: maximum shutter speed.
pub fn max_shutter_speed() -> String {
    pick("drone.max_shutter_speed")
}

/// Gem-compatible generator: minimum shutter speed.
pub fn min_shutter_speed() -> String {
    pick("drone.min_shutter_speed")
}

/// Alias matching the typo used in the gem's generated documentation.
pub use charging_temperature as charging_temprature;
/// Alias matching the typo used in the gem's generated documentation.
pub use operating_temperature as operating_temprature;

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

//! Vehicle generator - generates random vehicle info

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random vehicle manufacturer
pub fn manufacturer() -> String {
    fetch_locale("vehicle.manufacturer", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Toyota".to_string())
}

/// Generate a random vehicle make
pub fn make() -> String {
    fetch_locale("vehicle.makes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ford".to_string())
}

/// Generate a random vehicle color
pub fn color() -> String {
    fetch_locale("vehicle.colors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Red".to_string())
}

/// Generate a random transmission type
pub fn transmission() -> String {
    fetch_locale("vehicle.transmissions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Automatic".to_string())
}

/// Generate a random drive type
pub fn drive_type() -> String {
    fetch_locale("vehicle.drive_types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "FWD".to_string())
}

/// Generate a random fuel type
pub fn fuel_type() -> String {
    fetch_locale("vehicle.fuel_types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Gasoline".to_string())
}

/// Generate a random vehicle style
pub fn style() -> String {
    fetch_locale("vehicle.styles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sedan".to_string())
}

/// Generate a random car type
pub fn car_type() -> String {
    fetch_locale("vehicle.car_types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Coupe".to_string())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `vehicle.car_options`.
pub fn car_options() -> String {
    parity_pick("vehicle.car_options")
}

/// Gem-compatible generator: `vehicle.doors`.
pub fn doors() -> String {
    parity_pick("vehicle.doors")
}

/// Gem-compatible generator: `vehicle.cylinder_engine`.
pub fn engine() -> String {
    parity_pick("vehicle.cylinder_engine")
}

/// Gem-compatible generator: `vehicle.makes`.
pub fn make_and_model() -> String {
    parity_pick("vehicle.makes")
}

/// Gem-compatible generator: `vehicle.license_plate`.
pub fn singapore_license_plate() -> String {
    parity_pick("vehicle.license_plate")
}

/// Gem-compatible generator: `vehicle.standard_specs`.
pub fn standard_specs() -> String {
    parity_pick("vehicle.standard_specs")
}

/// Gem-compatible generator: `vehicle.version`.
pub fn version() -> String {
    parity_pick("vehicle.version")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vehicle_fields_are_nonempty() {
        for value in [
            manufacturer(),
            make(),
            color(),
            transmission(),
            drive_type(),
            fuel_type(),
            style(),
            car_type(),
        ] {
            assert!(!value.is_empty());
            assert!(!value.contains("#{"), "unresolved: {value}");
        }
    }
}

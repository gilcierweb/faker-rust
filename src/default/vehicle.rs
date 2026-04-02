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

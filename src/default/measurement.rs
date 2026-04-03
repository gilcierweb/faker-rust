//! Measurement generator

use crate::base::sample;
use crate::config::FakerConfig;

/// Generate a random length measurement
pub fn length() -> String {
    let config = FakerConfig::current();
    let value = config.rand_range(1, 100);
    let units = ["meters", "feet", "inches", "centimeters", "millimeters", "kilometers", "miles"];
    let unit = sample(&units);
    format!("{} {}", value, unit)
}

/// Generate a random weight measurement
pub fn weight() -> String {
    let config = FakerConfig::current();
    let value = config.rand_range(1, 100);
    let units = ["kilograms", "grams", "pounds", "ounces", "tons"];
    let unit = sample(&units);
    format!("{} {}", value, unit)
}

/// Generate a random volume measurement
pub fn volume() -> String {
    let config = FakerConfig::current();
    let value = config.rand_range(1, 100);
    let units = ["liters", "milliliters", "gallons", "cups", "fluid ounces"];
    let unit = sample(&units);
    format!("{} {}", value, unit)
}

/// Generate a random temperature
pub fn temperature() -> String {
    let config = FakerConfig::current();
    let value = config.rand_range_i64(-20, 110);
    let units = ["Celsius", "Fahrenheit", "Kelvin"];
    let unit = sample(&units);
    format!("{} {}", value, unit)
}

/// Generate a random metric height
pub fn metric_height() -> String {
    let config = FakerConfig::current();
    let cm = config.rand_range(150, 200);
    format!("{} cm", cm)
}

/// Generate a random imperial height
pub fn imperial_height() -> String {
    let config = FakerConfig::current();
    let feet = config.rand_range(5, 7);
    let inches = config.rand_range(0, 12);
    format!("{}'{}\"", feet, inches)
}

/// Generate a random metric weight
pub fn metric_weight() -> String {
    let config = FakerConfig::current();
    let kg = config.rand_range(50, 120);
    format!("{} kg", kg)
}

/// Generate a random imperial weight
pub fn imperial_weight() -> String {
    let config = FakerConfig::current();
    let lbs = config.rand_range(100, 250);
    format!("{} lbs", lbs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert!(!length().is_empty());
    }

    #[test]
    fn test_weight() {
        assert!(!weight().is_empty());
    }

    #[test]
    fn test_volume() {
        assert!(!volume().is_empty());
    }

    #[test]
    fn test_temperature() {
        assert!(!temperature().is_empty());
    }

    #[test]
    fn test_metric_height() {
        let h = metric_height();
        assert!(h.contains("cm"));
    }

    #[test]
    fn test_imperial_height() {
        let h = imperial_height();
        assert!(h.contains('\''));
    }

    #[test]
    fn test_metric_weight() {
        let w = metric_weight();
        assert!(w.contains("kg"));
    }

    #[test]
    fn test_imperial_weight() {
        let w = imperial_weight();
        assert!(w.contains("lbs"));
    }
}

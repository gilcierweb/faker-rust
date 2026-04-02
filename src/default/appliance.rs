//! Appliance generator - generates random appliance-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random appliance brand
pub fn brand() -> String {
    fetch_locale("appliance.brand", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bosch".to_string())
}

/// Generate a random appliance equipment
pub fn equipment() -> String {
    fetch_locale("appliance.equipment", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Refrigerator".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brand() {
        let b = brand();
        assert!(!b.is_empty());
    }

    #[test]
    fn test_equipment() {
        let e = equipment();
        assert!(!e.is_empty());
    }
}

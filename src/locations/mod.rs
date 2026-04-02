//! Locations generator - generates location-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random community type
pub fn community() -> String {
    fetch_locale("locations.communities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COMMUNITIES).to_string())
}

/// Generate a random neighborhood name
pub fn neighborhood() -> String {
    fetch_locale("locations.neighborhoods", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NEIGHBORHOODS).to_string())
}

/// Generate a random building type
pub fn building_type() -> String {
    fetch_locale("locations.building_types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BUILDING_TYPES).to_string())
}

/// Generate a random place
pub fn place() -> String {
    fetch_locale("locations.places", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLACES).to_string())
}

/// Generate a random locale
pub fn locale() -> String {
    fetch_locale("locations.locales", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCALES).to_string())
}

// Fallback data
const FALLBACK_COMMUNITIES: &[&str] = &[
    "City", "Town", "Village", "Hamlet", "Suburb", "District", "Borough",
    "County", "Province", "State", "Territory", "Region", "Zone", "Area",
];

const FALLBACK_NEIGHBORHOODS: &[&str] = &[
    "Downtown", "Uptown", "Midtown", "Eastside", "Westside", "North End",
    "South End", "Old Town", "Historic District", "Financial District",
    "Arts District", "Entertainment District", "Warehouse District",
];

const FALLBACK_BUILDING_TYPES: &[&str] = &[
    "House", "Apartment", "Condo", "Townhouse", "Mansion", "Cottage",
    "Bungalow", "Duplex", "Loft", "Studio", "Penthouse", "Villa",
    "Office Building", "Warehouse", "Factory", "Store", "Restaurant",
];

const FALLBACK_PLACES: &[&str] = &[
    "Park", "Museum", "Library", "School", "Hospital", "Church", "Temple",
    "Mosque", "Synagogue", "Theater", "Cinema", "Stadium", "Arena", "Zoo",
    "Aquarium", "Beach", "Mountain", "Forest", "Lake", "River", "Ocean",
];

const FALLBACK_LOCALES: &[&str] = &[
    "en-US", "en-GB", "en-CA", "en-AU", "es-ES", "es-MX", "fr-FR",
    "de-DE", "it-IT", "pt-BR", "ru-RU", "ja-JP", "zh-CN", "ko-KR",
    "nl-NL", "pl-PL", "sv-SE", "tr-TR", "ar-SA", "hi-IN",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_community() {
        assert!(!community().is_empty());
    }

    #[test]
    fn test_neighborhood() {
        assert!(!neighborhood().is_empty());
    }

    #[test]
    fn test_building_type() {
        assert!(!building_type().is_empty());
    }

    #[test]
    fn test_place() {
        assert!(!place().is_empty());
    }

    #[test]
    fn test_locale() {
        assert!(!locale().is_empty());
    }
}

//! House generator - generates house-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random house furniture
pub fn furniture() -> String {
    fetch_locale("house.furniture", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FURNITURE).to_string())
}

/// Generate a random house room
pub fn room() -> String {
    fetch_locale("house.rooms", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ROOMS).to_string())
}

// Fallback data
const FALLBACK_FURNITURE: &[&str] = &[
    "Sofa", "Armchair", "Coffee Table", "Dining Table", "Bed", "Wardrobe",
    "Dresser", "Nightstand", "Bookshelf", "Desk", "Office Chair", "TV Stand",
    "Cabinet", "Sideboard", "Console Table", "Ottoman", "Bench", "Stool",
    "Bookcase", "Chest of Drawers", "Futon", "Bunk Bed", "Loft Bed", "Headboard",
];

const FALLBACK_ROOMS: &[&str] = &[
    "Living Room", "Bedroom", "Kitchen", "Bathroom", "Dining Room",
    "Office", "Study", "Guest Room", "Nursery", "Playroom", "Laundry Room",
    "Garage", "Basement", "Attic", "Pantry", "Closet", "Hallway",
    "Entryway", "Sunroom", "Library", "Home Theater", "Gym", "Game Room",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furniture() {
        assert!(!furniture().is_empty());
    }

    #[test]
    fn test_room() {
        assert!(!room().is_empty());
    }
}

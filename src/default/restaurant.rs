//! Restaurant generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random restaurant name
pub fn name() -> String {
    fetch_locale("restaurant.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random restaurant type/cuisine
pub fn r#type() -> String {
    fetch_locale("restaurant.types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TYPES).to_string())
}

/// Generate a random restaurant review
pub fn review() -> String {
    fetch_locale("restaurant.reviews", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_REVIEWS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "The Golden Spoon", "Bella Vista", "Ocean's Catch", "Mountain View Grill",
    "Urban Kitchen", "The Corner Bistro", "Sunset Cafe", "Riverside Tavern",
    "Harvest Table", "The Copper Pot", "Blue Plate Special", "Garden Terrace",
];

const FALLBACK_TYPES: &[&str] = &[
    "Italian", "Chinese", "Japanese", "Mexican", "Indian", "Thai",
    "French", "Mediterranean", "American", "BBQ", "Seafood", "Steakhouse",
    "Vegetarian", "Vegan", "Fusion", "Fast Food", "Fine Dining", "Cafe",
];

const FALLBACK_REVIEWS: &[&str] = &[
    "Amazing food and great service!",
    "A hidden gem in the neighborhood.",
    "The atmosphere was perfect for date night.",
    "Best meal I've had in months.",
    "Highly recommend the chef's special.",
    "Will definitely be coming back!",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_type() {
        assert!(!r#type().is_empty());
    }

    #[test]
    fn test_review() {
        assert!(!review().is_empty());
    }
}

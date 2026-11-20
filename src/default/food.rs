//! Food generator - generates random food dishes, ingredients, and descriptions

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random food dish (e.g. "Caesar Salad")
pub fn dish() -> String {
    fetch_locale_with_context(
        "food.dish",
        &crate::config::FakerConfig::locale(),
        Some("food"),
    )
    .map(|v| sample_with_resolve(&v, Some("food")))
    .unwrap_or_else(|| sample(FALLBACK_DISHES).to_string())
}

/// Generate a random food description
pub fn description() -> String {
    fetch_locale_with_context(
        "food.description",
        &crate::config::FakerConfig::locale(),
        Some("food"),
    )
    .map(|v| sample_with_resolve(&v, Some("food")))
    .unwrap_or_else(|| "A delicious meal prepared with fresh ingredients.".to_string())
}

/// Generate a random food ingredient
pub fn ingredient() -> String {
    fetch_locale_with_context(
        "food.ingredient",
        &crate::config::FakerConfig::locale(),
        Some("food"),
    )
    .map(|v| sample_with_resolve(&v, Some("food")))
    .unwrap_or_else(|| sample(FALLBACK_INGREDIENTS).to_string())
}

/// Generate a random fruit
pub fn fruit() -> String {
    fetch_locale_with_context(
        "food.fruits",
        &crate::config::FakerConfig::locale(),
        Some("food"),
    )
    .map(|v| sample_with_resolve(&v, Some("food")))
    .unwrap_or_else(|| sample(FALLBACK_FRUITS).to_string())
}

/// Generate a random vegetable
pub fn vegetable() -> String {
    fetch_locale_with_context(
        "food.vegetables",
        &crate::config::FakerConfig::locale(),
        Some("food"),
    )
    .map(|v| sample_with_resolve(&v, Some("food")))
    .unwrap_or_else(|| sample(FALLBACK_VEGETABLES).to_string())
}

// Fallback data
const FALLBACK_DISHES: &[&str] = &["Pizza", "Burger", "Sushi", "Pasta", "Taco"];
const FALLBACK_INGREDIENTS: &[&str] = &["Tomato", "Cheese", "Onion", "Garlic", "Chicken"];
const FALLBACK_FRUITS: &[&str] = &["Apple", "Banana", "Orange", "Strawberry", "Grape"];
const FALLBACK_VEGETABLES: &[&str] = &["Carrot", "Broccoli", "Spinach", "Potato", "Cucumber"];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `food.allergens`.
pub fn allergen() -> String {
    parity_pick("food.allergens")
}

/// Gem-compatible generator: `food.ethnic_category`.
pub fn ethnic_category() -> String {
    parity_pick("food.ethnic_category")
}

/// Gem-compatible generator: `food.fruits`.
pub fn fruits() -> String {
    parity_pick("food.fruits")
}

/// Gem-compatible generator: `food.measurement_sizes`.
pub fn measurement() -> String {
    parity_pick("food.measurement_sizes")
}

/// Gem-compatible generator: `food.metric_measurements`.
pub fn metric_measurement() -> String {
    parity_pick("food.metric_measurements")
}

/// Gem-compatible generator: `food.spices`.
pub fn spice() -> String {
    parity_pick("food.spices")
}

/// Gem-compatible generator: `food.vegetables`.
pub fn vegetables() -> String {
    parity_pick("food.vegetables")
}

/// Gem-compatible generator: a sushi dish from the locale data.
pub fn sushi() -> String {
    parity_pick("food.sushi")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dish() {
        assert!(!dish().is_empty());
    }

    #[test]
    fn test_ingredient() {
        assert!(!ingredient().is_empty());
    }
}

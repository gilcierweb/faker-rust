//! Food generator - generates random food dishes, ingredients, and descriptions

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random food dish (e.g. "Caesar Salad")
pub fn dish() -> String {
    fetch_locale_with_context("food.dish", "en", Some("food"))
        .map(|v| sample_with_resolve(&v, Some("food")))
        .unwrap_or_else(|| sample(FALLBACK_DISHES).to_string())
}

/// Generate a random food description
pub fn description() -> String {
    fetch_locale_with_context("food.description", "en", Some("food"))
        .map(|v| sample_with_resolve(&v, Some("food")))
        .unwrap_or_else(|| "A delicious meal prepared with fresh ingredients.".to_string())
}

/// Generate a random food ingredient
pub fn ingredient() -> String {
    fetch_locale_with_context("food.ingredient", "en", Some("food"))
        .map(|v| sample_with_resolve(&v, Some("food")))
        .unwrap_or_else(|| sample(FALLBACK_INGREDIENTS).to_string())
}

/// Generate a random fruit
pub fn fruit() -> String {
    fetch_locale_with_context("food.fruits", "en", Some("food"))
        .map(|v| sample_with_resolve(&v, Some("food")))
        .unwrap_or_else(|| sample(FALLBACK_FRUITS).to_string())
}

/// Generate a random vegetable
pub fn vegetable() -> String {
    fetch_locale_with_context("food.vegetables", "en", Some("food"))
        .map(|v| sample_with_resolve(&v, Some("food")))
        .unwrap_or_else(|| sample(FALLBACK_VEGETABLES).to_string())
}

// Fallback data
const FALLBACK_DISHES: &[&str] = &["Pizza", "Burger", "Sushi", "Pasta", "Taco"];
const FALLBACK_INGREDIENTS: &[&str] = &["Tomato", "Cheese", "Onion", "Garlic", "Chicken"];
const FALLBACK_FRUITS: &[&str] = &["Apple", "Banana", "Orange", "Strawberry", "Grape"];
const FALLBACK_VEGETABLES: &[&str] = &["Carrot", "Broccoli", "Spinach", "Potato", "Cucumber"];

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

//! Tea generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random tea type
pub fn r#type() -> String {
    fetch_locale("tea.types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TYPES).to_string())
}

/// Generate a random tea variety
pub fn variety() -> String {
    fetch_locale("tea.varieties", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_VARIETIES).to_string())
}

/// Generate a random tea origin
pub fn origin() -> String {
    fetch_locale("tea.origins", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ORIGINS).to_string())
}

// Fallback data
const FALLBACK_TYPES: &[&str] = &[
    "Black", "Green", "White", "Oolong", "Pu-erh", "Herbal", "Rooibos",
    "Mate", "Chai", "Matcha", "Sencha", "Earl Grey", "Darjeeling",
];

const FALLBACK_VARIETIES: &[&str] = &[
    "English Breakfast", "Earl Grey", "Jasmine", "Chamomile", "Peppermint",
    "Green Tea", "Oolong", "White Tea", "Masala Chai", "Matcha",
    "Sencha", "Gyokuro", "Dragon Well", "Tie Guan Yin", "Da Hong Pao",
];

const FALLBACK_ORIGINS: &[&str] = &[
    "China", "India", "Japan", "Sri Lanka", "Taiwan", "Kenya", "Indonesia",
    "Vietnam", "Turkey", "Iran", "Argentina", "South Africa",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(!r#type().is_empty());
    }

    #[test]
    fn test_variety() {
        assert!(!variety().is_empty());
    }

    #[test]
    fn test_origin() {
        assert!(!origin().is_empty());
    }
}

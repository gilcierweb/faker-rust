//! Adjective generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random adjective
pub fn adjective() -> String {
    fetch_locale("adjective.adjectives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ADJECTIVES).to_string())
}

/// Generate a positive adjective
pub fn positive() -> String {
    fetch_locale("adjective.positives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_POSITIVE).to_string())
}

/// Generate a negative adjective
pub fn negative() -> String {
    fetch_locale("adjective.negatives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NEGATIVE).to_string())
}

// Fallback data
const FALLBACK_ADJECTIVES: &[&str] = &[
    "adorable", "adventurous", "aggressive", "agreeable", "alert", "alive",
    "amused", "angry", "annoyed", "annoying", "anxious", "arrogant", "ashamed",
    "attractive", "average", "awful", "bad", "beautiful", "better", "bewildered",
    "black", "bloody", "blue", "blushing", "bored", "brainy", "brave", "breakable",
    "bright", "busy", "calm", "careful", "cautious", "charming", "cheerful",
    "clean", "clear", "clever", "cloudy", "clumsy", "colorful", "combative",
    "comfortable", "concerned", "condemned", "confused", "cooperative", "courageous",
    "crazy", "creepy", "crowded", "cruel", "curious", "cute", "dangerous",
    "dark", "dead", "defeated", "defiant", "delightful", "depressed", "determined",
];

const FALLBACK_POSITIVE: &[&str] = &[
    "amazing", "awesome", "brilliant", "excellent", "fantastic", "good",
    "great", "incredible", "outstanding", "perfect", "remarkable", "superb",
    "terrific", "wonderful", "magnificent", "extraordinary", "fabulous",
];

const FALLBACK_NEGATIVE: &[&str] = &[
    "awful", "bad", "terrible", "horrible", "disgusting", "unpleasant",
    "disappointing", "mediocre", "poor", "weak", "useless", "pathetic",
    "dreadful", "atrocious", "appalling",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjective() {
        assert!(!adjective().is_empty());
    }

    #[test]
    fn test_positive() {
        assert!(!positive().is_empty());
    }

    #[test]
    fn test_negative() {
        assert!(!negative().is_empty());
    }
}

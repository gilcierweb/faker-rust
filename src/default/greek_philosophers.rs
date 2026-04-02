//! Greek philosophers generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Greek philosopher name
pub fn name() -> String {
    fetch_locale("greek_philosophers.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random philosophical quote
pub fn quote() -> String {
    fetch_locale("greek_philosophers.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random philosophical school
pub fn school() -> String {
    fetch_locale("greek_philosophers.schools", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SCHOOLS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Socrates", "Plato", "Aristotle", "Pythagoras", "Heraclitus",
    "Democritus", "Epicurus", "Zeno of Citium", "Diogenes", "Thales",
    "Anaximander", "Parmenides", "Empedocles", "Anaxagoras", "Protagoras",
    "Gorgias", "Antisthenes", "Pyrrho", "Plotinus", "Hypatia",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Know thyself.",
    "The unexamined life is not worth living.",
    "We are what we repeatedly do.",
    "Everything flows.",
    "Man is the measure of all things.",
    "There is only one good, knowledge, and one evil, ignorance.",
    "Happiness is the highest good.",
    "Nature loves to hide.",
    "Change is the only constant.",
    "Virtue is sufficient for happiness.",
];

const FALLBACK_SCHOOLS: &[&str] = &[
    "Stoicism", "Epicureanism", "Platonism", "Aristotelianism",
    "Cynicism", "Skepticism", "Pythagoreanism", "Pre-Socratic",
    "Neoplatonism", "Atomism", "Sophism",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_school() {
        assert!(!school().is_empty());
    }
}

//! Verb generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random verb
pub fn verb() -> String {
    fetch_locale("verb.verbs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_VERBS).to_string())
}

/// Generate a random past tense verb
pub fn past() -> String {
    fetch_locale("verb.pasts", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PASTS).to_string())
}

/// Generate a random present participle
pub fn ing_form() -> String {
    fetch_locale("verb.ing_forms", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ING).to_string())
}

// Fallback data
const FALLBACK_VERBS: &[&str] = &[
    "run", "walk", "jump", "swim", "fly", "drive", "read", "write", "speak",
    "listen", "eat", "drink", "sleep", "wake", "think", "feel", "know",
    "understand", "learn", "teach", "create", "build", "destroy", "grow",
    "shrink", "open", "close", "start", "stop", "begin", "end", "continue",
];

const FALLBACK_PASTS: &[&str] = &[
    "ran", "walked", "jumped", "swam", "flew", "drove", "read", "wrote",
    "spoke", "listened", "ate", "drank", "slept", "woke", "thought", "felt",
    "knew", "understood", "learned", "taught", "created", "built", "destroyed",
    "grew", "shrank", "opened", "closed", "started", "stopped", "began",
];

const FALLBACK_ING: &[&str] = &[
    "running", "walking", "jumping", "swimming", "flying", "driving", "reading",
    "writing", "speaking", "listening", "eating", "drinking", "sleeping",
    "waking", "thinking", "feeling", "knowing", "understanding", "learning",
    "teaching", "creating", "building", "destroying", "growing", "shrinking",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb() {
        assert!(!verb().is_empty());
    }

    #[test]
    fn test_past() {
        assert!(!past().is_empty());
    }

    #[test]
    fn test_ing_form() {
        assert!(!ing_form().is_empty());
    }
}

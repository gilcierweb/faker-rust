//! Emotion generator - generates emotional adjectives and nouns

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random emotional adjective
pub fn adjective() -> String {
    fetch_locale("emotion.adjective", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "happy".to_string())
}

/// Generate a random emotional noun
pub fn noun() -> String {
    fetch_locale("emotion.noun", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "joy".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_generation() {
        assert!(!adjective().is_empty());
        assert!(!noun().is_empty());
    }
}

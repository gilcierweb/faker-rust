//! Dog generator - generates dog names, breeds, sounds, and descriptors

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random dog name
pub fn name() -> String {
    fetch_locale("creature.dog.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Buddy".to_string())
}

/// Generate a random dog breed
pub fn breed() -> String {
    fetch_locale("creature.dog.breed", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Golden Retriever".to_string())
}

/// Generate a random dog sound
pub fn sound() -> String {
    fetch_locale("creature.dog.sound", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "woof".to_string())
}

/// Generate a random dog meme phrase
pub fn meme_phrase() -> String {
    fetch_locale("creature.dog.meme_phrase", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "wow such doge".to_string())
}

/// Generate a random dog age
pub fn age() -> String {
    fetch_locale("creature.dog.age", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "adult".to_string())
}

/// Generate a random dog coat length
pub fn coat_length() -> String {
    fetch_locale("creature.dog.coat_length", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "short".to_string())
}

/// Generate a random dog size
pub fn size() -> String {
    fetch_locale("creature.dog.size", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "medium".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_generation() {
        assert!(!name().is_empty());
        assert!(!breed().is_empty());
        assert!(!sound().is_empty());
        assert!(!meme_phrase().is_empty());
        assert!(!age().is_empty());
        assert!(!coat_length().is_empty());
        assert!(!size().is_empty());
    }
}

//! Tolkien-themed generators backed by the bundled `tolkien` locale data.

use crate::base::sample;
use crate::locale::fetch_locale_with_context;

const CONTEXT: &str = "tolkien";

fn values(key: &str) -> Option<Vec<String>> {
    fetch_locale_with_context(&format!("tolkien.{key}"), "en", Some(CONTEXT))
}

/// A random Tolkien race (Elves, Dwarves, Hobbits, ...).
pub fn race() -> String {
    values("races")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hobbit".to_string())
}

/// A random Tolkien character.
pub fn character() -> String {
    values("characters")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bilbo Baggins".to_string())
}

/// A random location from Tolkien's world.
pub fn location() -> String {
    values("locations")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Shire".to_string())
}

/// A random poem or song line.
pub fn poem() -> String {
    values("poems")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Chip the glasses and crack the plates".to_string())
}

/// A random character from *The Lord of the Rings*.
pub fn lord_of_the_rings_character() -> String {
    values("lord_of_the_rings.characters")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Frodo Baggins".to_string())
}

/// A random character from *The Hobbit*.
pub fn hobbit_character() -> String {
    values("hobbit.character")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bilbo Baggins".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tolkien_generators_resolve_locale_data() {
        // The bundled YAML has thousands of entries; a single fallback value
        // would never produce this much variety.
        fn distinct(f: fn() -> String) -> usize {
            let mut seen = std::collections::HashSet::new();
            for _ in 0..100 {
                seen.insert(f());
            }
            seen.len()
        }

        assert!(distinct(race) > 5, "races did not come from locale data");
        assert!(
            distinct(character) > 5,
            "characters did not come from locale data"
        );
        assert!(
            distinct(location) > 5,
            "locations did not come from locale data"
        );
        assert!(distinct(poem) > 5, "poems did not come from locale data");
        assert!(
            distinct(lord_of_the_rings_character) > 5,
            "LOTR characters did not come from locale data"
        );
        assert!(
            distinct(hobbit_character) > 5,
            "Hobbit characters did not come from locale data"
        );
    }

    #[test]
    fn test_tolkien_generators_are_deterministic_with_seed() {
        crate::config::FakerConfig::set_seed(3);
        let first = character();
        crate::config::FakerConfig::set_seed(3);
        let second = character();
        assert_eq!(first, second);
    }
}

//! Australian locations, animals, and states backed by bundled locale data.

use crate::base::sample;
use crate::locale::fetch_locale_with_context;

const CONTEXT: &str = "australia";

fn values(key: &str) -> Option<Vec<String>> {
    fetch_locale_with_context(
        &format!("locations.australia.{key}"),
        &crate::config::FakerConfig::locale(),
        Some(CONTEXT),
    )
}

/// A random Australian location.
pub fn location() -> String {
    values("locations")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sydney".to_string())
}

/// A random Australian animal.
pub fn animal() -> String {
    values("animals")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Kangaroo".to_string())
}

/// A random Australian state or territory.
pub fn state() -> String {
    values("states")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "New South Wales".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_australia_generators_resolve_locale_data() {
        const KNOWN_LOCATIONS: &[&str] = &["Sydney", "Melbourne", "Brisbane", "Perth", "Adelaide"];
        const KNOWN_ANIMALS: &[&str] = &["Koala", "Kangaroo", "Platypus", "Wallaby"];
        const KNOWN_STATES: &[&str] = &["Queensland", "Victoria", "Tasmania", "Western Australia"];

        let mut saw_known_location = false;
        let mut saw_known_animal = false;
        let mut saw_known_state = false;
        for _ in 0..100 {
            if KNOWN_LOCATIONS.contains(&location().as_str()) {
                saw_known_location = true;
            }
            if KNOWN_ANIMALS.contains(&animal().as_str()) {
                saw_known_animal = true;
            }
            if KNOWN_STATES.contains(&state().as_str()) {
                saw_known_state = true;
            }
        }
        assert!(
            saw_known_location,
            "locations did not come from locale data"
        );
        assert!(saw_known_animal, "animals did not come from locale data");
        assert!(saw_known_state, "states did not come from locale data");
    }
}

//! Ancient generator - generates names of gods, heroes, and locations from antiquity

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random ancient god
pub fn god() -> String {
    fetch_locale("ancient.god", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Zeus".to_string())
}

/// Generate a random primordial deity
pub fn primordial() -> String {
    fetch_locale("ancient.primordial", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Chaos".to_string())
}

/// Generate a random titan
pub fn titan() -> String {
    fetch_locale("ancient.titan", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Atlas".to_string())
}

/// Generate a random ancient hero
pub fn hero() -> String {
    fetch_locale("ancient.hero", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hercules".to_string())
}

/// Generate a random ancient location
pub fn location() -> String {
    fetch_locale("ancient.location", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Mount Olympus".to_string())
}

/// Generate a random ancient monster
pub fn monster() -> String {
    fetch_locale("ancient.monster", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Medusa".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancient_generation() {
        assert!(!god().is_empty());
        assert!(!primordial().is_empty());
        assert!(!titan().is_empty());
        assert!(!hero().is_empty());
        assert!(!location().is_empty());
        assert!(!monster().is_empty());
    }
}

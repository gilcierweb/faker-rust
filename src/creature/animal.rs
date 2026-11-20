//! Animal generator - generates animal names

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random animal name
pub fn name() -> String {
    fetch_locale(
        "creature.animal.name",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "alligator".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let n = name();
        assert!(!n.is_empty());
    }
}

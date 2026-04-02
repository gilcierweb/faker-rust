//! Horse generator - generates horse names and breeds

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random horse name
pub fn name() -> String {
    fetch_locale("creature.horse.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Noir".to_string())
}

/// Generate a random horse breed
pub fn breed() -> String {
    fetch_locale("creature.horse.breed", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Arabian".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horse_generation() {
        assert!(!name().is_empty());
        assert!(!breed().is_empty());
    }
}

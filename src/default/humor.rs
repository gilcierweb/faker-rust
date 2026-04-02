//! Humor generator - generates funny names and puns

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random funny name
pub fn name() -> String {
    fetch_locale("funny_name.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Anita Bath".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humor_generation() {
        assert!(!name().is_empty());
    }
}

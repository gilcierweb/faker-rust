//! Artist generator - generates random artist names

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random artist name
pub fn name() -> String {
    fetch_locale("artist.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Vincent van Gogh".to_string())
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

//! RuPaul's Drag Race generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random RuPaul's Drag Race queen
pub fn queen() -> String {
    fetch_locale("rupaul.queens", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "RuPaul".to_string())
}

/// Generate a random RuPaul's Drag Race quote
pub fn quote() -> String {
    fetch_locale("rupaul.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Shashay away!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rupaul_generation() {
        assert!(!queen().is_empty());
        assert!(!quote().is_empty());
    }
}

//! Hobby generator - generates random hobby activities

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random hobby activity
pub fn activity() -> String {
    fetch_locale("hobby.activity", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Reading".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_is_nonempty_without_placeholders() {
        let a = activity();
        assert!(!a.is_empty());
        assert!(!a.contains("#{"), "unresolved: {a}");
    }
}

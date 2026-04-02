//! Computer generator - generates random computer-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random computer type
pub fn type_name() -> String {
    fetch_locale("computer.type", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Laptop".to_string())
}

/// Generate a random computer platform
pub fn platform() -> String {
    fetch_locale("computer.platform", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Linux".to_string())
}

/// Generate a random linux processor
pub fn linux_processor() -> String {
    fetch_locale("computer.linux_processor", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "x86_64".to_string())
}

/// Generate a random operating system
pub fn os() -> String {
    fetch_locale("computer.os", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ubuntu".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_name() {
        let t = type_name();
        assert!(!t.is_empty());
    }
}

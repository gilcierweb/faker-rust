//! Control (Video Game) generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random character from Control
pub fn character() -> String {
    fetch_locale("control.character", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Jesse Faden".to_string())
}

/// Generate a random location from Control
pub fn location() -> String {
    fetch_locale("control.location", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "The Oldest House".to_string())
}

/// Generate a random Object of Power from Control
pub fn object_of_power() -> String {
    fetch_locale("control.object_of_power", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Service Weapon".to_string())
}

/// Generate a random Altered Item from Control
pub fn altered_item() -> String {
    fetch_locale("control.altered_item", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Floppy Disk".to_string())
}

/// Generate a random Altered World Event from Control
pub fn altered_world_event() -> String {
    fetch_locale("control.altered_world_event", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bright Falls AWE".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_generation() {
        assert!(!character().is_empty());
        assert!(!location().is_empty());
        assert!(!object_of_power().is_empty());
        assert!(!altered_item().is_empty());
        assert!(!altered_world_event().is_empty());
    }
}

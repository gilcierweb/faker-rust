//! Color generator - generates random color names

use crate::base::sample;
use crate::locale::fetch_locale;

/// Gem-compatible alias for [`hex`].
pub use self::hex as hex_color;
/// Gem-compatible alias for [`hsl`].
pub use self::hsl as hsl_color;
/// Gem-compatible alias for [`rgb`].
pub use self::rgb as rgb_color;

/// Generate a random color name
pub fn name() -> String {
    fetch_locale("color.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "red".to_string())
}

/// Generate a random hex color (e.g., #ffffff)
pub fn hex() -> String {
    let config = crate::config::FakerConfig::current();
    format!("#{:06x}", config.rand_range(0, 0xFFFFFF))
}

/// Generate a single random RGB color channel (0..=255).
pub fn single_rgb_color() -> u8 {
    crate::config::FakerConfig::current().rand_range(0, 255) as u8
}

/// Generate a random RGB color (e.g., [255, 255, 255])
pub fn rgb() -> [u8; 3] {
    let config = crate::config::FakerConfig::current();
    [
        config.rand_range(0, 255) as u8,
        config.rand_range(0, 255) as u8,
        config.rand_range(0, 255) as u8,
    ]
}

/// Generate a random HSL color (e.g., [360, 100, 100])
pub fn hsl() -> [u32; 3] {
    let config = crate::config::FakerConfig::current();
    [
        config.rand_range(0, 360),
        config.rand_range(0, 100),
        config.rand_range(0, 100),
    ]
}

/// Generate a random HSLA color (e.g., [360, 100, 100, 1.0])
pub fn hsla() -> (u32, u32, u32, f32) {
    let config = crate::config::FakerConfig::current();
    (
        config.rand_range(0, 360),
        config.rand_range(0, 100),
        config.rand_range(0, 100),
        config.rand_range(0, 100) as f32 / 100.0,
    )
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `color.name`.
pub fn color_name() -> String {
    parity_pick("color.name")
}
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: CSS `hsla()` colour string.
pub fn hsla_color() -> String {
    let config = crate::config::FakerConfig::current();
    format!(
        "hsla({}, {}%, {}%, {:.2})",
        config.rand_range(0, 361),
        config.rand_range(0, 101),
        config.rand_range(0, 101),
        config.rand_f64()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let h = hex();
        assert!(h.starts_with('#'));
        assert_eq!(h.len(), 7);
    }
}

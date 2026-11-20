//! Bird generator - generates bird names, anatomy, and descriptors

use crate::base::sample;
use crate::locale::{fetch_locale, sample_with_resolve};

/// Gem-compatible alias for [`adjectives`].
pub use self::adjectives as adjective;
/// Gem-compatible alias for [`colors`].
pub use self::colors as color;

/// Generate a random bird anatomy part
pub fn anatomy() -> String {
    fetch_locale("creature.bird.anatomy", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "beak".to_string())
}

/// Generate a random bird color
pub fn colors() -> String {
    fetch_locale("creature.bird.colors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "black".to_string())
}

/// Generate a random bird adjective
pub fn adjectives() -> String {
    fetch_locale("creature.bird.adjectives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "common".to_string())
}

/// Generate a random bird common family name
pub fn common_family_name() -> String {
    fetch_locale("creature.bird.common_family_name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Hawk".to_string())
}

/// Generate a plausible bird common name
pub fn common_name() -> String {
    fetch_locale("creature.bird.plausible_common_names", "en")
        .map(|v| sample_with_resolve(&v, Some("creature.bird")))
        .unwrap_or_else(|| "Common Black Hawk".to_string())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `creature.bird.anatomy_past_tense`.
pub fn anatomy_past_tense() -> String {
    parity_pick("creature.bird.anatomy_past_tense")
}

/// Gem-compatible generator: `creature.bird.emotional_adjectives`.
pub fn emotional_adjective() -> String {
    parity_pick("creature.bird.emotional_adjectives")
}

/// Gem-compatible generator: `creature.bird.geo`.
pub fn geo() -> String {
    parity_pick("creature.bird.geo")
}

/// Gem-compatible generator: `creature.bird.silly_adjectives`.
pub fn silly_adjective() -> String {
    parity_pick("creature.bird.silly_adjectives")
}
// --- gem parity generators (dynamic locale keys) ---
/// Gem-compatible generator: a random bird taxonomic order.
pub fn order() -> String {
    crate::base::sample(&crate::locale::fetch_locale_keys(
        "creature.bird.order_common_map",
        "en",
    ))
}
/// Gem-compatible generator: a plausible bird common name.
pub fn plausible_common_name() -> String {
    crate::locale::fetch_locale("creature.bird.plausible_common_names", "en")
        .map(|v| crate::locale::sample_with_resolve(&v, Some("creature.bird")))
        .unwrap_or_default()
}
/// Gem-compatible generator: an implausible bird common name.
pub fn implausible_common_name() -> String {
    crate::locale::fetch_locale("creature.bird.implausible_common_names", "en")
        .map(|v| crate::locale::sample_with_resolve(&v, Some("creature.bird")))
        .unwrap_or_default()
}
/// Gem-compatible generator: an order paired with one of its common names.
pub fn order_with_common_name(tax_order: Option<&str>) -> (String, String) {
    let selected = tax_order.map(str::to_string).unwrap_or_else(order);
    let names =
        crate::locale::fetch_locale(&format!("creature.bird.order_common_map.{selected}"), "en")
            .unwrap_or_default();
    (selected, crate::base::sample(&names))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bird_generation() {
        assert!(!anatomy().is_empty());
        assert!(!colors().is_empty());
        assert!(!adjectives().is_empty());
        assert!(!common_family_name().is_empty());
        assert!(!common_name().is_empty());
    }

    #[test]
    fn test_common_name_resolves_placeholders() {
        // The locale templates contain #{...} placeholders; outputs must be
        // resolved rather than leaking raw placeholder syntax.
        for _ in 0..50 {
            let name = common_name();
            assert!(!name.contains("#{"), "unresolved placeholder: {name}");
        }
    }
}

//! Mountain generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random mountain name
pub fn name() -> String {
    fetch_locale("mountain.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random mountain range
pub fn range() -> String {
    fetch_locale("mountain.ranges", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_RANGES).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Mount Everest", "K2", "Kangchenjunga", "Lhotse", "Makalu", "Cho Oyu",
    "Dhaulagiri", "Manaslu", "Nanga Parbat", "Annapurna", "Gasherbrum I",
    "Broad Peak", "Gasherbrum II", "Shishapangma", "Mount Kilimanjaro",
    "Mount McKinley", "Mount Elbrus", "Mount Kilimanjaro", "Mount Fuji",
    "Mount Rainier", "Matterhorn", "Mont Blanc",
];

const FALLBACK_RANGES: &[&str] = &[
    "Himalayas", "Karakoram", "Hindu Kush", "Andes", "Alps", "Rockies",
    "Appalachians", "Cascades", "Sierra Nevada", "Atlas Mountains",
    "Drakensberg", "Caucasus Mountains", "Ural Mountains",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_range() {
        assert!(!range().is_empty());
    }
}

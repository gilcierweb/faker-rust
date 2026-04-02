//! Minecraft generator - generates random Minecraft items and data

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Minecraft item
pub fn item() -> String {
    fetch_locale_with_context("games.minecraft.items", "en", Some("games.minecraft"))
        .map(|v| sample_with_resolve(&v, Some("games.minecraft")))
        .unwrap_or_else(|| sample(FALLBACK_MINECRAFT_ITEMS).to_string())
}

/// Generate a random Minecraft block
pub fn block() -> String {
    fetch_locale_with_context("games.minecraft.blocks", "en", Some("games.minecraft"))
        .map(|v| sample_with_resolve(&v, Some("games.minecraft")))
        .unwrap_or_else(|| sample(&["Stone", "Dirt", "Grass Block"]).to_string())
}

/// Generate a random Minecraft biome
pub fn biome() -> String {
    fetch_locale_with_context("games.minecraft.biome", "en", Some("games.minecraft"))
        .map(|v| sample_with_resolve(&v, Some("games.minecraft")))
        .unwrap_or_else(|| sample(&["Plains", "Forest", "Desert"]).to_string())
}

const FALLBACK_MINECRAFT_ITEMS: &[&str] = &[
    "Iron Pickaxe",
    "Diamond Sword",
    "Apple",
    "Torch",
    "Crafting Table",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        assert!(!item().is_empty());
    }
}

//! Pokemon generator - generates random Pokemon names and data

use crate::base::sample;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random Pokemon name
pub fn name() -> String {
    fetch_locale_with_context("games.pokemon.names", "en", Some("games.pokemon"))
        .map(|v| sample_with_resolve(&v, Some("games.pokemon")))
        .unwrap_or_else(|| sample(FALLBACK_POKEMON_NAMES).to_string())
}

/// Generate a random Pokemon location
pub fn location() -> String {
    fetch_locale_with_context("games.pokemon.locations", "en", Some("games.pokemon"))
        .map(|v| sample_with_resolve(&v, Some("games.pokemon")))
        .unwrap_or_else(|| sample(&["Pallet Town", "Pewter City", "Cerulean City"]).to_string())
}

/// Generate a random Pokemon move
pub fn move_name() -> String {
    fetch_locale_with_context("games.pokemon.moves", "en", Some("games.pokemon"))
        .map(|v| sample_with_resolve(&v, Some("games.pokemon")))
        .unwrap_or_else(|| sample(&["Tackle", "Thunderbolt", "Hyper Beam"]).to_string())
}

const FALLBACK_POKEMON_NAMES: &[&str] =
    &["Pikachu", "Bulbasaur", "Charmander", "Squirtle", "Mewtwo"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }
}

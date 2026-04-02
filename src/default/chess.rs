//! Chess generator - generates random chess-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random chess player name
pub fn player() -> String {
    fetch_locale("chess.players", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Magnus Carlsen".to_string())
}

/// Generate a random chess tournament name
pub fn tournament() -> String {
    fetch_locale("chess.tournaments", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Candidates Tournament".to_string())
}

/// Generate a random chess opening name
pub fn opening() -> String {
    fetch_locale("chess.openings", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sicilian Defense".to_string())
}

/// Generate a random chess title (GM, IM, etc.)
pub fn title() -> String {
    fetch_locale("chess.titles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Grandmaster".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player() {
        let p = player();
        assert!(!p.is_empty());
    }
}

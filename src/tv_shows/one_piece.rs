//! One Piece generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random One Piece character
pub fn character() -> String {
    fetch_locale("one_piece.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Monkey D. Luffy".to_string())
}

/// Generate a random One Piece sea
pub fn sea() -> String {
    fetch_locale("one_piece.seas", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "East Blue".to_string())
}

/// Generate a random One Piece island
pub fn island() -> String {
    fetch_locale("one_piece.islands", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Dawn Island".to_string())
}

/// Generate a random One Piece location
pub fn location() -> String {
    fetch_locale("one_piece.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Foosha Village".to_string())
}

/// Generate a random One Piece quote
pub fn quote() -> String {
    fetch_locale("one_piece.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "I'm gonna be the King of the Pirates!".to_string())
}

/// Generate a random One Piece Akuma no Mi (Devil Fruit)
pub fn akuma_no_mi() -> String {
    fetch_locale("one_piece.akuma_no_mi", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Gomu Gomu no Mi".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_piece_generation() {
        assert!(!character().is_empty());
        assert!(!sea().is_empty());
        assert!(!island().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
        assert!(!akuma_no_mi().is_empty());
    }
}

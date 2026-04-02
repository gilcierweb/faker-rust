//! Coin generator - generates random coin-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Flip a coin (Heads or Tails)
pub fn flip() -> String {
    fetch_locale("coin.flip", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Heads".to_string())
}

/// Generate a random coin name
pub fn name() -> String {
    let coins = ["Penny", "Nickel", "Dime", "Quarter", "Half Dollar", "Dollar"];
    sample(&coins).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip() {
        let f = flip();
        assert!(f == "Heads" || f == "Tails");
    }
}

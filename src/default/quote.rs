//! Quote generator - generates famous quotes from various sources

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a famous quote
pub fn famous() -> String {
    fetch_locale("quote.famous", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "I think, therefore I am.".to_string())
}

/// Generate a quote from Matz (Yukihiro Matsumoto)
pub fn matz() -> String {
    fetch_locale("quote.matz", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Ruby is designed to make programmers happy.".to_string())
}

/// Generate a quote from Robin Williams
pub fn robin() -> String {
    fetch_locale("quote.robin", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| {
            "No matter what people tell you, words and ideas can change the world.".to_string()
        })
}

/// Generate a quote by Jack Handey
pub fn jack_handey() -> String {
    fetch_locale("quote.jack_handey", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "I hope if dogs ever take over the world, and they chose a king, they don't just go by size, because I bet there are some Lincoln Terrier-sized dogs with a lot of good ideas.".to_string())
}

/// Generate a quote from the Yiddish proverb
pub fn yiddish() -> String {
    fetch_locale("quote.yiddish", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "A bad peace is better than a good war.".to_string())
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `quote.famous_last_words`.
pub fn famous_last_words() -> String {
    parity_pick("quote.famous_last_words")
}

/// Gem-compatible generator: `quote.mitch_hedberg`.
pub fn mitch_hedberg() -> String {
    parity_pick("quote.mitch_hedberg")
}

/// Gem-compatible generator: `quote.most_interesting_man_in_the_world`.
pub fn most_interesting_man_in_the_world() -> String {
    parity_pick("quote.most_interesting_man_in_the_world")
}

/// Gem-compatible generator: `quote.singular_siegler`.
pub fn singular_siegler() -> String {
    parity_pick("quote.singular_siegler")
}

/// Gem-compatible generator: `quote.yoda`.
pub fn yoda() -> String {
    parity_pick("quote.yoda")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_generation() {
        assert!(!famous().is_empty());
        assert!(!matz().is_empty());
        assert!(!robin().is_empty());
        assert!(!jack_handey().is_empty());
        assert!(!yiddish().is_empty());
    }
}

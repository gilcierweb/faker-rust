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
        .unwrap_or_else(|| "No matter what people tell you, words and ideas can change the world.".to_string())
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

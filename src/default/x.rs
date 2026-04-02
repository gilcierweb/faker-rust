//! X (Twitter) generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random X/Twitter username
pub fn screen_name() -> String {
    fetch_locale("x.screen_names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SCREEN_NAMES).to_string())
}

/// Generate a random X/Twitter hashtag
pub fn hashtag() -> String {
    fetch_locale("x.hashtags", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HASHTAGS).to_string())
}

/// Generate a random X/Twitter tweet text
pub fn tweet() -> String {
    fetch_locale("x.tweets", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TWEETS).to_string())
}

// Fallback data
const FALLBACK_SCREEN_NAMES: &[&str] = &[
    "@elonmusk", "@nasa", "@rustlang", "@github", "@stackoverflow",
    "@techcrunch", "@wired", "@verge", "@engadget", "@arstechnica",
    "@mozilla", "@google", "@apple", "@microsoft", "@amazon",
];

const FALLBACK_HASHTAGS: &[&str] = &[
    "#Rust", "#Programming", "#Tech", "#AI", "#MachineLearning",
    "#WebDev", "#OpenSource", "#Coding", "#DevOps", "#Cloud",
    "#CyberSecurity", "#DataScience", "#100DaysOfCode", "#TechNews",
];

const FALLBACK_TWEETS: &[&str] = &[
    "Just shipped a new feature! 🚀",
    "Learning Rust has been an amazing journey.",
    "Open source is the future of software.",
    "Working on something exciting today! 💻",
    "Code reviews are essential for quality.",
    "Debugging is like being a detective.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_name() {
        let name = screen_name();
        assert!(name.starts_with('@'));
    }

    #[test]
    fn test_hashtag() {
        let tag = hashtag();
        assert!(tag.starts_with('#'));
    }

    #[test]
    fn test_tweet() {
        assert!(!tweet().is_empty());
    }
}

//! X (Twitter) generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random X/Twitter username
pub fn screen_name() -> String {
    fetch_locale("x.screen_names", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SCREEN_NAMES).to_string())
}

/// Generate a random X/Twitter hashtag
pub fn hashtag() -> String {
    fetch_locale("x.hashtags", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HASHTAGS).to_string())
}

/// Generate a random X/Twitter tweet text
pub fn tweet() -> String {
    fetch_locale("x.tweets", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TWEETS).to_string())
}

/// Gem-compatible generator: a complete X user payload as a JSON object.
pub fn user() -> serde_json::Value {
    const MAX_ID: i64 = 9_223_372_036_854_775_807;
    let author_id = crate::number::between(1, MAX_ID).to_string();
    let tweet_id = crate::number::between(1, MAX_ID).to_string();
    let username = screen_name();

    let data = serde_json::json!([{
        "author_id": author_id,
        "id": tweet_id,
        "text": crate::lorem::sentence(None),
    }]);

    let public_metrics = serde_json::json!({
        "followers_count": crate::number::between(1, 1_000),
        "following_count": crate::number::between(1, 200),
        "tweet_count": crate::number::between(1, 10_000),
        "listed_count": crate::number::between(1, 1_000),
    });

    let entities = serde_json::json!({
        "url": { "urls": [] },
        "description": { "hashtags": [hashtag()] },
    });

    let user = serde_json::json!({
        "public_metrics": public_metrics,
        "username": username,
        "pinned_tweet_id": crate::number::between(1, MAX_ID).to_string(),
        "entities": entities,
        "description": crate::lorem::sentence(None),
        "name": crate::name::name(),
        "verified": crate::boolean::boolean(),
        "location": crate::internet::public_ip_v4_address(),
        "id": author_id,
        "protected": crate::boolean::boolean(),
        "url": format!("https://t.co/{}", crate::lorem::characters(10)),
        "profile_image_url": crate::avatar::image(Some(&tweet_id), Some("48x48"), None),
        "created_at": crate::date::between("2006-03-21", "2026-11-20"),
    });

    serde_json::json!({
        "data": data,
        "includes": { "users": [user] },
    })
}

// Fallback data
const FALLBACK_SCREEN_NAMES: &[&str] = &[
    "@elonmusk",
    "@nasa",
    "@rustlang",
    "@github",
    "@stackoverflow",
    "@techcrunch",
    "@wired",
    "@verge",
    "@engadget",
    "@arstechnica",
    "@mozilla",
    "@google",
    "@apple",
    "@microsoft",
    "@amazon",
];

const FALLBACK_HASHTAGS: &[&str] = &[
    "#Rust",
    "#Programming",
    "#Tech",
    "#AI",
    "#MachineLearning",
    "#WebDev",
    "#OpenSource",
    "#Coding",
    "#DevOps",
    "#Cloud",
    "#CyberSecurity",
    "#DataScience",
    "#100DaysOfCode",
    "#TechNews",
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

//! Slack Emoji generator

use crate::base::sample;

/// Generate a random Slack emoji
pub fn emoji() -> String {
    let emojis = [
        ":smile:",
        ":laughing:",
        ":wink:",
        ":heart_eyes:",
        ":thumbsup:",
        ":fire:",
        ":rocket:",
        ":tada:",
        ":sparkles:",
        ":star:",
        ":coffee:",
        ":pizza:",
        ":beer:",
        ":wine:",
        ":sushi:",
        ":computer:",
        ":phone:",
        ":bulb:",
        ":memo:",
        ":calendar:",
        ":white_check_mark:",
        ":warning:",
        ":x:",
        ":question:",
        ":exclamation:",
        ":wave:",
        ":+1:",
        ":-1:",
        ":clap:",
        ":pray:",
    ];
    sample(&emojis).to_string()
}

/// Generate a random custom Slack emoji name
pub fn custom_emoji() -> String {
    let names = [
        ":party-parrot:",
        ":deal-with-it:",
        ":ship-it:",
        ":aw-yeah:",
        ":table-flip:",
        ":success-kid:",
        ":dancing-banana:",
        ":nyancat:",
        ":doom:",
        ":facepalm:",
        ":mind-blown:",
        ":mic-drop:",
    ];
    sample(&names).to_string()
}

/// Generate a random people emoji
pub fn people() -> String {
    let emojis = [
        ":grinning:",
        ":smiley:",
        ":big_smile:",
        ":laughing:",
        ":sweat_smile:",
        ":joy:",
        ":rofl:",
        ":relaxed:",
        ":blush:",
        ":innocent:",
    ];
    sample(&emojis).to_string()
}

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `slack_emoji.activity`.
pub fn activity() -> String {
    parity_pick("slack_emoji.activity")
}

/// Gem-compatible generator: `slack_emoji.celebration`.
pub fn celebration() -> String {
    parity_pick("slack_emoji.celebration")
}

/// Gem-compatible generator: `slack_emoji.custom`.
pub fn custom() -> String {
    parity_pick("slack_emoji.custom")
}

/// Gem-compatible generator: `slack_emoji.food_and_drink`.
pub fn food_and_drink() -> String {
    parity_pick("slack_emoji.food_and_drink")
}

/// Gem-compatible generator: `slack_emoji.nature`.
pub fn nature() -> String {
    parity_pick("slack_emoji.nature")
}

/// Gem-compatible generator: `slack_emoji.objects_and_symbols`.
pub fn objects_and_symbols() -> String {
    parity_pick("slack_emoji.objects_and_symbols")
}

/// Gem-compatible generator: `slack_emoji.travel_and_places`.
pub fn travel_and_places() -> String {
    parity_pick("slack_emoji.travel_and_places")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji() {
        let e = emoji();
        assert!(e.starts_with(':'));
        assert!(e.ends_with(':'));
    }

    #[test]
    fn test_custom_emoji() {
        let e = custom_emoji();
        assert!(e.starts_with(':'));
        assert!(e.ends_with(':'));
    }

    #[test]
    fn test_people() {
        let e = people();
        assert!(e.starts_with(':'));
        assert!(e.ends_with(':'));
    }
}

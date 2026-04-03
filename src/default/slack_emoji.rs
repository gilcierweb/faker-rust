//! Slack Emoji generator

use crate::base::sample;

/// Generate a random Slack emoji
pub fn emoji() -> String {
    let emojis = [
        ":smile:", ":laughing:", ":wink:", ":heart_eyes:", ":thumbsup:",
        ":fire:", ":rocket:", ":tada:", ":sparkles:", ":star:",
        ":coffee:", ":pizza:", ":beer:", ":wine:", ":sushi:",
        ":computer:", ":phone:", ":bulb:", ":memo:", ":calendar:",
        ":white_check_mark:", ":warning:", ":x:", ":question:", ":exclamation:",
        ":wave:", ":+1:", ":-1:", ":clap:", ":pray:",
    ];
    sample(&emojis).to_string()
}

/// Generate a random custom Slack emoji name
pub fn custom_emoji() -> String {
    let names = [
        ":party-parrot:", ":deal-with-it:", ":ship-it:", ":aw-yeah:",
        ":table-flip:", ":success-kid:", ":dancing-banana:", ":nyancat:",
        ":doom:", ":facepalm:", ":mind-blown:", ":mic-drop:",
    ];
    sample(&names).to_string()
}

/// Generate a random people emoji
pub fn people() -> String {
    let emojis = [
        ":grinning:", ":smiley:", ":big_smile:", ":laughing:", ":sweat_smile:",
        ":joy:", ":rofl:", ":relaxed:", ":blush:", ":innocent:",
    ];
    sample(&emojis).to_string()
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

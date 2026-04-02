//! The Thick of It TV show generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random The Thick of It character
pub fn character() -> String {
    fetch_locale("the_thick_of_it.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random The Thick of It quote/swearing
pub fn quote() -> String {
    fetch_locale("the_thick_of_it.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Malcolm Tucker insult
pub fn malcolm_tucker() -> String {
    fetch_locale("the_thick_of_it.malcolm_tucker", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TUCKER).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Malcolm Tucker",
    "Nicola Murray",
    "Hugh Abbot",
    "Terri Coverley",
    "Robyn Murray",
    "Glenn Cullen",
    "Oliver Reeder",
    "Peter Mannion",
    "Stewart Pearson",
    "Emma Messinger",
    "Phil Smith",
    "Jamie MacDonald",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I need you to go and make peace with the printer.",
    "I'm not sure you've got the balls for that.",
    "You're as useless as a marzipan dildo.",
    "You are a f***ing omnishambles!",
    "I've never seen anything like this!",
    "You're like a sweaty octopus trying to unhook a bra.",
];

const FALLBACK_TUCKER: &[&str] = &[
    "Come the f*** in or f*** the f*** off!",
    "You're a f***ing useless prick!",
    "I'd love to stop and chat but I'd rather have type 2 diabetes.",
    "You are a f***ing waste of space!",
    "I'll tear your f***ing skin off!",
    "You're about as useful as a marzipan dildo.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_malcolm_tucker() {
        assert!(!malcolm_tucker().is_empty());
    }
}

//! Quotes generator - generates famous quotes

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random famous quote
pub fn famous_last_words() -> String {
    fetch_locale("quotes.famous_last_words", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FAMOUS_LAST_WORDS).to_string())
}

/// Generate a random motivational quote
pub fn motivational() -> String {
    fetch_locale("quotes.motivational", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MOTIVATIONAL).to_string())
}

/// Generate a random philosophical quote
pub fn philosophical() -> String {
    fetch_locale("quotes.philosophical", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PHILOSOPHICAL).to_string())
}

/// Generate a random Shakespeare quote
pub fn shakespeare() -> String {
    fetch_locale("quotes.shakespeare", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SHAKESPEARE).to_string())
}

/// Generate a random movie quote
pub fn movie() -> String {
    fetch_locale("quotes.movies", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MOVIES).to_string())
}

// Fallback data
const FALLBACK_FAMOUS_LAST_WORDS: &[&str] = &[
    "Et tu, Brute?",
    "I have a dream.",
    "That's one small step for man.",
    "Mr. Watson, come here, I want you.",
    "I only regret that I have but one life to lose for my country.",
    "I think therefore I am.",
    "To be or not to be.",
];

const FALLBACK_MOTIVATIONAL: &[&str] = &[
    "The only way to do great work is to love what you do.",
    "Be the change you wish to see in the world.",
    "Success is not final, failure is not fatal.",
    "Believe you can and you're halfway there.",
    "The future belongs to those who believe in the beauty of their dreams.",
    "It is during our darkest moments that we must focus to see the light.",
];

const FALLBACK_PHILOSOPHICAL: &[&str] = &[
    "The unexamined life is not worth living.",
    "We are what we repeatedly do.",
    "The only true wisdom is in knowing you know nothing.",
    "Man is condemned to be free.",
    "Hell is other people.",
    "I think, therefore I am.",
];

const FALLBACK_SHAKESPEARE: &[&str] = &[
    "To be, or not to be, that is the question.",
    "All the world's a stage.",
    "Romeo, Romeo, wherefore art thou Romeo?",
    "Something wicked this way comes.",
    "We are such stuff as dreams are made on.",
    "The course of true love never did run smooth.",
];

const FALLBACK_MOVIES: &[&str] = &[
    "Here's looking at you, kid.",
    "I'll be back.",
    "May the Force be with you.",
    "There's no place like home.",
    "You can't handle the truth!",
    "Life is like a box of chocolates.",
    "I see dead people.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_famous_last_words() {
        assert!(!famous_last_words().is_empty());
    }

    #[test]
    fn test_motivational() {
        assert!(!motivational().is_empty());
    }

    #[test]
    fn test_philosophical() {
        assert!(!philosophical().is_empty());
    }

    #[test]
    fn test_shakespeare() {
        assert!(!shakespeare().is_empty());
    }

    #[test]
    fn test_movie() {
        assert!(!movie().is_empty());
    }
}

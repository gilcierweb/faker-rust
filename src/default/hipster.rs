//! Hipster generator - generates hipster-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random hipster word
pub fn word() -> String {
    fetch_locale("hipster.words", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_WORDS).to_string())
}

/// Generate a random hipster sentence
pub fn sentence() -> String {
    let num_words = 5;
    let mut words = Vec::new();
    for _ in 0..num_words {
        words.push(word());
    }
    let mut sentence = words.join(" ");
    sentence.push('.');
    sentence
}

/// Generate a random hipster paragraph
pub fn paragraph() -> String {
    let num_sentences = 3;
    let mut sentences = Vec::new();
    for _ in 0..num_sentences {
        sentences.push(sentence());
    }
    sentences.join(" ")
}

// Fallback data
const FALLBACK_WORDS: &[&str] = &[
    "artisan", "aesthetic", "authentic", "bespoke", "bitters", "biodiesel",
    "brunch", "cardigan", "chia", "chillwave", "church-key", "craft beer",
    "cred", "cronut", "disrupt", "distillery", "DIY", "ennui", "ethical",
    "ethical", "fanny pack", "fashion axe", "fixie", "flannel", "food truck",
    "forage", "freegan", "gluten-free", "hashtag", "heirloom", "hella",
    "hoodie", "intelligentsia", "irony", "kale", "keffiyeh", "kitsch",
    "kombucha", "listicle", "literally", "locavore", "lo-fi", "meggings",
    "messenger bag", "microdosing", "mixtape", "mustache", "normcore",
    "occupy", "organic", "paleo", "pickled", "pinterest", "pitchfork",
    "plaid", "polaroid", "pop-up", "portland", "poutine", "pour-over",
    "quinoa", "raw denim", "readymade", "retro", "roof party", "sartorial",
    "scenester", "selfies", "selvage", "semiotics", "shabby chic", "single-origin",
    "skateboard", "slow-carb", "small batch", "squid", "sriracha", "stumptown",
    "sustainable", "synth", "taxidermy", "thundercats", "tofu", "tote bag",
    "trust fund", "tumblr", "typewriter", "umami", "ugh", "vegan", "venmo",
    "vinyl", "viral", "vintage", "waistcoat", "wayfarers", "whatever",
    "wolf", "yolo", "yr",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word() {
        assert!(!word().is_empty());
    }

    #[test]
    fn test_sentence() {
        assert!(!sentence().is_empty());
    }

    #[test]
    fn test_paragraph() {
        assert!(!paragraph().is_empty());
    }
}

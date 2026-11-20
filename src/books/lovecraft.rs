//! H.P. Lovecraft book generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Gem-compatible alias for [`book`].
pub use self::book as tome;

/// Generate a random Lovecraft creature/deity
pub fn creature() -> String {
    fetch_locale("lovecraft.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATURES).to_string())
}

/// Generate a random Lovecraft location
pub fn location() -> String {
    fetch_locale("lovecraft.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Lovecraft book/tome
pub fn book() -> String {
    fetch_locale("lovecraft.books", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BOOKS).to_string())
}

/// Generate a random Lovecraft quote
pub fn quote() -> String {
    fetch_locale("lovecraft.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CREATURES: &[&str] = &[
    "Cthulhu",
    "Azathoth",
    "Nyarlathotep",
    "Shub-Niggurath",
    "Yog-Sothoth",
    "Dagon",
    "Hastur",
    "Yithian",
    "Deep One",
    "Mi-Go",
    "Elder Thing",
    "Shoggoth",
    "Night-gaunt",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "R'lyeh",
    "Arkham",
    "Innsmouth",
    "Dunwich",
    "Kingsport",
    "Miskatonic University",
    "The Mountains of Madness",
    "Plateau of Leng",
    "Dreamlands",
];

const FALLBACK_BOOKS: &[&str] = &[
    "Necronomicon",
    "De Vermis Mysteriis",
    "Book of Eibon",
    "Cultes des Goules",
    "Unaussprechlichen Kulten",
    "Pnakotic Manuscripts",
    "Eltdown Shards",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn.",
    "That is not dead which can eternal lie, and with strange aeons even death may die.",
    "The oldest and strongest emotion of mankind is fear, and the oldest and strongest kind of fear is fear of the unknown.",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `lovecraft.deity`.
pub fn deity() -> String {
    parity_pick("lovecraft.deity")
}

/// Gem-compatible generator: `lovecraft.words`.
pub fn word() -> String {
    parity_pick("lovecraft.words")
}

// --- gem parity generators (algorithmic) ---
fn lovecraft_words() -> Vec<String> {
    crate::locale::fetch_locale("lovecraft.words", "en")
        .unwrap_or_else(|| vec!["eldritch".to_string()])
}

/// Gem-compatible generator: the Cthulhu chant repeated a number of times.
pub fn fhtagn(number: usize) -> String {
    let chant = parity_pick("lovecraft.fhtagn");
    vec![chant; number].join(". ")
}

/// Gem-compatible generator: a list of Lovecraft words.
pub fn words(number: usize, spaces_allowed: bool) -> Vec<String> {
    let pool = lovecraft_words();
    let candidates: Vec<String> = if spaces_allowed {
        pool
    } else {
        let filtered: Vec<String> = pool.into_iter().filter(|w| !w.contains(' ')).collect();
        if filtered.is_empty() {
            vec!["eldritch".to_string()]
        } else {
            filtered
        }
    };
    (0..number)
        .map(|_| crate::base::sample(&candidates))
        .collect()
}

/// Gem-compatible generator: a Lovecraft sentence.
pub fn sentence(
    word_count: usize,
    random_words_to_add: usize,
    open_compounds_allowed: bool,
) -> String {
    let config = crate::config::FakerConfig::current();
    let extra = if random_words_to_add == 0 {
        0
    } else {
        config.rand_usize(random_words_to_add)
    };
    let mut text = words(word_count + extra, open_compounds_allowed).join(" ");
    if let Some(first) = text.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    format!("{text}.")
}

/// Gem-compatible generator: a list of Lovecraft sentences.
pub fn sentences(number: usize) -> Vec<String> {
    (0..number).map(|_| sentence(3, 0, true)).collect()
}

/// Gem-compatible generator: a Lovecraft paragraph.
pub fn paragraph(sentence_count: usize, random_sentences_to_add: usize) -> String {
    let config = crate::config::FakerConfig::current();
    let extra = if random_sentences_to_add == 0 {
        0
    } else {
        config.rand_usize(random_sentences_to_add)
    };
    sentences(sentence_count + extra).join(" ")
}

/// Gem-compatible generator: a list of Lovecraft paragraphs.
pub fn paragraphs(number: usize) -> Vec<String> {
    (0..number).map(|_| paragraph(3, 0)).collect()
}

/// Gem-compatible generator: a Lovecraft paragraph truncated to a character count.
pub fn paragraph_by_chars(characters: usize) -> String {
    let mut text = paragraph(3, 0);
    while text.len() < characters {
        text.push(' ');
        text.push_str(&paragraph(3, 0));
    }
    let end = characters.saturating_sub(1).min(text.len());
    format!("{}.", &text[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creature() {
        assert!(!creature().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_book() {
        assert!(!book().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

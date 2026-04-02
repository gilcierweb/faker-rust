//! Lorem Ipsum generator - generates random latin-like text

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random word
pub fn word() -> String {
    fetch_locale("lorem.words", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "lorem".to_string())
}

/// Generate multiple random words
pub fn words(num: usize) -> String {
    let _config = crate::config::FakerConfig::current();
    let words: Vec<String> = (0..num).map(|_| word()).collect();
    words.join(" ")
}

/// Generate a random sentence
pub fn sentence(word_count: Option<usize>) -> String {
    let count = word_count.unwrap_or_else(|| {
        let config = crate::config::FakerConfig::current();
        config.rand_range(4, 8) as usize
    });
    let w = words(count);
    let mut chars: Vec<char> = w.chars().collect();
    if !chars.is_empty() {
        chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
    }
    let mut result: String = chars.into_iter().collect();
    result.push('.');
    result
}

/// Generate multiple sentences
pub fn sentences(sentence_count: Option<usize>, separator: Option<&str>) -> String {
    let count = sentence_count.unwrap_or(3);
    let sep = separator.unwrap_or(". ");
    (0..count)
        .map(|_| sentence(None))
        .collect::<Vec<_>>()
        .join(sep)
}

/// Generate a random paragraph
pub fn paragraph(sentence_count: Option<usize>) -> String {
    let count = sentence_count.unwrap_or_else(|| {
        let config = crate::config::FakerConfig::current();
        config.rand_range(3, 6) as usize
    });
    sentences(Some(count), None)
}

/// Generate multiple paragraphs
pub fn paragraphs(paragraph_count: Option<usize>, separator: Option<&str>) -> String {
    let count = paragraph_count.unwrap_or(3);
    let sep = separator.unwrap_or("\n\n");
    (0..count)
        .map(|_| paragraph(None))
        .collect::<Vec<_>>()
        .join(sep)
}

/// Generate a character (for backwards compatibility)
pub fn character() -> String {
    let config = crate::config::FakerConfig::current();
    config.rand_char(&crate::base::ALPHANUMERIC).to_string()
}

/// Generate characters (for backwards compatibility)
pub fn characters(num: usize) -> String {
    let config = crate::config::FakerConfig::current();
    (0..num)
        .map(|_| config.rand_char(&crate::base::ALPHANUMERIC))
        .collect()
}

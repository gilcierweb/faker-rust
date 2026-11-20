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

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `lorem.multibyte`.
pub fn multibyte() -> String {
    parity_pick("lorem.multibyte")
}
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: lorem paragraph truncated to a character count.
pub fn paragraph_by_chars(number: usize, supplemental: bool) -> String {
    let mut text = paragraph(None);
    while text.len() < number {
        text.push(' ');
        text.push_str(&paragraph(None));
    }
    if supplemental {
        text.push(' ');
        text.push_str(&crate::hipster::paragraph());
    }
    let end = number.saturating_sub(1).min(text.len());
    format!("{}.", &text[..end])
}

/// Gem-compatible generator: a lorem ipsum question.
pub fn question(
    word_count: usize,
    supplemental: bool,
    random_words_to_add: usize,
    exclude_words: Option<&str>,
) -> String {
    let config = crate::config::FakerConfig::current();
    let count = word_count + config.rand_usize(random_words_to_add.saturating_add(1));
    let mut chosen: Vec<String> = (0..count).map(|_| word()).collect();
    if let Some(excluded) = exclude_words {
        chosen.retain(|w| !excluded.split(',').any(|part| w == part.trim()));
    }
    if supplemental {
        chosen.extend((0..count).map(|_| crate::hipster::word()));
    }
    let mut text = chosen.join(" ");
    if let Some(first) = text.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    let mark = crate::locale::fetch_locale("lorem.punctuation.question_mark", "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_else(|| "?".to_string());
    format!("{text}{mark}")
}

/// Gem-compatible generator: a list of lorem ipsum questions.
pub fn questions(number: usize, supplemental: bool) -> Vec<String> {
    (0..number)
        .map(|_| question(3, supplemental, 0, None))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_respects_requested_count() {
        assert_eq!(words(5).split_whitespace().count(), 5);
        assert!(words(0).is_empty());
    }

    #[test]
    fn sentence_is_capitalized_and_terminated() {
        let s = sentence(Some(6));
        assert!(s.ends_with('.'));
        assert_eq!(s.split_whitespace().count(), 6);
        assert!(s.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn characters_respects_requested_length() {
        assert_eq!(characters(12).chars().count(), 12);
    }
}

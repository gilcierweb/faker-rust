//! HTML generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random HTML tag
pub fn tag() -> String {
    fetch_locale("html.tags", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TAGS).to_string())
}

/// Generate a random HTML attribute
pub fn attribute() -> String {
    fetch_locale("html.attributes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ATTRIBUTES).to_string())
}

/// Generate a random HTML color
pub fn color() -> String {
    let colors = vec![
        "#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#FF00FF", "#00FFFF", "#000000", "#FFFFFF",
        "#808080", "#FFA500", "#800080", "#008000",
    ];
    sample(&colors).to_string()
}

/// Generate a random HTML entity
pub fn entity() -> String {
    fetch_locale("html.entities", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ENTITIES).to_string())
}

// Fallback data
const FALLBACK_TAGS: &[&str] = &[
    "div", "span", "p", "a", "img", "ul", "ol", "li", "h1", "h2", "h3", "h4", "h5", "h6", "table",
    "tr", "td", "th", "form", "input", "button", "select", "option", "textarea", "label", "header",
    "footer", "nav", "section", "article", "aside", "main", "canvas", "video", "audio",
];

const FALLBACK_ATTRIBUTES: &[&str] = &[
    "class",
    "id",
    "style",
    "href",
    "src",
    "alt",
    "title",
    "type",
    "name",
    "value",
    "placeholder",
    "disabled",
    "checked",
    "selected",
    "readonly",
    "required",
    "multiple",
    "autofocus",
    "autocomplete",
];

const FALLBACK_ENTITIES: &[&str] = &[
    "&amp;", "&lt;", "&gt;", "&quot;", "&apos;", "&nbsp;", "&copy;", "&reg;", "&trade;", "&euro;",
    "&pound;", "&yen;",
];

// --- gem parity generators (algorithmic) ---

fn capitalize(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    if let Some(first) = chars.first_mut() {
        *first = first.to_uppercase().next().unwrap_or(*first);
    }
    chars.into_iter().collect()
}

fn list(tag: &str) -> String {
    let count = crate::config::FakerConfig::current().rand_range(1, 11) as usize;
    let items: Vec<String> = (0..count)
        .map(|_| format!("<li>{}</li>", crate::lorem::sentence(Some(1))))
        .collect();
    format!("<{tag}>\n{}\n</{tag}>", items.join("\n"))
}

fn table_row(tag: &str, cell_count: usize) -> String {
    let mut row = "<tr>\n".to_string();
    for _ in 0..cell_count {
        row.push_str(&format!("<{tag}>{}</{tag}>\n", crate::lorem::word()));
    }
    row.push_str("</tr>\n");
    row
}

/// Gem-compatible generator: HTML heading element.
pub fn heading() -> String {
    let level = crate::config::FakerConfig::current().rand_range(1, 7);
    let title = capitalize(&crate::lorem::word());
    format!("<h{level}>{title}</h{level}>")
}

/// Gem-compatible generator: HTML paragraph wrapping generated text.
pub fn paragraph(
    sentence_count: usize,
    supplemental: bool,
    random_sentences_to_add: usize,
    exclude_words: Option<&str>,
) -> String {
    let config = crate::config::FakerConfig::current();
    let extra = if random_sentences_to_add == 0 {
        0
    } else {
        config.rand_usize(random_sentences_to_add)
    };
    let mut body = crate::lorem::paragraph(Some(sentence_count + extra));
    if supplemental {
        body.push(' ');
        body.push_str(&crate::hipster::sentence());
    }
    if let Some(excluded) = exclude_words {
        for word in excluded.split(',').map(str::trim).filter(|w| !w.is_empty()) {
            body = body.replace(word, "");
        }
    }
    format!("<p>{body}</p>")
}

/// Gem-compatible generator: HTML emphasis element.
pub fn emphasis() -> String {
    format!("<em>{}</em>", crate::lorem::sentence(Some(3)))
}

/// Gem-compatible generator: HTML ordered list.
pub fn ordered_list() -> String {
    list("ol")
}

/// Gem-compatible generator: HTML unordered list.
pub fn unordered_list() -> String {
    list("ul")
}

/// Gem-compatible generator: HTML code element.
pub fn code() -> String {
    format!("<code>{}</code>", crate::lorem::sentence(Some(1)))
}

/// Gem-compatible generator: HTML table with header, body and footer rows.
pub fn table() -> String {
    let header = table_row("th", 3);
    let footer = table_row("td", 3);
    let body: Vec<String> = (0..3).map(|_| table_row("td", 3)).collect();
    format!(
        "<table>\n<thead>\n{header}</thead>\n<tbody>\n{}\n</tbody>\n<tfoot>\n{footer}</tfoot>\n</table>",
        body.join("\n")
    )
}

/// Gem-compatible generator: HTML script tag with a random source URL.
pub fn script() -> String {
    format!(
        "<script src=\"{}.js\"></script>",
        crate::internet::url(None, None, None)
    )
}

/// Gem-compatible generator: HTML link tag with a random stylesheet URL.
pub fn link(rel: &str) -> String {
    format!(
        "<link rel=\"{rel}\" href=\"{}.css\">",
        crate::internet::url(None, None, None)
    )
}

/// Gem-compatible generator: HTML element with generated attributes.
pub fn element(tag: &str, content: &str) -> String {
    let class = crate::lorem::word();
    let onclick = format!("{}()", crate::lorem::word());
    format!("<{tag} class=\"{class}\" onclick=\"{onclick}\">{content}</{tag}>")
}

/// Gem-compatible generator: one random HTML generator, excluding named ones.
pub fn random(exclude: &[&str]) -> String {
    let names = [
        "code",
        "element",
        "emphasis",
        "heading",
        "link",
        "ordered_list",
        "paragraph",
        "script",
        "table",
        "unordered_list",
    ];
    let choices: Vec<usize> = (0..names.len())
        .filter(|index| !exclude.contains(&names[*index]))
        .collect();
    if choices.is_empty() {
        return code();
    }
    match crate::base::sample(&choices) {
        0 => code(),
        1 => element("div", &crate::lorem::sentence(Some(3))),
        2 => emphasis(),
        3 => heading(),
        4 => link("stylesheet"),
        5 => ordered_list(),
        6 => paragraph(3, false, 0, None),
        7 => script(),
        8 => table(),
        _ => unordered_list(),
    }
}

/// Gem-compatible generator: heading followed by repeated paragraph and element blocks.
pub fn sandwich(sentences: usize, repeat: usize) -> String {
    let mut blocks = vec![heading()];
    for _ in 0..repeat {
        blocks.push(paragraph(sentences, false, 0, None));
        blocks.push(random(&["script", "link"]));
    }
    blocks.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag() {
        assert!(!tag().is_empty());
    }

    #[test]
    fn test_attribute() {
        assert!(!attribute().is_empty());
    }

    #[test]
    fn test_color() {
        assert!(!color().is_empty());
    }

    #[test]
    fn test_entity() {
        assert!(!entity().is_empty());
    }
}

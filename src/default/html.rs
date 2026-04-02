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
        "#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#FF00FF", "#00FFFF",
        "#000000", "#FFFFFF", "#808080", "#FFA500", "#800080", "#008000",
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
    "div", "span", "p", "a", "img", "ul", "ol", "li", "h1", "h2", "h3",
    "h4", "h5", "h6", "table", "tr", "td", "th", "form", "input", "button",
    "select", "option", "textarea", "label", "header", "footer", "nav",
    "section", "article", "aside", "main", "canvas", "video", "audio",
];

const FALLBACK_ATTRIBUTES: &[&str] = &[
    "class", "id", "style", "href", "src", "alt", "title", "type",
    "name", "value", "placeholder", "disabled", "checked", "selected",
    "readonly", "required", "multiple", "autofocus", "autocomplete",
];

const FALLBACK_ENTITIES: &[&str] = &[
    "&amp;", "&lt;", "&gt;", "&quot;", "&apos;", "&nbsp;",
    "&copy;", "&reg;", "&trade;", "&euro;", "&pound;", "&yen;",
];

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

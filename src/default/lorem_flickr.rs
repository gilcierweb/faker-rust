//! Lorem Flickr generator - placeholder image URLs

use crate::config::FakerConfig;

/// Generate a random Lorem Flickr image URL
pub fn image() -> String {
    let config = FakerConfig::current();
    let width = config.rand_range(300, 800);
    let height = config.rand_range(300, 800);
    let keywords = vec![
        "abstract",
        "animals",
        "business",
        "cats",
        "city",
        "food",
        "nightlife",
        "fashion",
        "people",
        "nature",
        "sports",
        "technics",
        "transport",
        "technics",
        "abstract",
        "people",
    ];
    let keyword = keywords[config.rand_usize(keywords.len())];

    format!("https://loremflickr.com/{}/{}/{}", width, height, keyword)
}

/// Generate a Lorem Flickr URL with grayscale
pub fn grayscale() -> String {
    format!("{}/g", image())
}

/// Generate a Lorem Flickr URL with specific dimensions
pub fn sized(width: u32, height: u32) -> String {
    let config = FakerConfig::current();
    let keywords = ["abstract", "animals", "business", "cats", "city", "food"];
    let keyword = keywords[config.rand_usize(keywords.len())];

    format!("https://loremflickr.com/{}/{}/{}", width, height, keyword)
}
// --- gem parity generators (algorithmic) ---
fn build_url(size: &str, format: &str, search_terms: &[String], match_all: bool) -> String {
    let mut parts = vec!["https://loremflickr.com".to_string(), format.to_string()];
    if let Some((width, height)) = size.split_once('x') {
        parts.push(width.to_string());
        parts.push(height.to_string());
    }
    parts.push(search_terms.join(","));
    if match_all {
        parts.push("all".to_string());
    }
    parts.join("/")
}

/// Gem-compatible generator: colorized loremflickr image URL.
pub fn colorized_image(
    size: &str,
    color: &str,
    search_terms: &[String],
    match_all: bool,
) -> String {
    build_url(size, color, search_terms, match_all)
}
/// Gem-compatible generator: grayscale loremflickr image URL.
pub fn grayscale_image(size: &str, search_terms: &[String], match_all: bool) -> String {
    build_url(size, "g", search_terms, match_all)
}
/// Gem-compatible generator: pixelated loremflickr image URL.
pub fn pixelated_image(size: &str, search_terms: &[String], match_all: bool) -> String {
    build_url(size, "p", search_terms, match_all)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let url = image();
        assert!(url.starts_with("https://loremflickr.com/"));
    }

    #[test]
    fn test_grayscale() {
        let url = grayscale();
        assert!(url.contains("/g"));
    }

    #[test]
    fn test_sized() {
        let url = sized(100, 200);
        assert!(url.contains("/100/200/"));
    }
}

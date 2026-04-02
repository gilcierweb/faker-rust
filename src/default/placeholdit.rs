//! Placehold.it (placeholder image) generator

use crate::config::FakerConfig;

/// Generate a random Placehold.it URL
pub fn image() -> String {
    let config = FakerConfig::current();
    let width = config.rand_range(100, 800);
    let height = config.rand_range(100, 800);
    
    format!("https://placehold.it/{}x{}", width, height)
}

/// Generate a Placehold.it URL with custom text
pub fn image_with_text(text: &str) -> String {
    let config = FakerConfig::current();
    let width = config.rand_range(100, 800);
    let height = config.rand_range(100, 800);
    
    format!("https://placehold.it/{}x{}?text={}", width, height, text.replace(' ', "+"))
}

/// Generate a Placehold.it URL with specific dimensions
pub fn sized(width: u32, height: u32) -> String {
    format!("https://placehold.it/{}x{}", width, height)
}

/// Generate a colored placeholder
pub fn colored(background: &str, text: &str) -> String {
    let config = FakerConfig::current();
    let width = config.rand_range(100, 800);
    let height = config.rand_range(100, 800);
    
    format!("https://placehold.it/{}x{}/{}/{}", width, height, background, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let url = image();
        assert!(url.starts_with("https://placehold.it/"));
    }

    #[test]
    fn test_image_with_text() {
        let url = image_with_text("Hello World");
        assert!(url.contains("text="));
    }

    #[test]
    fn test_sized() {
        let url = sized(300, 200);
        assert_eq!(url, "https://placehold.it/300x200");
    }

    #[test]
    fn test_colored() {
        let url = colored("ff0000", "ffffff");
        assert!(url.contains("/ff0000/ffffff"));
    }
}

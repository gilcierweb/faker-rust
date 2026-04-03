//! Lorem Flickr generator - placeholder image URLs

use crate::config::FakerConfig;

/// Generate a random Lorem Flickr image URL
pub fn image() -> String {
    let config = FakerConfig::current();
    let width = config.rand_range(300, 800);
    let height = config.rand_range(300, 800);
    let keywords = vec![
        "abstract", "animals", "business", "cats", "city", "food",
        "nightlife", "fashion", "people", "nature", "sports", "technics",
        "transport", "technics", "abstract", "people",
    ];
    let keyword = keywords[(config.rand_range(0, keywords.len() as u32)) as usize];
    
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
    let keyword = keywords[(config.rand_range(0, keywords.len() as u32)) as usize];
    
    format!("https://loremflickr.com/{}/{}/{}", width, height, keyword)
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

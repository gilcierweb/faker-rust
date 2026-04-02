//! Driving licence generator

use crate::base::sample;
use crate::locale::fetch_locale;
use crate::config::FakerConfig;

/// Generate a random UK driving licence number
pub fn uk() -> String {
    let config = FakerConfig::current();
    let surname_part = sample(&["SMITH", "JONES", "BROWN", "TAYLOR", "WILSON"]);
    let year = config.rand_range(1950, 2005);
    let month = config.rand_range(1, 13);
    let day = config.rand_range(1, 29);
    let initials = sample(&["AB", "CD", "EF", "GH", "IJ"]);
    let numbers = format!("{:02}{:02}{:02}", 
        config.rand_range(0, 100),
        config.rand_range(0, 100),
        config.rand_range(0, 100)
    );
    
    format!("{}{:02}{:02}{:02}{}9AA", 
        &surname_part[..5.min(surname_part.len())],
        year % 100,
        month,
        day,
        initials
    )
}

/// Generate a random US driver's license
pub fn usa() -> String {
    let config = FakerConfig::current();
    let letters: String = (0..2)
        .map(|_| config.rand_char(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']))
        .collect();
    let numbers: String = (0..6)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    format!("{}{}", letters, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uk() {
        let licence = uk();
        assert!(!licence.is_empty());
        assert!(licence.len() >= 16);
    }

    #[test]
    fn test_usa() {
        let licence = usa();
        assert!(!licence.is_empty());
        assert_eq!(licence.len(), 8);
    }
}

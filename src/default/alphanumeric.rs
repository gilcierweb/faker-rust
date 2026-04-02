//! Alphanumeric generator

use crate::config::FakerConfig;

/// Generate a random alphanumeric string
pub fn alphanumeric(length: usize) -> String {
    let config = FakerConfig::current();
    let chars: Vec<char> = (0..length)
        .map(|_| {
            let choice = config.rand_range(0, 3);
            match choice {
                0 => config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
                1 => config.rand_char(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']),
                _ => config.rand_char(&['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']),
            }
        })
        .collect();
    chars.into_iter().collect()
}

/// Generate a random alpha string (letters only)
pub fn alpha(length: usize) -> String {
    let config = FakerConfig::current();
    let chars: Vec<char> = (0..length)
        .map(|_| {
            let choice = config.rand_bool();
            if choice {
                config.rand_char(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'])
            } else {
                config.rand_char(&['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'])
            }
        })
        .collect();
    chars.into_iter().collect()
}

/// Generate a random numeric string
pub fn numeric(length: usize) -> String {
    let config = FakerConfig::current();
    let chars: Vec<char> = (0..length)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    chars.into_iter().collect()
}

/// Generate a random hex string
pub fn hex(length: usize) -> String {
    let config = FakerConfig::current();
    let chars: Vec<char> = (0..length)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']))
        .collect();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphanumeric() {
        let s = alphanumeric(10);
        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_alpha() {
        let s = alpha(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_alphabetic()));
    }

    #[test]
    fn test_numeric() {
        let s = numeric(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_hex() {
        let s = hex(8);
        assert_eq!(s.len(), 8);
        assert!(s.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

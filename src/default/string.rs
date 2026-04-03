//! String generator


/// Generate a random string
pub fn random(length: usize) -> String {
    let chars: Vec<char> = (0..length)
        .map(|_| {
            let alphabet = "abcdefghijklmnopqrstuvwxyz";
            let idx = (rand::random::<u8>() as usize) % alphabet.len();
            alphabet.chars().nth(idx).unwrap()
        })
        .collect();
    chars.into_iter().collect()
}

/// Generate a random uppercase string
pub fn uppercase(length: usize) -> String {
    random(length).to_uppercase()
}

/// Generate a random lowercase string
pub fn lowercase(length: usize) -> String {
    random(length).to_lowercase()
}

/// Generate a random alphanumeric string
pub fn alphanumeric(length: usize) -> String {
    let chars: Vec<char> = (0..length)
        .map(|_| {
            let alphabet = "abcdefghijklmnopqrstuvwxyz0123456789";
            let idx = (rand::random::<u8>() as usize) % alphabet.len();
            alphabet.chars().nth(idx).unwrap()
        })
        .collect();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let s = random(10);
        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_uppercase() {
        let s = uppercase(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_uppercase()));
    }

    #[test]
    fn test_lowercase() {
        let s = lowercase(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_lowercase()));
    }

    #[test]
    fn test_alphanumeric() {
        let s = alphanumeric(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_alphanumeric()));
    }
}

//! Base utilities for Faker generators

use crate::config::FakerConfig;

/// Uppercase letters
pub const U_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Lowercase letters
pub const L_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// All letters
pub const LETTERS: [char; 52] = {
    let mut letters = [' '; 52];
    let mut i = 0;
    while i < 26 {
        letters[i] = U_LETTERS[i];
        letters[i + 26] = L_LETTERS[i];
        i += 1;
    }
    letters
};

/// Digits
pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Hex digits uppercase
pub const HEX_UPPER: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Hex digits lowercase
pub const HEX_LOWER: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Alphanumeric characters
pub const ALPHANUMERIC: [char; 62] = {
    let mut chars = [' '; 62];
    let mut i = 0;
    while i < 26 {
        chars[i] = U_LETTERS[i];
        chars[i + 26] = L_LETTERS[i];
        i += 1;
    }
    // Add digits after letters
    let mut j = 0;
    while j < 10 {
        chars[52 + j] = DIGITS[j];
        j += 1;
    }
    chars
};

/// Replace `#` with random digit, `?` with random letter
/// By default, the first `#` is replaced with 1-9 (no leading zero)
pub fn numerify(template: &str) -> String {
    let config = FakerConfig::current();
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '#' {
            // Check if this is the first # (to avoid leading zeros)
            let prev_is_hash = result.ends_with('#');
            if !prev_is_hash {
                // Check if the previous char in template was # (we're in a sequence)
                let is_first_in_sequence = result.is_empty() || result.chars().last() != Some('#');

                if is_first_in_sequence && !chars.peek().is_none() {
                    // Use 1-9 for first digit in sequence to avoid leading zero
                    result.push(config.rand_char(&['1', '2', '3', '4', '5', '6', '7', '8', '9']));
                } else {
                    result.push(config.rand_char(&DIGITS));
                }
            } else {
                result.push(config.rand_char(&DIGITS));
            }
        } else {
            result.push(c);
        }
    }

    // Clean up consecutive # markers
    while result.contains("##") {
        result = result.replace("##", "#");
    }

    result
}

/// Replace `?` with random letter
pub fn letterify(template: &str) -> String {
    let config = FakerConfig::current();
    template.replace('?', &config.rand_char(&LETTERS).to_string())
}

/// Replace both `#` and `?` in the template
pub fn bothify(template: &str) -> String {
    letterify(&numerify(template))
}

/// Sample a random element from a slice
pub fn sample<T: Clone>(items: &[T]) -> T {
    FakerConfig::current().sample(items)
}

/// Sample multiple random elements from a slice (with replacement)
pub fn sample_many<T: Clone>(items: &[T], count: usize) -> Vec<T> {
    let config = FakerConfig::current();
    (0..count).map(|_| config.sample(items)).collect()
}

/// Generate a random number in range [min, max]
pub fn rand_in_range(min: u32, max: u32) -> u32 {
    if max < min {
        return min;
    }
    FakerConfig::current().rand_range(min, max)
}

/// Parse a template with interpolations like "#{first_name} #{last_name}"
pub fn parse(template: &str) -> String {
    let mut result = template.to_string();

    // Simple parsing - in the real implementation this would be more sophisticated
    // For now, just handle basic patterns
    while result.contains("#{") {
        if let Some(start) = result.find("#{") {
            if let Some(end) = result[start..].find('}') {
                let placeholder = &result[start + 2..start + end];
                let replacement = fetch_value(placeholder);
                result = format!(
                    "{}{}{}",
                    &result[..start],
                    replacement,
                    &result[start + end + 1..]
                );
            } else {
                break;
            }
        } else {
            break;
        }
    }

    result
}

fn fetch_value(key: &str) -> String {
    // Map placeholder keys to actual generator calls
    // For now, return key as-is - the t! macro from rust-i18n will handle the rest
    key.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerify() {
        let result = numerify("###");
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_letterify() {
        let result = letterify("???");
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_alphabetic()));
    }

    #[test]
    fn test_bothify() {
        let result = bothify("???###");
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_sample() {
        let items = vec![1, 2, 3, 4, 5];
        let result = sample(&items);
        assert!(items.contains(&result));
    }

    #[test]
    fn test_rand_in_range() {
        let result = rand_in_range(1, 10);
        assert!(result >= 1 && result <= 10);
    }
}

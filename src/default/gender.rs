//! Gender generator - generates random gender data

use crate::base::sample;

/// Generate a random gender type
pub fn gender() -> String {
    sample(&[
        "Male".to_string(),
        "Female".to_string(),
        "Non-binary".to_string(),
        "Gender fluid".to_string(),
        "Other".to_string(),
    ])
}

/// Generate a shorter gender type
pub fn binary_type() -> String {
    sample(&["Male".to_string(), "Female".to_string()])
}

/// Generate a short binary type
pub fn short_binary_type() -> String {
    sample(&["M".to_string(), "F".to_string()])
}

/// Generate a type
pub fn r#type() -> String {
    gender()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_types_are_from_expected_sets() {
        for _ in 0..20 {
            assert!(["Male", "Female"].contains(&binary_type().as_str()));
            assert!(["M", "F"].contains(&short_binary_type().as_str()));
        }
    }
}

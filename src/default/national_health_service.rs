//! National Health Service (NHS) generator for UK

use crate::config::FakerConfig;

/// Generate a random NHS number
pub fn nhs_number() -> String {
    let config = FakerConfig::current();
    let numbers: String = (0..10)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    numbers
}

/// Generate a random NHS practitioner number
pub fn practitioner() -> String {
    let config = FakerConfig::current();
    let prefix = config.rand_char(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'T', 'U',
        'V', 'W', 'X', 'Y',
    ]);
    let numbers: String = (0..6)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    format!("{}{}", prefix, numbers)
}

/// Generate a random NHS hospital name
pub fn hospital() -> String {
    let hospitals = [
        "St. Thomas' Hospital",
        "Guy's Hospital",
        "Royal Free Hospital",
        "University College Hospital",
        "St. George's Hospital",
        "King's College Hospital",
        "Royal London Hospital",
        "Manchester Royal Infirmary",
        "Leeds General Infirmary",
    ];
    sample(&hospitals).to_string()
}

use crate::base::sample;
// --- gem parity generators (algorithmic) ---
fn check_digit(base: u32) -> u32 {
    let digits: Vec<u32> = format!("{base:09}")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let weights = [10_u32, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits.iter().zip(weights).map(|(d, w)| d * w).sum();
    (11 - sum % 11) % 11
}

/// Gem-compatible generator: formatted British NHS number.
pub fn british_number() -> String {
    let config = crate::config::FakerConfig::current();
    let mut base = 999_000_001 + config.rand_u32(999_998);
    if check_digit(base) == 10 {
        base -= 1;
    }
    let raw = format!("{base}{}", check_digit(base));
    format!("{} {} {}", &raw[..3], &raw[3..6], &raw[6..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nhs_number() {
        let num = nhs_number();
        assert_eq!(num.len(), 10);
        assert!(num.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_practitioner() {
        let prac = practitioner();
        assert_eq!(prac.len(), 7);
    }

    #[test]
    fn test_hospital() {
        assert!(!hospital().is_empty());
    }
}

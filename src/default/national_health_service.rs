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
    let prefix = config.rand_char(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'T', 'U', 'V', 'W', 'X', 'Y']);
    let numbers: String = (0..6)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    format!("{}{}", prefix, numbers)
}

/// Generate a random NHS hospital name
pub fn hospital() -> String {
    let hospitals = [
        "St. Thomas' Hospital", "Guy's Hospital", "Royal Free Hospital",
        "University College Hospital", "St. George's Hospital", "King's College Hospital",
        "Royal London Hospital", "Manchester Royal Infirmary", "Leeds General Infirmary",
    ];
    sample(&hospitals).to_string()
}

use crate::base::sample;

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

//! Invoice generator

use crate::config::FakerConfig;

/// Generate a random invoice reference
pub fn reference() -> String {
    let config = FakerConfig::current();
    let prefix = config.rand_char(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]);
    let numbers: String = (0..8)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    format!("{}{}", prefix, numbers)
}

/// Generate a random invoice amount
pub fn amount() -> String {
    let config = FakerConfig::current();
    let dollars = config.rand_range(10, 10000);
    let cents = config.rand_range(0, 100);
    format!("{}.{:02}", dollars, cents)
}

/// Generate a random invoice line item description
pub fn line_item() -> String {
    let items = [
        "Consulting Services",
        "Software License",
        "Hardware Maintenance",
        "Cloud Hosting",
        "Training Session",
        "Support Contract",
        "Professional Services",
        "Implementation Fee",
        "Setup Fee",
    ];
    sample(&items).to_string()
}

use crate::base::sample;
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: a rounded amount within `from..=to`.
pub fn amount_between(from: f64, to: f64) -> f64 {
    let (low, high) = if from <= to { (from, to) } else { (to, from) };
    let value = low + FakerConfig::current().rand_f64() * (high - low);
    (value * 100.0).round() / 100.0
}

/// Gem-compatible generator: creditor reference with an IBAN check digit.
pub fn creditor_reference(ref_value: &str) -> String {
    let reference = if ref_value.is_empty() {
        reference()
    } else {
        ref_value.to_string()
    };
    let digits: String = format!("{reference}RF00")
        .to_uppercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                (c as u32 - 55).to_string()
            } else {
                c.to_string()
            }
        })
        .collect();
    let number: u128 = digits.parse().unwrap_or(0);
    let checksum = (1 + 97 - number % 97) % 97;
    format!("RF{checksum:02}{reference}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference() {
        let ref_num = reference();
        assert_eq!(ref_num.len(), 9);
        assert!(ref_num.chars().next().unwrap().is_alphabetic());
    }

    #[test]
    fn test_amount() {
        let amt = amount();
        assert!(amt.contains('.'));
    }

    #[test]
    fn test_line_item() {
        assert!(!line_item().is_empty());
    }
}

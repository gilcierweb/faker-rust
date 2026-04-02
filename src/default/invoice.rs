//! Invoice generator

use crate::config::FakerConfig;

/// Generate a random invoice reference
pub fn reference() -> String {
    let config = FakerConfig::current();
    let prefix = config.rand_char(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']);
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
        "Consulting Services", "Software License", "Hardware Maintenance",
        "Cloud Hosting", "Training Session", "Support Contract",
        "Professional Services", "Implementation Fee", "Setup Fee",
    ];
    sample(&items).to_string()
}

use crate::base::sample;

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

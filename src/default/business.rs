//! Business generator - generates random business-related data

use crate::base::{bothify, sample};
use crate::locale::fetch_locale;

/// Generate a random credit card number
pub fn credit_card_number() -> String {
    fetch_locale("business.credit_card_numbers", "en")
        .map(|v| bothify(&sample(&v)))
        .unwrap_or_else(|| bothify("####-####-####-####"))
}

/// Generate a random credit card type
pub fn credit_card_type() -> String {
    fetch_locale("business.credit_card_types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Visa".to_string())
}

/// Generate a random credit card expiry date
pub fn credit_card_expiry_date() -> String {
    fetch_locale("business.credit_card_expiry_dates", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "2028-12-31".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_number() {
        let n = credit_card_number();
        assert!(!n.is_empty());
    }
}

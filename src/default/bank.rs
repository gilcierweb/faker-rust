//! Bank generator - generates random bank names and details

use crate::base::{bothify, sample};
use crate::locale::fetch_locale;

/// Generate a random bank name
pub fn name() -> String {
    fetch_locale("bank.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Chase Bank".to_string())
}

/// Generate a random IBAN (GB format with valid check digits)
pub fn iban() -> String {
    let config = crate::config::FakerConfig::current();
    let bank_code: String = (0..4)
        .map(|_| config.rand_char(&crate::base::U_LETTERS))
        .collect();
    let sort_code: String = (0..6)
        .map(|_| config.rand_char(&crate::base::DIGITS))
        .collect();
    let account: String = (0..8)
        .map(|_| config.rand_char(&crate::base::DIGITS))
        .collect();
    let bban = format!("{}{}{}", bank_code, sort_code, account);

    // Compute the two check digits: rearrange to BBAN + country code + "00"
    // (placeholders for the check digits), expand every letter A-Z to 10-35,
    // then check = 98 - (mod 97).
    let mut numeric = String::with_capacity(bban.len() * 2 + 6);
    for c in bban.chars().chain("GB00".chars()) {
        if c.is_ascii_digit() {
            numeric.push(c);
        } else {
            numeric.push_str(&(c as u32 - b'A' as u32 + 10).to_string());
        }
    }
    let mut remainder: u64 = 0;
    for c in numeric.chars() {
        remainder = (remainder * 10 + (c as u64 - b'0' as u64)) % 97;
    }
    let check = 98u32 - remainder as u32;

    format!("GB{:02}{}", check, bban)
}

/// Get a random IBAN country code
pub fn iban_country_code() -> String {
    fetch_locale("bank.iban_country_code", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "GB".to_string())
}

/// Generate a random SWIFT/BIC code
pub fn swift_bic() -> String {
    let config = crate::config::FakerConfig::current();
    let bank_code: String = (0..4)
        .map(|_| config.rand_char(&crate::base::U_LETTERS))
        .collect();
    let country: String = (0..2)
        .map(|_| config.rand_char(&crate::base::U_LETTERS))
        .collect();
    let location: String = (0..2)
        .map(|_| config.rand_char(&crate::base::U_LETTERS))
        .collect();
    format!("{}{}{}XXX", bank_code, country, location)
}

/// Generate a random account number
pub fn account_number() -> String {
    bothify("##########")
}

/// Generate a routing number (US)
pub fn routing_number() -> String {
    bothify("#########")
}

/// Generate a routing number with format
pub fn routing_number_with_format() -> String {
    bothify("###-###-###")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Standard IBAN mod-97 validation: rotate country/check digits to the
    /// end, expand letters (A=10..Z=35), then require remainder == 1.
    fn iban_mod97_valid(iban: &str) -> bool {
        let rotated = format!("{}{}", &iban[4..], &iban[..4]);
        let mut remainder: u32 = 0;
        for c in rotated.chars() {
            if c.is_ascii_digit() {
                remainder = (remainder * 10 + (c as u32 - '0' as u32)) % 97;
            } else if c.is_ascii_uppercase() {
                let value = c as u32 - 'A' as u32 + 10;
                remainder = (remainder * 100 + value) % 97;
            } else {
                return false;
            }
        }
        remainder == 1
    }

    #[test]
    fn iban_is_structurally_valid_gb_with_correct_check_digits() {
        for _ in 0..200 {
            let iban = iban();
            assert!(iban.starts_with("GB"), "bad prefix: {iban}");
            assert_eq!(iban.len(), 22, "bad length: {iban}");
            assert!(iban_mod97_valid(&iban), "invalid check digits: {iban}");
        }
    }

    #[test]
    fn swift_bic_is_eleven_uppercase_letters() {
        let bic = swift_bic();
        assert_eq!(bic.len(), 11);
        assert!(
            bic.chars().all(|c| c.is_ascii_uppercase()),
            "bad bic: {bic}"
        );
        assert!(bic.ends_with("XXX"));
    }

    #[test]
    fn account_and_routing_numbers_have_expected_shape() {
        assert_eq!(account_number().len(), 10);
        assert_eq!(routing_number().len(), 9);
        let formatted = routing_number_with_format();
        assert_eq!(formatted.len(), 11);
        assert_eq!(formatted.as_bytes()[3], b'-');
        assert_eq!(formatted.as_bytes()[7], b'-');
    }
}

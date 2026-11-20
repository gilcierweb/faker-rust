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
    let bank_code: String = (0..4).map(|_| config.rand_char(&crate::base::U_LETTERS)).collect();
    let sort_code: String = (0..6).map(|_| config.rand_char(&crate::base::DIGITS)).collect();
    let account: String = (0..8).map(|_| config.rand_char(&crate::base::DIGITS)).collect();
    let bban = format!("{}{}{}", bank_code, sort_code, account);

    // Compute the two check digits: move the country code to the end, convert
    // letters A-Z to 10-35, then check = 98 - (that number mod 97).
    let mut numeric = String::with_capacity(bban.len() + 4);
    for c in bban.chars() {
        numeric.push(c);
    }
    for c in "GB".chars() {
        numeric.push_str(&(c as u32 - b'A' as u32 + 10).to_string());
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

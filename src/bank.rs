//! Bank generator - generates random bank names and details

use crate::base::{bothify, sample};
use crate::locale::fetch_locale;

/// Generate a random bank name
pub fn name() -> String {
    fetch_locale("bank.name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Chase Bank".to_string())
}

/// Generate a random IBAN
pub fn iban() -> String {
    let config = crate::config::FakerConfig::current();
    let country_code = iban_country_code();
    let length = config.rand_range(15, 32) as usize;
    let chars: String = (0..length)
        .map(|_| config.rand_char(&crate::base::DIGITS))
        .collect();
    format!("{} {}", country_code, chars)
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

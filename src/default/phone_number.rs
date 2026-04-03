//! Phone number generator - generates random phone numbers

use crate::base::{bothify, sample};
use crate::config::FakerConfig;
use crate::locale::fetch_locale;

/// Generate a random phone number
pub fn phone_number() -> String {
    let formats = ["(###) ###-####",
        "###-###-####",
        "(###) ###-####",
        "###.###.####",
        "(###) ###-####",
        "##########"];
    let config = FakerConfig::current();
    let format = formats[config.rand_usize(formats.len())];
    bothify(format)
}

/// Generate a random phone number with country code
pub fn phone_number_with_country_code() -> String {
    let country_code = fetch_locale("phone_number.country_code", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "1".to_string());
    format!("{} {}", country_code, phone_number())
}

/// Generate a random area code
pub fn area_code() -> String {
    fetch_locale("phone_number.area_code", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "212".to_string())
}

/// Generate a random cell phone number
pub fn cell_phone() -> String {
    let formats = ["(###) ###-####",
        "###-###-####",
        "(###) ###-####",
        "##########",
        "+1 (###) ###-####"];
    let config = FakerConfig::current();
    let format = formats[config.rand_usize(formats.len())];
    bothify(format)
}

/// Generate a random cell phone number in E164 format
pub fn cell_phone_in_e164() -> String {
    format!(
        "+1{}",
        cell_phone().replace(|c: char| !c.is_ascii_digit(), "")
    )
}

/// Generate a random cell phone with country code
pub fn cell_phone_with_country_code() -> String {
    let country_code = fetch_locale("phone_number.country_code", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "1".to_string());
    format!("+{} {}", country_code, cell_phone())
}

/// Generate a random country code
pub fn country_code() -> String {
    fetch_locale("phone_number.country_code", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "1".to_string())
}

/// Generate a random extension
pub fn extension() -> String {
    let config = FakerConfig::current();
    let len = config.rand_range(3, 6) as usize;
    (0..len)
        .map(|_| config.rand_char(&crate::base::DIGITS))
        .collect()
}

/// Generate a random exchange code
pub fn exchange_code() -> String {
    let config = FakerConfig::current();
    format!(
        "{}{}{}",
        config.rand_char(&crate::base::DIGITS),
        config.rand_char(&crate::base::DIGITS),
        config.rand_char(&crate::base::DIGITS)
    )
}

/// Generate a random subscriber number
pub fn subscriber_number() -> String {
    bothify("####")
}

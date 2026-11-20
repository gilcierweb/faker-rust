//! Phone number generator - generates random phone numbers

use crate::base::{bothify, sample};
use crate::config::FakerConfig;
use crate::locale::fetch_locale;

/// Generate a random phone number
pub fn phone_number() -> String {
    let formats = [
        "(###) ###-####",
        "###-###-####",
        "(###) ###-####",
        "###.###.####",
        "(###) ###-####",
        "##########",
    ];
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
    let formats = [
        "(###) ###-####",
        "###-###-####",
        "(###) ###-####",
        "##########",
        "+1 (###) ###-####",
    ];
    let config = FakerConfig::current();
    let format = formats[config.rand_usize(formats.len())];
    bothify(format)
}

/// Generate a random cell phone number in E164 format (+1 followed by ten digits).
pub fn cell_phone_in_e164() -> String {
    // Use national formats only so the ten generated digits are the subscriber
    // number; a leading '1' in an area code must not be mistaken for a country
    // code and stripped. The country code is always prepended explicitly.
    let config = FakerConfig::current();
    let formats = [
        "###-###-####",
        "(###) ###-####",
        "##########",
        "###.###.####",
    ];
    let format = formats[config.rand_usize(formats.len())];
    let digits: String = bothify(format)
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    format!("+1{digits}")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e164_has_single_country_code_and_ten_digits() {
        for _ in 0..100 {
            let n = cell_phone_in_e164();
            assert!(n.starts_with("+1"), "bad prefix: {n}");
            assert_eq!(n.len(), 12, "bad e164: {n}");
            assert!(n[1..].chars().all(|c| c.is_ascii_digit()), "bad e164: {n}");
        }
    }

    #[test]
    fn area_code_is_three_digits() {
        let a = area_code();
        assert_eq!(a.len(), 3);
        assert!(a.chars().all(|c| c.is_ascii_digit()));
    }
}

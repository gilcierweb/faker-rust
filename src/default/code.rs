//! Code generator - generates random ISBN and EAN codes

use crate::base::bothify;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random ASIN code
pub fn asin() -> String {
    fetch_locale_with_context(
        "code.asin",
        &crate::config::FakerConfig::locale(),
        Some("code"),
    )
    .map(|v| sample_with_resolve(&v, Some("code")))
    .unwrap_or_else(|| bothify("B000#######"))
}

/// Generate a random ISBN-10 code
pub fn isbn() -> String {
    fetch_locale_with_context(
        "code.isbn",
        &crate::config::FakerConfig::locale(),
        Some("code"),
    )
    .map(|v| sample_with_resolve(&v, Some("code")))
    .unwrap_or_else(|| bothify("#########-#"))
}

/// Generate a random EAN code
pub fn ean() -> String {
    fetch_locale_with_context(
        "code.ean",
        &crate::config::FakerConfig::locale(),
        Some("code"),
    )
    .map(|v| sample_with_resolve(&v, Some("code")))
    .unwrap_or_else(|| bothify("#############"))
}
// --- gem parity generators (algorithmic) ---
fn digits(len: usize) -> String {
    let config = crate::config::FakerConfig::current();
    (0..len)
        .map(|_| char::from(b'0' + config.rand_u32(10) as u8))
        .collect()
}

/// Gem-compatible generator: 15-digit IMEI.
pub fn imei() -> String {
    let body = digits(14);
    let sum: u32 = body
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 0 {
                d
            } else {
                d * 2 / 10 + d * 2 % 10
            }
        })
        .sum();
    format!("{body}{}", (10 - sum % 10) % 10)
}

/// Gem-compatible generator: 10-digit National Provider Identifier.
pub fn npi() -> String {
    digits(10)
}

/// Gem-compatible generator: Singapore NRIC identifier.
pub fn nric(min_age: i32, max_age: i32) -> String {
    let current = time::OffsetDateTime::now_utc().year();
    let year = current
        - crate::config::FakerConfig::current()
            .rand_range(min_age.max(0) as u32, (max_age.max(min_age) + 1) as u32) as i32;
    let prefix = if year < 2000 { 'S' } else { 'T' };
    format!(
        "{prefix}{}{}",
        digits(7),
        crate::base::sample(&[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
        ])
    )
}

/// Gem-compatible generator: Chilean RUT identifier.
pub fn rut() -> String {
    let body = digits(8);
    let reversed: Vec<u32> = body.chars().rev().filter_map(|c| c.to_digit(10)).collect();
    let seq = [2_u32, 3, 4, 5, 6, 7];
    let sum: u32 = reversed
        .iter()
        .zip(seq.iter().cycle())
        .map(|(d, w)| d * w)
        .sum();
    let result = 11 - sum % 11;
    let verifier = match result {
        10 => "k".to_string(),
        11 => "0".to_string(),
        n => n.to_string(),
    };
    format!("{body}-{verifier}")
}

/// Gem-compatible generator: Canadian Social Insurance Number.
pub fn sin() -> String {
    digits(9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isbn() {
        assert!(!isbn().is_empty());
    }

    #[test]
    fn test_ean() {
        assert!(!ean().is_empty());
    }
}

//! Barcode generator - generates random barcode-related data

use crate::base::{bothify, sample};
use crate::locale::fetch_locale;

/// Generate a random EAN-8 barcode
pub fn ean_8() -> String {
    let pattern = fetch_locale("barcode.ean_8", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "#######".to_string());

    let base = bothify(&pattern);
    let check_digit = calculate_ean_check_digit(&base);
    format!("{}{}", base, check_digit)
}

/// Generate a random EAN-13 barcode
pub fn ean_13() -> String {
    let pattern = fetch_locale("barcode.ean_13", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "############".to_string());

    let base = bothify(&pattern);
    let check_digit = calculate_ean_check_digit(&base);
    format!("{}{}", base, check_digit)
}

/// Generate a random UPC-A barcode
pub fn upc_a() -> String {
    let pattern = fetch_locale("barcode.upc_a", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "###########".to_string());

    let base = bothify(&pattern);
    let check_digit = calculate_ean_check_digit(&base);
    format!("{}{}", base, check_digit)
}

/// Generate a random UPC-E barcode
pub fn upc_e() -> String {
    fetch_locale("barcode.upc_e", "en")
        .map(|v| bothify(&sample(&v)))
        .unwrap_or_else(|| bothify("0######"))
}

/// Generate a random ISBN barcode
pub fn isbn() -> String {
    fetch_locale("barcode.isbn", "en")
        .map(|v| bothify(&sample(&v)))
        .unwrap_or_else(|| bothify("978#########"))
}

/// Helper function to calculate EAN/UPC check digit
fn calculate_ean_check_digit(s: &str) -> u32 {
    let mut sum = 0;
    let len = s.len();

    for (i, c) in s.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            // Weights depend on position from the RIGHT (starting at 1):
            // positions odd from the right carry weight 3, even carry 1.
            // For a payload of length `len`, left index `i` has right-position
            // `len - i`, so weight 3 when `(len - 1 - i)` is even.
            let weight = if (len - 1 - i).is_multiple_of(2) {
                3
            } else {
                1
            };
            sum += digit * weight;
        }
    }

    (10 - (sum % 10)) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ean_8() {
        let b = ean_8();
        assert_eq!(b.len(), 8);
    }

    #[test]
    fn test_ean_13() {
        let b = ean_13();
        assert_eq!(b.len(), 13);
    }

    #[test]
    fn test_upc_a() {
        let b = upc_a();
        assert_eq!(b.len(), 12);
    }

    #[test]
    fn test_calculate_ean_check_digit_known_vectors() {
        assert_eq!(calculate_ean_check_digit("590123412345"), 7); // EAN-13
        assert_eq!(calculate_ean_check_digit("400638133393"), 1); // EAN-13
        assert_eq!(calculate_ean_check_digit("4017072"), 5); // EAN-8
    }
}

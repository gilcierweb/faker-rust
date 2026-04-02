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
            // Weights depend on position from RIGHT (starting at 1)
            // For EAN-13 (12 digits input): index 0 weight 1, 1 weight 3, 2 weight 1...
            // Basically: if (len - i) is even, weight is 3, else 1.
            let weight = if (len - i) % 2 == 0 { 3 } else { 1 };
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
}

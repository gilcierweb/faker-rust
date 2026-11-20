//! Chile RUT (Rol Único Tributario) generator

use crate::config::FakerConfig;

/// Generate a random Chilean RUT number
pub fn rut() -> String {
    let config = FakerConfig::current();
    let number = config.rand_range(1000000, 25000000);
    let dv = calculate_dv(number);
    format!("{}-{}", number, dv)
}

/// Calculate the verification digit for a RUT
fn calculate_dv(number: u32) -> char {
    let mut sum = 0;
    let mut multiplier = 2;
    let mut n = number;

    while n > 0 {
        sum += (n % 10) * multiplier;
        n /= 10;
        multiplier += 1;
        if multiplier > 7 {
            multiplier = 2;
        }
    }

    let remainder = sum % 11;
    let dv = 11 - remainder;

    match dv {
        11 => '0',
        10 => 'K',
        _ => std::char::from_digit(dv, 10).unwrap_or('0'),
    }
}
// --- gem parity generators (algorithmic) ---
fn dv_for(body: &str) -> String {
    let reversed: Vec<u32> = body.chars().rev().filter_map(|c| c.to_digit(10)).collect();
    let seq = [2_u32, 3, 4, 5, 6, 7];
    let sum: u32 = reversed
        .iter()
        .zip(seq.iter().cycle())
        .map(|(d, w)| d * w)
        .sum();
    let result = 11 - (sum % 11);
    match result {
        10 => "K".to_string(),
        11 => "0".to_string(),
        n => n.to_string(),
    }
}

/// Gem-compatible generator: Chilean RUT check digit.
pub fn check_digit(body: &str) -> String {
    dv_for(body)
}
/// Gem-compatible generator: alias for the RUT check digit.
pub fn dv(body: &str) -> String {
    dv_for(body)
}
/// Gem-compatible generator: complete Chilean RUT.
pub fn full_rut(min: u32, max: u32, fixed: bool, formatted: bool) -> String {
    let config = crate::config::FakerConfig::current();
    let body = if fixed {
        min
    } else {
        config.rand_range(min, max)
    };
    let dv = dv_for(&body.to_string());
    if formatted {
        let digits = format!("{body:08}");
        format!(
            "{}.{}.{}-{}",
            &digits[0..2],
            &digits[2..5],
            &digits[5..8],
            dv
        )
    } else {
        format!("{body}-{dv}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rut() {
        let rut_num = rut();
        assert!(rut_num.contains('-'));
        let parts: Vec<&str> = rut_num.split('-').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_calculate_dv_known_vector() {
        assert_eq!(calculate_dv(12_345_678), '5');
    }
}

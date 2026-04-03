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
}

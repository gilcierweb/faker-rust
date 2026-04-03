//! ID Number generator

use crate::config::FakerConfig;

/// Generate a random Spanish DNI (Documento Nacional de Identidad)
pub fn spanish() -> String {
    let config = FakerConfig::current();
    let number = config.rand_range(10000000, 99999999);
    let letters = ['T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H', 'L', 'C', 'K', 'E'];
    let letter = letters[(number % 23) as usize];
    format!("{}{}", number, letter)
}

/// Generate a random US Social Security Number
pub fn ssn() -> String {
    let config = FakerConfig::current();
    let area = config.rand_range(1, 900);
    let group = config.rand_range(1, 100);
    let serial = config.rand_range(1, 10000);
    format!("{:03}-{:02}-{:04}", area, group, serial)
}

/// Generate a random valid ID number
pub fn valid() -> String {
    spanish()
}

/// Generate a random invalid ID number
pub fn invalid() -> String {
    let config = FakerConfig::current();
    let number = config.rand_range(10000000, 99999999);
    let wrong_letter = config.rand_char(&['I', 'O', 'U', 'Ñ']);
    format!("{}{}", number, wrong_letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanish() {
        let dni = spanish();
        assert_eq!(dni.len(), 9);
    }

    #[test]
    fn test_ssn() {
        let ssn_num = ssn();
        assert_eq!(ssn_num.len(), 11);
        assert!(ssn_num.contains('-'));
    }

    #[test]
    fn test_valid() {
        assert!(!valid().is_empty());
    }

    #[test]
    fn test_invalid() {
        assert!(!invalid().is_empty());
    }
}

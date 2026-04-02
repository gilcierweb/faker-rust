//! South Africa specific data generator

use crate::base::sample;
use crate::config::FakerConfig;

/// Generate a random South African ID number
pub fn id_number() -> String {
    let config = FakerConfig::current();
    // YYMMDD format for date of birth
    let year = config.rand_range(50, 100); // 1950-1999
    let month = config.rand_range(1, 13);
    let day = config.rand_range(1, 29);
    
    // Gender: 0000-4999 for female, 5000-9999 for male
    let gender_seq = config.rand_range(0, 10000);
    
    // Citizenship: 0 for SA citizen, 1 for permanent resident
    let citizenship = config.rand_range(0, 2);
    
    // Check digit (simplified - usually needs Luhn algorithm)
    let check = config.rand_range(0, 10);
    
    format!("{:02}{:02}{:02}{:04}{}{}", year, month, day, gender_seq, citizenship, check)
}

/// Generate a random South African phone number
pub fn phone_number() -> String {
    let config = FakerConfig::current();
    let prefixes = ["060", "061", "062", "063", "064", "065", "066", "067", "068", "069",
                    "071", "072", "073", "074", "076", "078", "079", "081", "082", "083", "084"];
    let prefix = sample(&prefixes);
    let number: String = (0..7)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    format!("{} {}", prefix, number)
}

/// Generate a random South African province
pub fn province() -> String {
    let provinces = [
        "Eastern Cape", "Free State", "Gauteng", "KwaZulu-Natal", "Limpopo",
        "Mpumalanga", "Northern Cape", "North West", "Western Cape",
    ];
    sample(&provinces).to_string()
}

/// Generate a random South African license plate
pub fn license_plate() -> String {
    let config = FakerConfig::current();
    let province_codes = ["EC", "FS", "GP", "KZN", "L", "MP", "NC", "NW", "WP"];
    let code = sample(&province_codes);
    let letters: String = (0..2)
        .map(|_| config.rand_char(&['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']))
        .collect();
    let numbers: String = (0..3)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    format!("{} {}{}-{}", code, letters, config.rand_range(0, 10), numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_number() {
        let id = id_number();
        assert_eq!(id.len(), 12);
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_phone_number() {
        let phone = phone_number();
        assert!(phone.starts_with("0"));
    }

    #[test]
    fn test_province() {
        assert!(!province().is_empty());
    }

    #[test]
    fn test_license_plate() {
        assert!(!license_plate().is_empty());
    }
}

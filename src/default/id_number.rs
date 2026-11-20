//! ID Number generator

use crate::config::FakerConfig;

/// Gem-compatible alias for [`valid_south_african_id_number`].
pub use self::valid_south_african_id_number as south_african_id_number;

/// Letters used by the Spanish DNI check-digit scheme, indexed by number % 23.
const DNI_LETTERS: [char; 23] = [
    'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H',
    'L', 'C', 'K', 'E',
];

/// Compute the DNI check letter for a given number.
fn spanish_letter(number: u32) -> char {
    DNI_LETTERS[(number % 23) as usize]
}

/// Generate a random Spanish DNI (Documento Nacional de Identidad)
pub fn spanish() -> String {
    let config = FakerConfig::current();
    let number = config.rand_range(10000000, 99999999);
    format!("{}{}", number, spanish_letter(number))
}

/// Generate a random US Social Security Number
pub fn ssn() -> String {
    let config = FakerConfig::current();
    let area = config.rand_range(1, 899);
    let group = config.rand_range(1, 99);
    let serial = config.rand_range(1, 9999);
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

// --- gem parity generators (algorithmic identifiers) ---

const DNI_CHECKS: [char; 23] = [
    'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H',
    'L', 'C', 'K', 'E',
];

fn config() -> crate::config::FakerConfig {
    crate::config::FakerConfig::current()
}

fn random_digits(len: usize) -> String {
    (0..len)
        .map(|_| char::from(b'0' + config().rand_u32(10) as u8))
        .collect()
}

fn random_number_in(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    min + config().rand_u32(max - min + 1)
}

fn south_african_checksum(id: &str) -> u32 {
    let characters: Vec<char> = id.chars().collect();
    let odd: u32 = characters[..characters.len() - 1]
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, character)| character.to_digit(10).unwrap_or(0))
        .sum();
    let even: String = characters
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, character)| *character)
        .collect();
    let doubled = even.parse::<u32>().unwrap_or(0) * 2;
    let even_sum: u32 = doubled
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum();
    (10 - (odd + even_sum) % 10) % 10
}

fn brazilian_checksum(digits: &str) -> u32 {
    let len = digits.len() as u32;
    digits
        .chars()
        .enumerate()
        .map(|(index, character)| character.to_digit(10).unwrap_or(0) * (len + 1 - index as u32))
        .sum::<u32>()
        * 10
}

fn citizen_digit(checksum: u32) -> String {
    match checksum % 11 {
        10 => "0".to_string(),
        remainder => remainder.to_string(),
    }
}

fn identity_digit(checksum: u32) -> String {
    match 11 - checksum % 11 {
        10 => "X".to_string(),
        11 => "0".to_string(),
        value => value.to_string(),
    }
}

fn croatian_checksum(digits: &str) -> u32 {
    let mut control = 10u32;
    for character in digits.chars() {
        control = (control + character.to_digit(10).unwrap_or(0)) % 10;
        if control == 0 {
            control = 10;
        }
        control = control * 2 % 11;
    }
    (11 - control) % 10
}

/// Gem-compatible generator: Spanish citizen number (DNI).
pub fn spanish_citizen_number() -> String {
    let number = random_number_in(10_000_000, 99_999_999);
    format!("{number}-{}", DNI_CHECKS[(number % 23) as usize])
}

/// Gem-compatible generator: Spanish foreign citizen number (NIE).
pub fn spanish_foreign_citizen_number() -> String {
    let prefix = crate::base::sample(&['X', 'Y', 'Z']);
    let prefix_value = match prefix {
        'X' => 0,
        'Y' => 1,
        _ => 2,
    };
    let digits = random_number_in(1_000_000, 9_999_999);
    let combined = format!("{prefix_value}{digits:07}");
    let remainder = combined.parse::<u32>().unwrap_or(0) % 23;
    format!("{prefix}-{digits:07}-{}", DNI_CHECKS[remainder as usize])
}

/// Gem-compatible generator: valid South African ID number.
pub fn valid_south_african_id_number() -> String {
    let body = format!(
        "{:02}{:02}{:02}{:04}{}8",
        config().rand_u32(100),
        random_number_in(1, 13),
        random_number_in(1, 29),
        config().rand_u32(10_000),
        config().rand_u32(2)
    );
    format!("{body}{}", south_african_checksum(&body))
}

/// Gem-compatible generator: invalid South African ID number.
pub fn invalid_south_african_id_number() -> String {
    let body = format!(
        "{:02}{:02}{:02}{:04}{}8",
        config().rand_u32(100),
        random_number_in(13, 99),
        random_number_in(32, 99),
        config().rand_u32(10_000),
        config().rand_u32(2)
    );
    format!("{body}{}", south_african_checksum(&body))
}

/// Gem-compatible generator: Brazilian citizen number (CPF).
pub fn brazilian_citizen_number(formatted: bool) -> String {
    let digits = loop {
        let candidate = random_digits(9);
        if candidate
            .chars()
            .any(|c| c != candidate.chars().next().unwrap_or('0'))
        {
            break candidate;
        }
    };
    let first = citizen_digit(brazilian_checksum(&digits));
    let second = citizen_digit(brazilian_checksum(&format!("{digits}{first}")));
    let number = format!("{digits}{first}{second}");
    if formatted {
        format!(
            "{}.{}.{}-{}",
            &number[0..3],
            &number[3..6],
            &number[6..9],
            &number[9..11]
        )
    } else {
        number
    }
}

/// Gem-compatible generator: Brazilian identity number (RG).
pub fn brazilian_id(formatted: bool) -> String {
    let digits = random_number_in(10_000_000, 99_999_999).to_string();
    let number = format!("{digits}{}", identity_digit(brazilian_checksum(&digits)));
    if formatted {
        format!(
            "{}.{}.{}-{}",
            &number[0..2],
            &number[2..5],
            &number[5..8],
            &number[8..]
        )
    } else {
        number
    }
}

/// Gem-compatible generator: Chilean ID (RUT).
pub fn chilean_id() -> String {
    let digits = random_digits(8);
    let weights = [3u32, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits
        .chars()
        .zip(weights)
        .map(|(character, weight)| character.to_digit(10).unwrap_or(0) * weight)
        .sum();
    let difference = 11 - sum % 11;
    let code = match difference {
        0..=9 => difference.to_string(),
        10 => "K".to_string(),
        _ => "0".to_string(),
    };
    format!("{digits}-{code}")
}

/// Gem-compatible generator: Croatian ID number (OIB).
pub fn croatian_id(international: bool) -> String {
    let digits = random_digits(10);
    let prefix = if international { "HR" } else { "" };
    format!("{prefix}{digits}{}", croatian_checksum(&digits))
}

/// Gem-compatible generator: Danish ID number (CPR).
pub fn danish_id_number(formatted: bool) -> String {
    let year = random_number_in(1900, 1999);
    let short_year = year % 100;
    let controls = if short_year <= 36 {
        vec![0u32, 1, 2, 3]
    } else {
        vec![0u32, 1, 2, 3, 4, 9]
    };
    let control = crate::base::sample(&controls);
    let separator = if formatted { "-" } else { "" };
    format!(
        "{:02}{:02}{:02}{}{}{:02}{}",
        random_number_in(1, 29),
        random_number_in(1, 13),
        short_year,
        separator,
        control,
        config().rand_u32(100),
        config().rand_u32(10)
    )
}

/// Gem-compatible generator: French INSEE social security number.
pub fn french_insee_number() -> String {
    let gender = random_number_in(1, 3);
    let year = config().rand_u32(100);
    let month = random_number_in(1, 13);
    let place = config().rand_u32(100_000);
    let order = random_number_in(100, 1000);
    let number = format!("{gender}{year:02}{month:02}{place:05}{order:03}");
    let remainder = number.parse::<u64>().unwrap_or(0) % 97;
    let check = (97 - remainder) % 97;
    format!("{number}{check:02}")
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
        // Repeat to catch boundary values that overflow fixed-width fields.
        for _ in 0..1000 {
            let ssn_num = ssn();
            assert_eq!(ssn_num.len(), 11, "bad ssn: {ssn_num}");
            assert!(ssn_num.contains('-'));
        }
    }

    #[test]
    fn test_valid() {
        assert!(!valid().is_empty());
    }

    #[test]
    fn test_invalid() {
        assert!(!invalid().is_empty());
    }

    #[test]
    fn test_spanish_dni_letter_vector() {
        assert_eq!(spanish_letter(12_345_678), 'Z');
    }
}

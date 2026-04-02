//! Number generator - generates random numbers

use crate::config::FakerConfig;

/// Generate a random number as a string
pub fn number() -> String {
    let config = FakerConfig::current();
    config.rand_range(0, 10_000_000).to_string()
}

/// Generate a random number within a range (inclusive)
pub fn between(min: i64, max: i64) -> i64 {
    let config = FakerConfig::current();
    config.rand_range(min as u32, max as u32) as i64
}

/// Generate a random number within a range (u32)
pub fn between_u32(min: u32, max: u32) -> u32 {
    let config = FakerConfig::current();
    config.rand_range(min, max)
}

/// Generate a random decimal number
pub fn decimal(l_digits: i32, r_digits: i32) -> String {
    let config = FakerConfig::current();
    let left: u32 = config.rand_range(0, 10_u32.pow(l_digits as u32));
    let right: u32 = config.rand_range(0, 10_u32.pow(r_digits as u32));
    format!("{}.{:0>width$}", left, right, width = r_digits as usize)
}

/// Generate a random number within a range and convert to string
pub fn number_range(min: u32, max: u32) -> String {
    between_u32(min, max).to_string()
}

/// Generate a random hexadecimal string
pub fn hexify(upper: bool) -> String {
    let config = FakerConfig::current();
    let chars: &[char] = if upper {
        &crate::base::HEX_UPPER
    } else {
        &crate::base::HEX_LOWER
    };
    (0..10).map(|_| config.rand_char(chars)).collect()
}

/// Generate a random number between 0 and 999
pub fn number_to_ruby() -> i64 {
    between(0, 999)
}

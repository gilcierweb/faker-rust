//! Number generator - generates random numbers

use crate::config::FakerConfig;

/// Generate a random number as a string
pub fn number() -> String {
    let config = FakerConfig::current();
    config.rand_u32(10_000_000).to_string()
}

/// Generate a random number within a range (inclusive)
pub fn between(min: i64, max: i64) -> i64 {
    let config = FakerConfig::current();
    config.rand_range_i64(min, max)
}

/// Generate a random number within a range (u32)
pub fn between_u32(min: u32, max: u32) -> u32 {
    let config = FakerConfig::current();
    config.rand_range(min, max)
}

/// Generate a random decimal number
pub fn decimal(l_digits: i32, r_digits: i32) -> String {
    let config = FakerConfig::current();
    let left: u32 = config.rand_u32(10_u32.pow(l_digits as u32));
    let right: u32 = config.rand_u32(10_u32.pow(r_digits as u32));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FakerConfig;

    #[test]
    fn test_between_negative() {
        FakerConfig::set_seed(7);
        let mut saw_negative = false;
        for _ in 0..1000 {
            if between(-10, 10) < 0 {
                saw_negative = true;
                break;
            }
        }
        assert!(saw_negative, "between must be able to return negative values");
    }

    #[test]
    fn test_between_bounds() {
        FakerConfig::set_seed(1);
        let mut saw_min = false;
        let mut saw_max = false;
        for _ in 0..10000 {
            let v = between(1, 10);
            if v == 1 {
                saw_min = true;
            }
            if v == 10 {
                saw_max = true;
            }
        }
        assert!(saw_min && saw_max, "both bounds must be reachable");
    }
}

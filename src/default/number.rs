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
// --- gem parity generators (algorithmic) ---
fn random_digits(digits: u32, alphabet: &[char]) -> String {
    let config = crate::config::FakerConfig::current();
    (0..digits).map(|_| config.rand_char(alphabet)).collect()
}

/// Gem-compatible generator: random binary string of `digits` characters.
pub fn binary(digits: u32) -> String {
    random_digits(digits, &['0', '1'])
}
/// Gem-compatible generator: random decimal fraction of `digits` characters.
pub fn decimal_part(digits: u32) -> String {
    let config = crate::config::FakerConfig::current();
    (0..digits)
        .map(|_| char::from(b'0' + config.rand_u32(10) as u8))
        .collect()
}
/// Gem-compatible generator: one digit (0-9).
pub fn digit() -> u32 {
    crate::config::FakerConfig::current().rand_u32(10)
}
/// Gem-compatible generator: random hexadecimal string of `digits` characters.
pub fn hexadecimal(digits: u32) -> String {
    random_digits(digits, &crate::base::HEX_LOWER)
}
/// Gem-compatible generator: `digits`-character number, zero padding allowed.
pub fn leading_zero_number(digits: u32) -> String {
    decimal_part(digits)
}
/// Gem-compatible generator: negative floating-point value in `from..=to`.
pub fn negative(from: f64, to: f64) -> f64 {
    let config = crate::config::FakerConfig::current();
    from + config.rand_f64() * (to - from)
}
/// Gem-compatible generator: one non-zero digit (1-9).
pub fn non_zero_digit() -> u32 {
    1 + crate::config::FakerConfig::current().rand_u32(9)
}
/// Gem-compatible generator: normally distributed value (Box-Muller).
pub fn normal(mean: f64, standard_deviation: f64) -> f64 {
    let config = crate::config::FakerConfig::current();
    let u1 = config.rand_f64().max(f64::MIN_POSITIVE);
    let u2 = config.rand_f64();
    let z = (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos();
    mean + z * standard_deviation
}
/// Gem-compatible generator: positive floating-point value in `from..=to`.
pub fn positive(from: f64, to: f64) -> f64 {
    let config = crate::config::FakerConfig::current();
    from + config.rand_f64() * (to - from)
}
/// Gem-compatible generator: floating-point value inside an inclusive range.
pub fn within(range: std::ops::RangeInclusive<f64>) -> f64 {
    let (start, end) = range.into_inner();
    positive(start, end)
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
        assert!(
            saw_negative,
            "between must be able to return negative values"
        );
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

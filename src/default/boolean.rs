//! Boolean generator - generates random boolean values

use crate::config::FakerConfig;

/// Generate a random boolean value
pub fn boolean() -> bool {
    FakerConfig::current().rand_u32(2) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_produces_both_values() {
        let mut saw_true = false;
        let mut saw_false = false;
        for _ in 0..100 {
            if boolean() {
                saw_true = true;
            } else {
                saw_false = true;
            }
        }
        assert!(saw_true && saw_false);
    }
}

//! Boolean generator - generates random boolean values

use crate::config::FakerConfig;

/// Generate a random boolean value
pub fn boolean() -> bool {
    FakerConfig::current().rand_u32(2) == 0
}

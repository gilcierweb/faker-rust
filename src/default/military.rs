//! Military generator - generates military ranks

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Army rank
pub fn army_rank() -> String {
    fetch_locale("military.army_rank", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random Marines rank
pub fn marines_rank() -> String {
    fetch_locale(
        "military.marines_rank",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random Navy rank
pub fn navy_rank() -> String {
    fetch_locale("military.navy_rank", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Petty Officer".to_string())
}

/// Generate a random Coast Guard rank
pub fn coast_guard_rank() -> String {
    fetch_locale(
        "military.coast_guard_rank",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Petty Officer".to_string())
}

/// Generate a random Air Force rank
pub fn air_force_rank() -> String {
    fetch_locale(
        "military.air_force_rank",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random Space Force rank
pub fn space_force_rank() -> String {
    fetch_locale(
        "military.space_force_rank",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random DoD pay grade
pub fn dod_paygrade() -> String {
    fetch_locale(
        "military.dod_paygrade",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| "E-5".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_are_nonempty() {
        for rank in [
            army_rank(),
            marines_rank(),
            navy_rank(),
            coast_guard_rank(),
            air_force_rank(),
            space_force_rank(),
            dod_paygrade(),
        ] {
            assert!(!rank.is_empty());
            assert!(!rank.contains("#{"), "unresolved: {rank}");
        }
    }
}

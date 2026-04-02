//! Military generator - generates military ranks

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Army rank
pub fn army_rank() -> String {
    fetch_locale("military.army_rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random Marines rank
pub fn marines_rank() -> String {
    fetch_locale("military.marines_rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random Navy rank
pub fn navy_rank() -> String {
    fetch_locale("military.navy_rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Petty Officer".to_string())
}

/// Generate a random Coast Guard rank
pub fn coast_guard_rank() -> String {
    fetch_locale("military.coast_guard_rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Petty Officer".to_string())
}

/// Generate a random Air Force rank
pub fn air_force_rank() -> String {
    fetch_locale("military.air_force_rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random Space Force rank
pub fn space_force_rank() -> String {
    fetch_locale("military.space_force_rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Sergeant".to_string())
}

/// Generate a random DoD pay grade
pub fn dod_paygrade() -> String {
    fetch_locale("military.dod_paygrade", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "E-5".to_string())
}

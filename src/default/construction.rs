//! Construction generator - generates random construction-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random construction material
pub fn material() -> String {
    fetch_locale("construction.materials", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Steel".to_string())
}

/// Generate a random subcontract category
pub fn subcontract_category() -> String {
    fetch_locale("construction.subcontract_categories", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Plumbing".to_string())
}

/// Generate a random heavy equipment name
pub fn heavy_equipment() -> String {
    fetch_locale("construction.heavy_equipment", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Excavator".to_string())
}

/// Generate a random trade
pub fn trade() -> String {
    fetch_locale("construction.trades", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Carpenter".to_string())
}

/// Generate a random standard cost code
pub fn standard_cost_code() -> String {
    fetch_locale("construction.standard_cost_codes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "1000".to_string())
}

/// Generate a random construction role
pub fn role() -> String {
    fetch_locale("construction.roles", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Project Manager".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material() {
        let m = material();
        assert!(!m.is_empty());
    }
}

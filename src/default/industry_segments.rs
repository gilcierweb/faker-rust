//! Industry segments generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random industry
pub fn industry() -> String {
    fetch_locale("industry_segments.industries", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_INDUSTRIES).to_string())
}

/// Generate a random sector
pub fn sector() -> String {
    fetch_locale("industry_segments.sectors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SECTORS).to_string())
}

/// Generate a random subsector
pub fn subsector() -> String {
    fetch_locale("industry_segments.subsectors", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SUBSECTORS).to_string())
}

// Fallback data
const FALLBACK_INDUSTRIES: &[&str] = &[
    "Technology", "Healthcare", "Finance", "Manufacturing", "Retail",
    "Energy", "Transportation", "Agriculture", "Construction", "Education",
    "Entertainment", "Hospitality", "Telecommunications", "Automotive",
    "Aerospace", "Pharmaceuticals", "Biotechnology", "Real Estate",
];

const FALLBACK_SECTORS: &[&str] = &[
    "Primary", "Secondary", "Tertiary", "Quaternary", "Public Sector",
    "Private Sector", "Non-profit", "Government", "Defense", "Space",
];

const FALLBACK_SUBSECTORS: &[&str] = &[
    "Software", "Hardware", "Services", "Consulting", "Research",
    "Development", "Sales", "Marketing", "Operations", "Logistics",
    "Support", "Maintenance", "Quality Assurance", "Compliance",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_industry() {
        assert!(!industry().is_empty());
    }

    #[test]
    fn test_sector() {
        assert!(!sector().is_empty());
    }

    #[test]
    fn test_subsector() {
        assert!(!subsector().is_empty());
    }
}

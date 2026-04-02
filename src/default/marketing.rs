//! Marketing generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random marketing buzzword
pub fn buzzword() -> String {
    fetch_locale("marketing.buzzwords", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BUZZWORDS).to_string())
}

/// Generate a random marketing term
pub fn term() -> String {
    fetch_locale("marketing.terms", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TERMS).to_string())
}

/// Generate a random marketing strategy
pub fn strategy() -> String {
    fetch_locale("marketing.strategies", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STRATEGIES).to_string())
}

// Fallback data
const FALLBACK_BUZZWORDS: &[&str] = &[
    "Synergy", "Paradigm Shift", "Disruptive", "Scalable", "Holistic",
    "Granular", "Actionable", "Best-in-Class", "Value-Add", "Game-Changer",
    "Growth Hacking", "Deep Dive", "Thought Leader", "ROI", "Low-Hanging Fruit",
    "Move the Needle", "Circle Back", "Bandwidth", "Leverage", "Pivot",
];

const FALLBACK_TERMS: &[&str] = &[
    "Brand Awareness", "Lead Generation", "Conversion Rate", "Customer Acquisition",
    "Retention", "Engagement", "Analytics", "SEO", "Content Marketing",
    "Social Media", "Influencer", "Email Campaign", "Funnel", "Touchpoint",
];

const FALLBACK_STRATEGIES: &[&str] = &[
    "Inbound Marketing", "Outbound Marketing", "Account-Based Marketing",
    "Growth Marketing", "Performance Marketing", "Viral Marketing",
    "Referral Marketing", "Affiliate Marketing", "Content Strategy",
    "Social Media Strategy", "Email Marketing Strategy", "Brand Strategy",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buzzword() {
        assert!(!buzzword().is_empty());
    }

    #[test]
    fn test_term() {
        assert!(!term().is_empty());
    }

    #[test]
    fn test_strategy() {
        assert!(!strategy().is_empty());
    }
}

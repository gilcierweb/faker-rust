//! Stripe generator - fake data for Stripe-like financial services

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Stripe plan
pub fn plan() -> String {
    fetch_locale("stripe.plans", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLANS).to_string())
}

/// Generate a random Stripe product
pub fn product() -> String {
    fetch_locale("stripe.products", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PRODUCTS).to_string())
}

/// Generate a random Stripe subscription status
pub fn subscription_status() -> String {
    fetch_locale("stripe.subscription_statuses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STATUSES).to_string())
}

// Fallback data
const FALLBACK_PLANS: &[&str] = &[
    "Basic", "Standard", "Premium", "Enterprise", "Starter", "Pro", "Ultimate",
    "Free", "Gold", "Platinum", "Silver", "Bronze",
];

const FALLBACK_PRODUCTS: &[&str] = &[
    "SaaS Subscription", "API Access", "Premium Support", "Storage Plan",
    "Analytics Dashboard", "Custom Integration", "White Label Solution",
    "Data Export", "Advanced Reporting", "Team Collaboration",
];

const FALLBACK_STATUSES: &[&str] = &[
    "active", "canceled", "past_due", "unpaid", "trialing", "paused",
    "incomplete", "incomplete_expired",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        assert!(!plan().is_empty());
    }

    #[test]
    fn test_product() {
        assert!(!product().is_empty());
    }

    #[test]
    fn test_subscription_status() {
        assert!(!subscription_status().is_empty());
    }
}

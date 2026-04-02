//! Subscription generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random subscription plan
pub fn plan() -> String {
    fetch_locale("subscription.plans", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLANS).to_string())
}

/// Generate a random subscription status
pub fn status() -> String {
    fetch_locale("subscription.statuses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STATUSES).to_string())
}

/// Generate a random payment method
pub fn payment_method() -> String {
    fetch_locale("subscription.payment_methods", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PAYMENT_METHODS).to_string())
}

// Fallback data
const FALLBACK_PLANS: &[&str] = &[
    "Free", "Basic", "Standard", "Premium", "Pro", "Enterprise", "Starter",
    "Gold", "Silver", "Bronze", "Platinum", "Ultimate",
];

const FALLBACK_STATUSES: &[&str] = &[
    "active", "inactive", "pending", "canceled", "expired", "suspended",
    "trial", "past_due",
];

const FALLBACK_PAYMENT_METHODS: &[&str] = &[
    "Credit Card", "Debit Card", "PayPal", "Bank Transfer", "Apple Pay",
    "Google Pay", "Cryptocurrency", "Direct Debit", "Check",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        assert!(!plan().is_empty());
    }

    #[test]
    fn test_status() {
        assert!(!status().is_empty());
    }

    #[test]
    fn test_payment_method() {
        assert!(!payment_method().is_empty());
    }
}

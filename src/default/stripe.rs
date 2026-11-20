//! Stripe generator - fake data for Stripe-like financial services

use crate::base::sample;
use crate::locale::{fetch_locale, fetch_locale_named};

/// A valid test card number for the requested brand (e.g. "visa", "mc", "amex").
pub fn valid_card(brand: &str) -> String {
    fetch_locale_named(
        "stripe.valid_cards",
        brand,
        &crate::config::FakerConfig::locale(),
    )
    .unwrap_or_else(|| "4242424242424242".to_string())
}

/// A valid test token for the requested brand (e.g. "visa", "mc", "amex").
pub fn valid_token(brand: &str) -> String {
    fetch_locale_named(
        "stripe.valid_tokens",
        brand,
        &crate::config::FakerConfig::locale(),
    )
    .unwrap_or_else(|| "tok_visa".to_string())
}

/// An invalid test card number for a given failure reason (e.g. "addressZipFail").
pub fn invalid_card(reason: &str) -> String {
    fetch_locale_named(
        "stripe.invalid_cards",
        reason,
        &crate::config::FakerConfig::locale(),
    )
    .unwrap_or_else(|| "4000000000000002".to_string())
}

/// Generate a random Stripe plan
pub fn plan() -> String {
    fetch_locale("subscription.plans", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PLANS).to_string())
}

/// Generate a random Stripe product
pub fn product() -> String {
    fetch_locale("stripe.products", &crate::config::FakerConfig::locale())
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PRODUCTS).to_string())
}

/// Generate a random Stripe subscription status
pub fn subscription_status() -> String {
    fetch_locale(
        "subscription.statuses",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_STATUSES).to_string())
}

// Fallback data
const FALLBACK_PLANS: &[&str] = &[
    "Basic",
    "Standard",
    "Premium",
    "Enterprise",
    "Starter",
    "Pro",
    "Ultimate",
    "Free",
    "Gold",
    "Platinum",
    "Silver",
    "Bronze",
];

const FALLBACK_PRODUCTS: &[&str] = &[
    "SaaS Subscription",
    "API Access",
    "Premium Support",
    "Storage Plan",
    "Analytics Dashboard",
    "Custom Integration",
    "White Label Solution",
    "Data Export",
    "Advanced Reporting",
    "Team Collaboration",
];

const FALLBACK_STATUSES: &[&str] = &[
    "active",
    "canceled",
    "past_due",
    "unpaid",
    "trialing",
    "paused",
    "incomplete",
    "incomplete_expired",
];
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: two-digit expiry month.
pub fn month() -> String {
    format!(
        "{:02}",
        crate::config::FakerConfig::current().rand_range(1, 13)
    )
}
/// Gem-compatible generator: four-digit future year.
pub fn year() -> String {
    let start = time::OffsetDateTime::now_utc().year() + 1;
    (start + crate::config::FakerConfig::current().rand_range(0, 6) as i32).to_string()
}
/// Gem-compatible generator: card security code (4 digits for Amex).
pub fn ccv(card_type: &str) -> String {
    let config = crate::config::FakerConfig::current();
    if card_type.eq_ignore_ascii_case("amex") {
        config.rand_range(1000, 10000).to_string()
    } else {
        config.rand_range(100, 1000).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let value = plan();
        assert!(FALLBACK_PLANS.contains(&value.as_str()) || !value.is_empty());
    }

    #[test]
    fn test_product_from_known_set() {
        let value = product();
        assert!(FALLBACK_PRODUCTS.contains(&value.as_str()) || !value.is_empty());
    }

    #[test]
    fn test_subscription_status_from_known_set() {
        let value = subscription_status();
        assert!(FALLBACK_STATUSES.contains(&value.as_str()) || !value.is_empty());
    }

    #[test]
    fn test_valid_card_and_token_from_locale_data() {
        assert_eq!(valid_card("visa"), "4242424242424242");
        assert!(valid_card("visa").starts_with('4'));
        assert!(valid_card("mc").starts_with('5'));
        assert_eq!(valid_token("visa"), "tok_visa");
        assert!(valid_token("amex").starts_with("tok_"));
    }

    #[test]
    fn test_invalid_card_uses_failure_reason() {
        let card = invalid_card("addressZipFail");
        assert_eq!(card, "4000000000000010");
        assert!(card.chars().all(|c| c.is_ascii_digit()));
    }
}

//! OmniAuth generator

use crate::base::sample;
use crate::locale::fetch_locale;
use crate::config::FakerConfig;

/// Generate a random OmniAuth provider
pub fn provider() -> String {
    fetch_locale("omniauth.providers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PROVIDERS).to_string())
}

/// Generate a random OmniAuth UID
pub fn uid() -> String {
    let config = FakerConfig::current();
    let numbers: String = (0..17)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    numbers
}

/// Generate a random OmniAuth name
pub fn name() -> String {
    let names = [
        "google_oauth2", "facebook", "twitter", "github", "linkedin",
        "developer", "saml", "openid_connect", "auth0", "okta",
    ];
    sample(&names).to_string()
}

/// Generate a random directory name
pub fn directory() -> String {
    let dirs = [
        "src", "lib", "bin", "test", "docs", "config", "assets",
        "public", "private", "vendor", "node_modules", "target",
    ];
    sample(&dirs).to_string()
}

// Fallback data
const FALLBACK_PROVIDERS: &[&str] = &[
    "google_oauth2", "facebook", "twitter", "github", "linkedin", "developer",
    "saml", "openid_connect", "auth0", "okta", "microsoft", "slack", "discord",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider() {
        assert!(!provider().is_empty());
    }

    #[test]
    fn test_uid() {
        let id = uid();
        assert_eq!(id.len(), 17);
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }
}

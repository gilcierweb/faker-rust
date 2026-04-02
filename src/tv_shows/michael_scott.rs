//! Michael Scott (The Office) character generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Michael Scott quote
pub fn quote() -> String {
    fetch_locale("michael_scott.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random Michael Scott business idea
pub fn business_idea() -> String {
    fetch_locale("michael_scott.business_ideas", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_BUSINESS_IDEAS).to_string())
}

// Fallback data
const FALLBACK_QUOTES: &[&str] = &[
    "That's what she said!",
    "I'm not superstitious, but I am a little stitious.",
    "Would I rather be feared or loved? Easy. Both.",
    "I have flaws. What are they? I sing in the shower.",
    "I am Beyoncé, always.",
    "You miss 100% of the shots you don't take. - Wayne Gretzky - Michael Scott",
    "The worst thing about prison was the dementors.",
    "I am running away from my responsibilities. And it feels good.",
    "I love inside jokes. I'd love to be a part of one someday.",
];

const FALLBACK_BUSINESS_IDEAS: &[&str] = &[
    "The Michael Scott Paper Company",
    "Threat Level Midnight",
    "Scott's Tots",
    "Dunder Mifflin Infinity",
    "Golden Ticket",
    "Crime Aid",
    "Michael Scott's Dunder Mifflin Scranton Meredith Palmer Memorial Celebrity Rabies Awareness Pro-Am Fun Run Race for the Cure",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_business_idea() {
        assert!(!business_idea().is_empty());
    }
}

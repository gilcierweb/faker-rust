//! Hacker/coder generator - generates hacker-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random hacker abbreviation
pub fn abbreviation() -> String {
    fetch_locale("hacker.abbreviations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ABBREVIATIONS).to_string())
}

/// Generate a random hacker adjective
pub fn adjective() -> String {
    fetch_locale("hacker.adjectives", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ADJECTIVES).to_string())
}

/// Generate a random hacker noun
pub fn noun() -> String {
    fetch_locale("hacker.nouns", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NOUNS).to_string())
}

/// Generate a random hacker verb
pub fn verb() -> String {
    fetch_locale("hacker.verbs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_VERBS).to_string())
}

/// Generate a random hacker phrase
pub fn ingverb() -> String {
    fetch_locale("hacker.ingverbs", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_INGVERBS).to_string())
}

/// Generate a random hacker phrase
pub fn phrase() -> String {
    let parts = [adjective(),
        abbreviation(),
        noun(),
        verb(),
        ingverb()];
    parts.join(" ")
}

// Fallback data
const FALLBACK_ABBREVIATIONS: &[&str] = &[
    "TCP", "HTTP", "SSD", "RAM", "GB", "CSS", "SSL", "AGP", "SQL", "XML",
    "EXE", "COM", "HDD", "THX", "SMTP", "SMS", "USB", "PNG", "SAS", "IB",
    "PCI", "DNS", "FTP", "GSM", "CSS", "HDMI", "HTTP", "TLS", "CSS", "JSON",
];

const FALLBACK_ADJECTIVES: &[&str] = &[
    "auxiliary", "primary", "back-end", "digital", "open-source", "virtual",
    "cross-platform", "redundant", "online", "haptic", "multi-byte", "bluetooth",
    "wireless", "1080p", "neural", "optical", "solid state", "mobile",
];

const FALLBACK_NOUNS: &[&str] = &[
    "driver", "protocol", "bandwidth", "panel", "microchip", "program",
    "port", "card", "array", "interface", "system", "sensor", "firewall",
    "hard drive", "pixel", "alarm", "feed", "monitor", "application",
    "transmitter", "bus", "circuit", "capacitor", "matrix",
];

const FALLBACK_VERBS: &[&str] = &[
    "back up", "bypass", "hack", "override", "compress", "copy", "navigate",
    "index", "connect", "generate", "quantify", "calculate", "synthesize",
    "input", "transmit", "program", "reboot", "parse",
];

const FALLBACK_INGVERBS: &[&str] = &[
    "backing up", "bypassing", "hacking", "overriding", "compressing",
    "copying", "navigating", "indexing", "connecting", "generating",
    "quantifying", "calculating", "synthesizing", "inputting", "transmitting",
    "programming", "rebooting", "parsing",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviation() {
        assert!(!abbreviation().is_empty());
    }

    #[test]
    fn test_adjective() {
        assert!(!adjective().is_empty());
    }

    #[test]
    fn test_noun() {
        assert!(!noun().is_empty());
    }

    #[test]
    fn test_verb() {
        assert!(!verb().is_empty());
    }

    #[test]
    fn test_phrase() {
        assert!(!phrase().is_empty());
    }
}

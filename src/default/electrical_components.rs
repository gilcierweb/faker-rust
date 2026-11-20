//! Electrical components generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random electrical component
pub fn component() -> String {
    fetch_locale(
        "electrical_components.components",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_COMPONENTS).to_string())
}

/// Generate a random active component
pub fn active() -> String {
    fetch_locale(
        "electrical_components.actives",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_ACTIVE).to_string())
}

/// Generate a random passive component
pub fn passive() -> String {
    fetch_locale(
        "electrical_components.passives",
        &crate::config::FakerConfig::locale(),
    )
    .map(|v| sample(&v))
    .unwrap_or_else(|| sample(FALLBACK_PASSIVE).to_string())
}

// Fallback data
const FALLBACK_COMPONENTS: &[&str] = &[
    "Resistor",
    "Capacitor",
    "Inductor",
    "Transistor",
    "Diode",
    "LED",
    "Microcontroller",
    "Integrated Circuit",
    "Transformer",
    "Relay",
    "Switch",
    "Fuse",
    "Circuit Breaker",
    "Potentiometer",
    "Oscillator",
    "Sensor",
];

const FALLBACK_ACTIVE: &[&str] = &[
    "Transistor",
    "Diode",
    "LED",
    "Microcontroller",
    "Op-Amp",
    "Voltage Regulator",
    "Logic Gate",
    "Amplifier",
    "Rectifier",
    "Thyristor",
    "MOSFET",
    "BJT",
];

const FALLBACK_PASSIVE: &[&str] = &[
    "Resistor",
    "Capacitor",
    "Inductor",
    "Transformer",
    "Potentiometer",
    "Varistor",
    "Thermistor",
    "Photoresistor",
    "Crystal",
    "Fuse",
    "Switch",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, &crate::config::FakerConfig::locale())
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `electrical_components.electromechanical`.
pub fn electromechanical() -> String {
    parity_pick("electrical_components.electromechanical")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component() {
        assert!(!component().is_empty());
    }

    #[test]
    fn test_active() {
        assert!(!active().is_empty());
    }

    #[test]
    fn test_passive() {
        assert!(!passive().is_empty());
    }
}

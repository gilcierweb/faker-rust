//! Escape from Tarkov game generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Tarkov location/raid
pub fn location() -> String {
    fetch_locale("tarkov.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Tarkov trader
pub fn trader() -> String {
    fetch_locale("tarkov.traders", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TRADERS).to_string())
}

/// Generate a random Tarkov item/weapon
pub fn item() -> String {
    fetch_locale("tarkov.items", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ITEMS).to_string())
}

/// Generate a random Tarkov ammunition type
pub fn ammo() -> String {
    fetch_locale("tarkov.ammo", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_AMMO).to_string())
}

/// Generate a random Tarkov faction
pub fn faction() -> String {
    fetch_locale("tarkov.factions", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FACTIONS).to_string())
}

// Fallback data
const FALLBACK_LOCATIONS: &[&str] = &[
    "Customs",
    "Factory",
    "Interchange",
    "Reserve",
    "Woods",
    "Shoreline",
    "The Lab",
    "Lighthouse",
    "Streets of Tarkov",
    "Ground Zero",
    "Hideout",
];

const FALLBACK_TRADERS: &[&str] = &[
    "Prapor",
    "Therapist",
    "Fence",
    "Skier",
    "Peacekeeper",
    "Mechanic",
    "Ragman",
    "Jaeger",
    "Ref",
    "Lightkeeper",
];

const FALLBACK_ITEMS: &[&str] = &[
    "AK-74N",
    "AKM",
    "M4A1",
    "MP5",
    "Mosin Nagant",
    "SVDS",
    "RSASS",
    "DVL-10",
    "M870",
    "MP-133",
    "Saiga-12",
    "AS VAL",
    "VSS Vintorez",
    "RPK-16",
    "HK 416A5",
    "DT MDR",
    "SR-25",
    "M700",
    "T-7 Thermal Goggles",
    "SICC Case",
    "Docs Case",
    "Money Case",
    "Item Case",
    "Weapon Case",
    "Ammo Case",
    "Magazine Case",
    "Grenade Case",
    "Keytool",
    "Injector Case",
    "Secure Container Alpha",
    "Secure Container Beta",
    "Secure Container Gamma",
    "Secure Container Kappa",
];

const FALLBACK_AMMO: &[&str] = &[
    "5.45x39mm PS",
    "5.45x39mm BP",
    "5.45x39mm BS",
    "5.45x39mm 7N39",
    "5.56x45mm M855",
    "5.56x45mm M855A1",
    "5.56x45mm M995",
    "7.62x39mm PS",
    "7.62x39mm BP",
    "7.62x51mm M80",
    "7.62x51mm M61",
    "7.62x54mmR LPS",
    "7.62x54mmR 7N1",
    "9x19mm PST",
    "9x19mm AP 6.3",
    "9x19mm 7N31",
    "12/70 Buckshot",
    "12/70 Slug",
    "12/70 AP-20",
    ".300 Blackout",
    ".338 Lapua",
];

const FALLBACK_FACTIONS: &[&str] = &[
    "BEAR",
    "USEC",
    "Scavs",
    "Raider",
    "Boss",
    "Rogue",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_trader() {
        assert!(!trader().is_empty());
    }

    #[test]
    fn test_item() {
        assert!(!item().is_empty());
    }

    #[test]
    fn test_ammo() {
        assert!(!ammo().is_empty());
    }

    #[test]
    fn test_faction() {
        assert!(!faction().is_empty());
    }
}

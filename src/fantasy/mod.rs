//! Fantasy generator - generates fantasy-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Tolkien race
pub fn tolkien_race() -> String {
    fetch_locale("fantasy.tolkien_races", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TOLKIEN_RACES).to_string())
}

/// Generate a random fantasy creature
pub fn creature() -> String {
    fetch_locale("fantasy.creatures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATURES).to_string())
}

/// Generate a random fantasy location
pub fn location() -> String {
    fetch_locale("fantasy.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random fantasy weapon
pub fn weapon() -> String {
    fetch_locale("fantasy.weapons", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_WEAPONS).to_string())
}

/// Generate a random fantasy spell
pub fn spell() -> String {
    fetch_locale("fantasy.spells", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SPELLS).to_string())
}

// Fallback data
const FALLBACK_TOLKIEN_RACES: &[&str] = &[
    "Elf", "Dwarf", "Human", "Hobbit", "Orc", "Ent", "Wizard", "Dragon",
    "Troll", "Goblin", "Uruk-hai", "Balrog", "Nazgûl",
];

const FALLBACK_CREATURES: &[&str] = &[
    "Dragon", "Phoenix", "Unicorn", "Griffin", "Kraken", "Basilisk",
    "Chimera", "Hydra", "Minotaur", "Pegasus", "Centaur", "Werewolf",
    "Vampire", "Fairy", "Giant", "Troll", "Ogre", "Goblin",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Mordor", "Rivendell", "The Shire", "Gondor", "Rohan", "Isengard",
    "Helm's Deep", "Minas Tirith", "Lothlórien", "Mirkwood", "The Misty Mountains",
    "Castle Dracula", "The Forbidden Forest", "Hogwarts", "Narnia",
];

const FALLBACK_WEAPONS: &[&str] = &[
    "Excalibur", "Glamdring", "Orcrist", "Sting", "Narsil", "Andúril",
    "The Master Sword", "The Elder Wand", "Stormbringer", "Mjolnir",
    "Longbow", "Battle Axe", "War Hammer", "Dagger", "Spear", "Staff",
];

const FALLBACK_SPELLS: &[&str] = &[
    "Fireball", "Lightning Bolt", "Healing Touch", "Invisibility", "Teleport",
    "Frost Nova", "Chain Lightning", "Arcane Blast", "Shadow Bolt", "Holy Light",
    "Polymorph", "Time Stop", "Resurrection", "Summon Demon", "Shield of Light",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tolkien_race() {
        assert!(!tolkien_race().is_empty());
    }

    #[test]
    fn test_creature() {
        assert!(!creature().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_weapon() {
        assert!(!weapon().is_empty());
    }

    #[test]
    fn test_spell() {
        assert!(!spell().is_empty());
    }
}

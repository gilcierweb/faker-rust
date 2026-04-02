//! Super Smash Bros game generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Super Smash Bros fighter
pub fn fighter() -> String {
    fetch_locale("super_smash_bros.fighters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FIGHTERS).to_string())
}

/// Generate a random Super Smash Bros stage
pub fn stage() -> String {
    fetch_locale("super_smash_bros.stages", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STAGES).to_string())
}

/// Generate a random Super Smash Bros item
pub fn item() -> String {
    fetch_locale("super_smash_bros.items", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_ITEMS).to_string())
}

// Fallback data
const FALLBACK_FIGHTERS: &[&str] = &[
    "Mario",
    "Luigi",
    "Peach",
    "Bowser",
    "Yoshi",
    "Donkey Kong",
    "Link",
    "Zelda",
    "Samus",
    "Kirby",
    "Fox",
    "Pikachu",
    "Captain Falcon",
    "Ness",
    "Jigglypuff",
    "Marth",
    "Roy",
    "Mr. Game & Watch",
    "Ice Climbers",
    "Sheik",
    "Dr. Mario",
    "Pichu",
    "Falco",
    "Marth",
    "Young Link",
    "Ganondorf",
    "Mewtwo",
    "Meta Knight",
    "Pit",
    "Wario",
    "Snake",
    "Ike",
    "Pokemon Trainer",
    "Diddy Kong",
    "Lucas",
    "Sonic",
    "King Dedede",
    "Olimar",
    "Lucario",
    "R.O.B.",
    "Toon Link",
    "Wolf",
    "Villager",
    "Mega Man",
    "Wii Fit Trainer",
    "Rosalina & Luma",
    "Little Mac",
    "Greninja",
    "Mii Fighter",
    "Palutena",
    "Pac-Man",
    "Robin",
    "Shulk",
    "Bowser Jr.",
    "Duck Hunt",
    "Ryu",
    "Cloud",
    "Corrin",
    "Bayonetta",
    "Inkling",
    "Ridley",
    "Simon",
    "Richter",
    "King K. Rool",
    "Isabelle",
    "Incineroar",
    "Piranha Plant",
    "Joker",
    "Hero",
    "Banjo & Kazooie",
    "Terry",
    "Byleth",
    "Min Min",
    "Steve",
    "Sephiroth",
    "Pyra/Mythra",
    "Kazuya",
    "Sora",
];

const FALLBACK_STAGES: &[&str] = &[
    "Battlefield",
    "Final Destination",
    "Peach's Castle",
    "Mushroom Kingdom",
    "Hyrule Castle",
    "Planet Zebes",
    "Dream Land",
    "Sector Z",
    "Saffron City",
    "Super Happy Tree",
    "Mute City",
    "Pokemon Stadium",
    "Onett",
    "Corneria",
    "Big Blue",
    "Brinstar",
    "Rainbow Cruise",
    "Temple",
    "Yoshi's Island",
    "Jungle Japes",
    "Green Greens",
    "Fountain of Dreams",
    "Venom",
    "Bridge of Eldin",
    "Smashville",
    "Lylat Cruise",
    "Castle Siege",
    "Delfino Plaza",
    "Halberd",
    "New Pork City",
    "Summit",
    "Skyworld",
    "PictoChat",
    "Shadow Moses Island",
    "Luigi's Mansion",
    "Pirate Ship",
    "Spear Pillar",
    "Port Town Aero Dive",
    "Norfair",
    "Frigate Orpheon",
    "Yoshi's Island (Brawl)",
    "Hanenbow",
    "Green Hill Zone",
    "Mario Circuit",
    "WarioWare Inc.",
    "Distant Planet",
    "75m",
    "Mario Bros.",
    "Rumble Falls",
    "Flat Zone 2",
    "Skyloft",
    "Kalos Pokemon League",
    "Pac-Land",
    "Suzaku Castle",
    "Midgar",
    "Umbra Clock Tower",
    "Moray Towers",
    "Dracula's Castle",
    "Mementos",
    "Yggdrasil's Altar",
    "Spiral Mountain",
    "King of Fighters Stadium",
    "Garreg Mach Monastery",
    "Spring Stadium",
    "Minecraft World",
    "Northern Cave",
    "Cloud Sea of Alrest",
    "Mishima Dojo",
    "Hollow Bastion",
];

const FALLBACK_ITEMS: &[&str] = &[
    "Super Mushroom",
    "Poison Mushroom",
    "Fire Flower",
    "Super Star",
    "Beam Sword",
    "Home-Run Bat",
    "Fan",
    "Star Rod",
    "Ray Gun",
    "Hammer",
    "Motion-Sensor Bomb",
    "Bob-omb",
    "Poke Ball",
    "Bumper",
    "Assist Trophy",
    "Smash Ball",
    "Master Ball",
    "Food",
    "Maxim Tomato",
    "Heart Container",
    "Dragoon",
    "Daybreak",
    "Pitfall",
    "Beehive",
    "Killer Eye",
    "Spiny Shell",
    "Bombchu",
    "Beastball",
    "Black Hole",
    "Healing Field",
    "Fake Smash Ball",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fighter() {
        assert!(!fighter().is_empty());
    }

    #[test]
    fn test_stage() {
        assert!(!stage().is_empty());
    }

    #[test]
    fn test_item() {
        assert!(!item().is_empty());
    }
}

//! The Witcher game generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Witcher character
pub fn character() -> String {
    fetch_locale("witcher.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Witcher location
pub fn location() -> String {
    fetch_locale("witcher.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random Witcher monster
pub fn monster() -> String {
    fetch_locale("witcher.monsters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MONSTERS).to_string())
}

/// Generate a random Witcher school
pub fn school() -> String {
    fetch_locale("witcher.schools", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SCHOOLS).to_string())
}

/// Generate a random Witcher quote
pub fn quote() -> String {
    fetch_locale("witcher.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Geralt of Rivia",
    "Yennefer of Vengerberg",
    "Ciri",
    "Triss Merigold",
    "Dandelion",
    "Vesemir",
    "Zoltan Chivay",
    "Regis",
    "Milva",
    "Cahir",
    "Angouleme",
    "Emhyr var Emreis",
    "Foltest",
    "Radovid V",
    "Vernon Roche",
    "Iorveth",
    "Philippa Eilhart",
    "Francesca Findabair",
    "Eredin Breacc Glas",
    "Avallac'h",
    "Keira Metz",
    "Lambert",
    "Eskel",
    "Letho",
    "Gaunter O'Dimm",
    "Olgierd von Everec",
    "Dettlaff van der Eretein",
    "Sylvia Anna",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "Kaer Morhen",
    "Novigrad",
    "Oxenfurt",
    "Vizima",
    "Velen",
    "Skellige",
    "Toussaint",
    "Beauclair",
    "White Orchard",
    "Flotsam",
    "Vergen",
    "Loc Muinne",
    "Nilfgaard",
    "Redania",
    "Temeria",
    "Aedirn",
    "Lyria",
    "Rivia",
    "Kovir",
    "Poviss",
    "Mahakam",
    "Brokilon",
    "Thanedd Island",
];

const FALLBACK_MONSTERS: &[&str] = &[
    "Drowner",
    "Nekker",
    "Ghoul",
    "Alghoul",
    "Graveir",
    "Foglet",
    "Water Hag",
    "Griffin",
    "Cockatrice",
    "Basilisk",
    "Wyvern",
    "Forktail",
    "Royal Wyvern",
    "Fiend",
    "Chort",
    "Bies",
    "Leshen",
    "Leshy",
    "Kikimore",
    "Endrega",
    "Arachas",
    "Giant Spider",
    "Wraith",
    "Noonwraith",
    "Nightwraith",
    "Barghest",
    "Devourer",
    "Rotfiend",
    "Scavenger",
    "Werewolf",
    "Ulfhedinn",
    "Fleder",
    "Garkain",
    "Katakan",
    "Ekimma",
    "Bruxa",
    "Alp",
    "Troll",
    "Ice Troll",
    "Cyclops",
    "Golem",
    "Elemental",
    "Gargoyle",
    "Relict",
    "Necrophage",
    "Ogroid",
    "Specter",
    "Vampire",
    "Cursed One",
    "Draconid",
    "Hybrid",
    "Insectoid",
    "Beast",
];

const FALLBACK_SCHOOLS: &[&str] = &[
    "Wolf School",
    "Cat School",
    "Bear School",
    "Griffin School",
    "Viper School",
    "Manticore School",
];

const FALLBACK_QUOTES: &[&str] = &[
    "Wind's howling.",
    "Damn, a storm.",
    "Place of power, gotta be.",
    " medallion's humming.",
    "How about a round of Gwent?",
    "Toss a coin to your Witcher.",
    "I am the butcher of Blaviken.",
    "Evil is evil. Lesser, greater, middling... Makes no difference.",
    "The world doesn't need a hero. It needs a professional.",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_monster() {
        assert!(!monster().is_empty());
    }

    #[test]
    fn test_school() {
        assert!(!school().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }
}

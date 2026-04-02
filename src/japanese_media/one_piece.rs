//! One Piece anime/manga generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random One Piece character
pub fn character() -> String {
    fetch_locale("one_piece.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random One Piece devil fruit
pub fn devil_fruit() -> String {
    fetch_locale("one_piece.devil_fruits", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_DEVIL_FRUITS).to_string())
}

/// Generate a random One Piece location/island
pub fn location() -> String {
    fetch_locale("one_piece.locations", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_LOCATIONS).to_string())
}

/// Generate a random One Piece quote
pub fn quote() -> String {
    fetch_locale("one_piece.quotes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_QUOTES).to_string())
}

/// Generate a random One Piece sea
pub fn sea() -> String {
    fetch_locale("one_piece.seas", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SEAS).to_string())
}

// Fallback data
const FALLBACK_CHARACTERS: &[&str] = &[
    "Monkey D. Luffy",
    "Roronoa Zoro",
    "Nami",
    "Usopp",
    "Sanji",
    "Tony Tony Chopper",
    "Nico Robin",
    "Franky",
    "Brook",
    "Jinbe",
    "Portgas D. Ace",
    "Shanks",
    "Edward Newgate (Whitebeard)",
    "Gol D. Roger",
    "Silvers Rayleigh",
    "Trafalgar D. Water Law",
    "Eustass Kid",
    "Killer",
    "Basil Hawkins",
    "Scratchmen Apoo",
    "X Drake",
    "Jewelry Bonney",
    "Capone Bege",
    "Urouge",
    "Marshall D. Teach (Blackbeard)",
    "Boa Hancock",
    "Donquixote Doflamingo",
    "Kaido",
    "Big Mom",
    "Kurozumi Orochi",
];

const FALLBACK_DEVIL_FRUITS: &[&str] = &[
    "Gomu Gomu no Mi",
    "Bara Bara no Mi",
    "Sube Sube no Mi",
    "Mera Mera no Mi",
    "Goro Goro no Mi",
    "Hie Hie no Mi",
    "Yami Yami no Mi",
    "Pika Pika no Mi",
    "Ope Ope no Mi",
    "Gura Gura no Mi",
    "Tori Tori no Mi",
    "Uo Uo no Mi",
    "Hito Hito no Mi",
    "Inu Inu no Mi",
    "Neko Neko no Mi",
    "Zo Zo no Mi",
    "Ryu Ryu no Mi",
    "Hebi Hebi no Mi",
    "Mogu Mogu no Mi",
    "Suna Suna no Mi",
    "Moku Moku no Mi",
    "Magu Magu no Mi",
    "Yuki Yuki no Mi",
];

const FALLBACK_LOCATIONS: &[&str] = &[
    "East Blue",
    "West Blue",
    "North Blue",
    "South Blue",
    "Grand Line",
    "New World",
    "Reverse Mountain",
    "Whiskey Peak",
    "Drum Island",
    "Alabasta",
    "Skypiea",
    "Water 7",
    "Enies Lobby",
    "Thriller Bark",
    "Sabaody Archipelago",
    "Fish-Man Island",
    "Dressrosa",
    "Zou",
    "Whole Cake Island",
    "Wano Country",
    "Laugh Tale",
    "Marineford",
    "Impel Down",
    "Mariejois",
];

const FALLBACK_QUOTES: &[&str] = &[
    "I am going to be King of the Pirates!",
    "If you don't take risks, you can't create a future!",
    "One Piece is real!",
    "I don't want to conquer anything. I just think the guy with the most freedom on the sea is the Pirate King.",
    "There is no such thing as being born into the world to be alone.",
    "No matter how hard or impossible it is, never lose sight of your goal.",
    "The world isn't perfect. But it's there for us, trying the best it can.",
];

const FALLBACK_SEAS: &[&str] = &[
    "East Blue",
    "West Blue",
    "North Blue",
    "South Blue",
    "Grand Line",
    "New World",
    "All Blue",
    "Calm Belt",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_devil_fruit() {
        assert!(!devil_fruit().is_empty());
    }

    #[test]
    fn test_location() {
        assert!(!location().is_empty());
    }

    #[test]
    fn test_quote() {
        assert!(!quote().is_empty());
    }

    #[test]
    fn test_sea() {
        assert!(!sea().is_empty());
    }
}

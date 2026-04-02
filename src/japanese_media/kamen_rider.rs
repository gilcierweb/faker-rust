//! Kamen Rider tokusatsu generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Kamen Rider series
pub fn series() -> String {
    fetch_locale("kamen_rider.series", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SERIES).to_string())
}

/// Generate a random Kamen Rider character
pub fn character() -> String {
    fetch_locale("kamen_rider.characters", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CHARACTERS).to_string())
}

/// Generate a random Kamen Rider transformation device
pub fn device() -> String {
    fetch_locale("kamen_rider.devices", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_DEVICES).to_string())
}

/// Generate a random Kamen Rider form
pub fn form() -> String {
    fetch_locale("kamen_rider.forms", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FORMS).to_string())
}

// Fallback data
const FALLBACK_SERIES: &[&str] = &[
    "Kamen Rider",
    "Kamen Rider V3",
    "Kamen Rider X",
    "Kamen Rider Amazon",
    "Kamen Rider Stronger",
    "Kamen Rider Black",
    "Kamen Rider Black RX",
    "Kamen Rider Kuuga",
    "Kamen Rider Agito",
    "Kamen Rider Ryuki",
    "Kamen Rider 555",
    "Kamen Rider Blade",
    "Kamen Rider Hibiki",
    "Kamen Rider Kabuto",
    "Kamen Rider Den-O",
    "Kamen Rider Kiva",
    "Kamen Rider Decade",
    "Kamen Rider W",
    "Kamen Rider OOO",
    "Kamen Rider Fourze",
    "Kamen Rider Wizard",
    "Kamen Rider Gaim",
    "Kamen Rider Drive",
    "Kamen Rider Ghost",
    "Kamen Rider Ex-Aid",
    "Kamen Rider Build",
    "Kamen Rider Zi-O",
    "Kamen Rider Zero-One",
    "Kamen Rider Saber",
    "Kamen Rider Revice",
    "Kamen Rider Geats",
];

const FALLBACK_CHARACTERS: &[&str] = &[
    "Takeshi Hongo",
    "Hayato Ichimonji",
    "Shiro Kazami",
    "Keisuke Jin",
    "Daisuke Yamamoto",
    "Shigeru Jo",
    "Kotaro Minami",
    "Yusuke Godai",
    "Shouichi Tsugami",
    "Shinji Kido",
    "Takumi Inui",
    "Kazuma Kenzaki",
    "Hitoshi Hidaka",
    "Souji Tendou",
    "Ryotaro Nogami",
    "Wataru Kurenai",
    "Tsukasa Kadoya",
    "Shotaro Hidari",
    "Philip",
    "Eiji Hino",
    "Gentaro Kisaragi",
    "Haruto Soma",
    "Kouta Kazuraba",
    "Shinnosuke Tomari",
    "Takeru Tenkuji",
    "Emu Hojo",
    "Sento Kiryu",
    "Sougo Tokiwa",
    "Aruto Hiden",
    "Touma Kamiyama",
    "Ikki Igarashi",
    "Ace Ukiyo",
];

const FALLBACK_DEVICES: &[&str] = &[
    "Typhoon",
    "Double Typhoon",
    "Ridol",
    "Sonic Wave",
    "Electrer",
    "Kingstone",
    "Arcle",
    "V Buckle",
    "Altering",
    "Evil Eye",
    "Faiz Driver",
    "Blay Buckle",
    "Henshin Onsa",
    "Kabuto Zecter",
    "Den-O Belt",
    "Kivat Belt",
    "Decadriver",
    "Double Driver",
    "OOO Driver",
    "Fourze Driver",
    "Wizardriver",
    "Sengoku Driver",
    "Drive Driver",
    "Ghost Driver",
    "Gamer Driver",
    "Build Driver",
    "Ziku-Driver",
    "Hiden Zero-One Driver",
    "Seiken Swordriver",
    "Revice Driver",
    "Desire Driver",
];

const FALLBACK_FORMS: &[&str] = &[
    "Rider Form",
    "Super Form",
    "Final Form",
    "Ultimate Form",
    "Rising Form",
    "Emperor Form",
    "Xtreme Form",
    "Putotyra Combo",
    "Cosmic States",
    "Infinity Style",
    "Kiwami Arms",
    "Type Tridoron",
    "Grateful Damashii",
    "Muteki Gamer",
    "Genius Form",
    "Grand Zi-O",
    "Ark-One",
    "Xross Saber",
    "Ultimate Revi",
    "Ultimate Vice",
    "Geats IX",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series() {
        assert!(!series().is_empty());
    }

    #[test]
    fn test_character() {
        assert!(!character().is_empty());
    }

    #[test]
    fn test_device() {
        assert!(!device().is_empty());
    }

    #[test]
    fn test_form() {
        assert!(!form().is_empty());
    }
}

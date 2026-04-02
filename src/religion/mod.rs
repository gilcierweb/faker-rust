//! Religion generator - generates religion-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random religion name
pub fn name() -> String {
    fetch_locale("religion.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random religious figure
pub fn figure() -> String {
    fetch_locale("religion.figures", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FIGURES).to_string())
}

/// Generate a random religious text
pub fn text() -> String {
    fetch_locale("religion.texts", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_TEXTS).to_string())
}

/// Generate a random religious practice
pub fn practice() -> String {
    fetch_locale("religion.practices", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PRACTICES).to_string())
}

/// Generate a random religious holiday
pub fn holiday() -> String {
    fetch_locale("religion.holidays", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HOLIDAYS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Christianity", "Islam", "Judaism", "Hinduism", "Buddhism", "Sikhism",
    "Taoism", "Confucianism", "Shinto", "Zoroastrianism", "Jainism",
    "Bahá'í Faith", "Rastafari", "Animism", "Paganism", "Wicca",
];

const FALLBACK_FIGURES: &[&str] = &[
    "Jesus Christ", "Muhammad", "Moses", "Abraham", "Buddha", "Krishna",
    "Lao Tzu", "Confucius", "Zoroaster", "Guru Nanak", "Baha'u'llah",
    "Dalai Lama", "Pope Francis", "Rumi", "Martin Luther", "Joan of Arc",
];

const FALLBACK_TEXTS: &[&str] = &[
    "Bible", "Quran", "Torah", "Vedas", "Tripitaka", "Guru Granth Sahib",
    "Tao Te Ching", "Analects", "Avesta", "Book of Mormon", "Bhagavad Gita",
];

const FALLBACK_PRACTICES: &[&str] = &[
    "Prayer", "Meditation", "Fasting", "Pilgrimage", "Baptism", "Communion",
    "Circumcision", "Bar Mitzvah", "Confirmation", "Confession", "Tithing",
    "Charity", "Chanting", "Yoga", "Ritual Sacrifice", "Divination",
];

const FALLBACK_HOLIDAYS: &[&str] = &[
    "Christmas", "Easter", "Ramadan", "Eid al-Fitr", "Yom Kippur",
    "Hanukkah", "Passover", "Diwali", "Holi", "Vesak", "Lunar New Year",
    "Day of the Dead", "All Saints' Day", "Winter Solstice", "Vernal Equinox",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_figure() {
        assert!(!figure().is_empty());
    }

    #[test]
    fn test_text() {
        assert!(!text().is_empty());
    }

    #[test]
    fn test_practice() {
        assert!(!practice().is_empty());
    }

    #[test]
    fn test_holiday() {
        assert!(!holiday().is_empty());
    }
}

//! Name generator - generates random names

use crate::base::sample;
use crate::config::FakerConfig;
use crate::locale::{fetch_locale_with_context, sample_with_resolve};

/// Generate a random full name
pub fn name() -> String {
    format!("{} {}", first_name(), last_name())
}

/// Generate a random first name
pub fn first_name() -> String {
    fetch_locale_with_context("name.first_name", "en", Some("name"))
        .map(|v| sample_with_resolve(&v, Some("name")))
        .unwrap_or_else(|| sample(FALLBACK_FIRST_NAMES).to_string())
}

/// Generate a random last name
pub fn last_name() -> String {
    fetch_locale_with_context("name.last_name", "en", Some("name"))
        .map(|v| sample_with_resolve(&v, Some("name")))
        .unwrap_or_else(|| sample(FALLBACK_LAST_NAMES).to_string())
}

/// Generate a random name prefix (Mr., Mrs., Ms., Dr., etc.)
pub fn prefix() -> String {
    fetch_locale_with_context("name.prefix", "en", Some("name"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PREFIXES).to_string())
}

/// Generate a random name suffix (Jr., Sr., I, II, III, IV, etc.)
pub fn suffix() -> String {
    fetch_locale_with_context("name.suffix", "en", Some("name"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_SUFFIXES).to_string())
}

/// Generate random initials
pub fn initials(num: usize) -> String {
    let config = FakerConfig::current();
    (0..num)
        .map(|_| config.rand_char(&crate::base::U_LETTERS))
        .collect()
}

/// Generate a random name with middle name
pub fn name_with_middle() -> String {
    format!("{} {} {}", first_name(), first_name(), last_name())
}

/// Generate a random male first name
pub fn male_first_name() -> String {
    fetch_locale_with_context("name.male_first_name", "en", Some("name"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_MALE_FIRST_NAMES).to_string())
}

/// Generate a random female first name
pub fn female_first_name() -> String {
    fetch_locale_with_context("name.female_first_name", "en", Some("name"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_FEMALE_FIRST_NAMES).to_string())
}

/// Generate a random gender-neutral first name
pub fn neutral_first_name() -> String {
    fetch_locale_with_context("name.neutral_first_name", "en", Some("name"))
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NEUTRAL_FIRST_NAMES).to_string())
}

// Static fallback data - used when locale files are not available
// Note: Locale loading infrastructure is in place but needs refinement
const FALLBACK_FIRST_NAMES: &[&str] = &[
    "James",
    "Mary",
    "Robert",
    "Patricia",
    "John",
    "Jennifer",
    "Michael",
    "Linda",
    "David",
    "Elizabeth",
    "William",
    "Barbara",
    "Richard",
    "Susan",
    "Joseph",
    "Jessica",
    "Thomas",
    "Sarah",
    "Charles",
    "Karen",
    "Christopher",
    "Nancy",
    "Daniel",
    "Lisa",
    "Matthew",
    "Betty",
    "Anthony",
    "Margaret",
    "Mark",
    "Sandra",
    "Donald",
    "Ashley",
];

const FALLBACK_MALE_FIRST_NAMES: &[&str] = &[
    "James",
    "John",
    "Robert",
    "Michael",
    "William",
    "David",
    "Richard",
    "Joseph",
    "Thomas",
    "Charles",
    "Christopher",
    "Daniel",
    "Matthew",
    "Anthony",
    "Mark",
    "Donald",
    "Steven",
    "Paul",
    "Andrew",
    "Joshua",
    "Kenneth",
    "Kevin",
    "Brian",
    "George",
];

const FALLBACK_FEMALE_FIRST_NAMES: &[&str] = &[
    "Mary",
    "Patricia",
    "Jennifer",
    "Linda",
    "Barbara",
    "Elizabeth",
    "Susan",
    "Jessica",
    "Sarah",
    "Karen",
    "Nancy",
    "Lisa",
    "Betty",
    "Margaret",
    "Sandra",
    "Ashley",
];

const FALLBACK_NEUTRAL_FIRST_NAMES: &[&str] = &[
    "Alex", "Jordan", "Taylor", "Morgan", "Casey", "Riley", "Jamie", "Avery",
];

const FALLBACK_LAST_NAMES: &[&str] = &[
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzalez",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
    "Lee",
    "Perez",
    "Thompson",
    "White",
];

const FALLBACK_PREFIXES: &[&str] = &["Mr.", "Mrs.", "Ms.", "Miss", "Dr.", "Prof."];

const FALLBACK_SUFFIXES: &[&str] = &["Jr.", "Sr.", "I", "II", "III", "IV", "V"];

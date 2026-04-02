//! Name generator - generates random names

use crate::base::sample;

/// Generate a random full name
///
/// # Example
///
/// ```rust
/// let name = faker::Name::name();
/// println!("{}", name); // e.g., "John Smith"
/// ```
pub fn name() -> String {
    format!("{} {}", first_name(), last_name())
}

/// Generate a random first name
pub fn first_name() -> String {
    sample(&FIRST_NAMES).to_string()
}

/// Generate a random last name
pub fn last_name() -> String {
    sample(&LAST_NAMES).to_string()
}

/// Generate a random name prefix (Mr., Mrs., Ms., Dr., etc.)
pub fn prefix() -> String {
    sample(&PREFIXES).to_string()
}

/// Generate a random name suffix (Jr., Sr., I, II, III, IV, etc.)
pub fn suffix() -> String {
    sample(&SUFFIXES).to_string()
}

/// Generate random initials
///
/// # Arguments
///
/// * `num` - Number of initials to generate (default: 3)
pub fn initials(num: usize) -> String {
    use crate::config::FakerConfig;
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
    sample(&MALE_FIRST_NAMES).to_string()
}

/// Generate a random female first name
pub fn female_first_name() -> String {
    sample(&FEMALE_FIRST_NAMES).to_string()
}

/// Generate a random gender-neutral first name
pub fn neutral_first_name() -> String {
    sample(&NEUTRAL_FIRST_NAMES).to_string()
}

// ============================================================================
// Data - English (en) locale
// ============================================================================

const FIRST_NAMES: &[&str] = &[
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
    "Steven",
    "Kimberly",
    "Paul",
    "Emily",
    "Andrew",
    "Donna",
    "Joshua",
    "Michelle",
    "Kenneth",
    "Dorothy",
    "Kevin",
    "Carol",
    "Brian",
    "Amanda",
    "George",
    "Melissa",
    "Timothy",
    "Deborah",
    "Ronald",
    "Stephanie",
    "Edward",
    "Rebecca",
    "Jason",
    "Sharon",
    "Jeffrey",
    "Laura",
    "Ryan",
    "Cynthia",
    "Jacob",
    "Kathleen",
    "Gary",
    "Amy",
    "Nicholas",
    "Angela",
    "Eric",
    "Shirley",
    "Jonathan",
    "Anna",
    "Stephen",
    "Brenda",
    "Larry",
    "Pamela",
    "Justin",
    "Emma",
    "Scott",
    "Nicole",
    "Brandon",
    "Helen",
    "Benjamin",
    "Samantha",
    "Samuel",
    "Katherine",
    "Raymond",
    "Christine",
    "Gregory",
    "Debra",
    "Frank",
    "Rachel",
    "Alexander",
    "Carolyn",
    "Patrick",
    "Janet",
    "Jack",
    "Catherine",
    "Dennis",
    "Maria",
    "Jerry",
    "Heather",
    "Tyler",
    "Diane",
    "Aaron",
    "Ruth",
    "Jose",
    "Julie",
    "Adam",
    "Olivia",
    "Nathan",
    "Joyce",
    "Henry",
    "Virginia",
    "Zachary",
    "Virginia",
    "Douglas",
    "Kelly",
    "Peter",
    "Lauren",
    "Kyle",
    "Christina",
];

const MALE_FIRST_NAMES: &[&str] = &[
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
    "Timothy",
    "Ronald",
    "Edward",
    "Jason",
    "Jeffrey",
    "Ryan",
    "Jacob",
    "Nicholas",
    "Gary",
    "Nicholas",
    "Eric",
    "Jonathan",
    "Stephen",
    "Larry",
    "Justin",
    "Scott",
    "Brandon",
    "Benjamin",
    "Samuel",
    "Raymond",
    "Gregory",
    "Frank",
    "Alexander",
    "Patrick",
    "Jack",
    "Dennis",
    "Jerry",
    "Tyler",
    "Aaron",
    "Jose",
    "Adam",
    "Nathan",
    "Henry",
    "Zachary",
    "Douglas",
    "Peter",
    "Kyle",
    "Noah",
    "Ethan",
    "Jeremy",
];

const FEMALE_FIRST_NAMES: &[&str] = &[
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
    "Kimberly",
    "Emily",
    "Donna",
    "Michelle",
    "Dorothy",
    "Carol",
    "Amanda",
    "Melissa",
    "Deborah",
    "Stephanie",
    "Rebecca",
    "Sharon",
    "Laura",
    "Cynthia",
    "Kathleen",
    "Amy",
    "Angela",
    "Shirley",
    "Anna",
    "Brenda",
    "Pamela",
    "Emma",
    "Nicole",
    "Helen",
    "Samantha",
    "Katherine",
    "Christine",
    "Debra",
    "Rachel",
    "Carolyn",
    "Janet",
    "Catherine",
    "Maria",
    "Heather",
    "Diane",
    "Julie",
    "Olivia",
    "Joyce",
    "Virginia",
    "Victoria",
    "Kelly",
    "Lauren",
    "Christina",
    "Joan",
    "Evelyn",
    "Judith",
    "Megan",
    "Andrea",
];

const NEUTRAL_FIRST_NAMES: &[&str] = &[
    "Alex", "Jordan", "Taylor", "Morgan", "Casey", "Riley", "Jamie", "Avery", "Quinn", "Skyler",
    "Dakota", "Reese", "Finley", "Rowan", "Sage", "Phoenix", "River", "Drew", "Cameron", "Hayden",
    "Parker", "Emerson", "Logan", "Sawyer", "Sydney",
];

const LAST_NAMES: &[&str] = &[
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
    "Harris",
    "Sanchez",
    "Clark",
    "Ramirez",
    "Lewis",
    "Robinson",
    "Walker",
    "Young",
    "Allen",
    "King",
    "Wright",
    "Scott",
    "Torres",
    "Nguyen",
    "Hill",
    "Flores",
    "Green",
    "Adams",
    "Nelson",
    "Baker",
    "Hall",
    "Rivera",
    "Campbell",
    "Mitchell",
    "Carter",
    "Roberts",
    "Gomez",
    "Phillips",
    "Evans",
    "Turner",
    "Diaz",
    "Parker",
    "Cruz",
    "Edwards",
    "Collins",
    "Reyes",
    "Stewart",
    "Morris",
    "Morales",
    "Murphy",
    "Cook",
    "Rogers",
    "Gutierrez",
    "Ortiz",
    "Morgan",
    "Cooper",
    "Peterson",
    "Bailey",
    "Reed",
    "Kelly",
    "Howard",
    "Ramos",
    "Kim",
    "Cox",
    "Ward",
    "Richardson",
    "Watson",
    "Brooks",
    "Chavez",
    "Wood",
    "James",
    "Bennett",
    "Gray",
    "Mendoza",
    "Ruiz",
    "Hughes",
    "Price",
    "Alvarez",
    "Castillo",
    "Sanders",
    "Patel",
    "Myers",
    "Long",
    "Ross",
    "Foster",
    "Jimenez",
];

const PREFIXES: &[&str] = &["Mr.", "Mrs.", "Ms.", "Miss", "Dr.", "Prof."];

const SUFFIXES: &[&str] = &["Jr.", "Sr.", "I", "II", "III", "IV", "V"];

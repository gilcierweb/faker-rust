//! Address generator - generates random addresses

use crate::base::{bothify, sample};

/// Generate a random city name
pub fn city() -> String {
    sample(&CITIES).to_string()
}

/// Generate a random street name
pub fn street_name() -> String {
    format!("{} {}", sample(&STREET_PREFIXES), sample(&STREET_SUFFIXES))
}

/// Generate a random street address
pub fn street_address() -> String {
    format!("{} {}", sample(&STREET_NUMBERS), street_name())
}

/// Generate a random secondary address (apt, suite, etc.)
pub fn secondary_address() -> String {
    let config = crate::config::FakerConfig::current();
    let pattern = sample(&["Apt. ###", "Suite ###", "Floor #", "Unit #", "###"]);
    crate::base::numerify(pattern)
}

/// Generate a random zip code
pub fn zip_code() -> String {
    bothify("#####")
}

/// Generate a random zip code with extension
pub fn zip_code_with_extension() -> String {
    bothify("#####-####")
}

/// Generate a random country name
pub fn country() -> String {
    sample(&COUNTRIES).to_string()
}

/// Generate a random country code (ISO 3166-1 alpha-2)
pub fn country_code() -> String {
    sample(&COUNTRY_CODES).to_string()
}

/// Generate a random full address
pub fn full_address() -> String {
    let pattern = crate::config::FakerConfig::current().rand_range(0, 4);
    
    match pattern {
        0 => format!("{}, {}", street_address(), city_state_zip()),
        1 => format!("{}\n{}", street_address(), city_state_zip()),
        2 => format!("{}, {}", secondary_address(), city_state_zip()),
        _ => format!("{}\n{}, {}", street_address(), secondary_address(), city_state_zip()),
    }
}
}

/// Generate city, state, and zip combined
fn city_state_zip() -> String {
    format!("{}, {} {}", city(), sample(&US_STATES), zip_code())
}

/// Generate a random time zone
pub fn time_zone() -> String {
    sample(&TIME_ZONES).to_string()
}

/// Generate a random latitude
pub fn latitude() -> String {
    let config = crate::config::FakerConfig::current();
    let lat = config.rand_range(0, 180_000_000) as f64 - 90_000_000.0;
    format!("{:.6}", lat / 1_000_000.0)
}

/// Generate a random longitude
pub fn longitude() -> String {
    let config = crate::config::FakerConfig::current();
    let lon = config.rand_range(0, 360_000_000) as f64 - 180_000_000.0;
    format!("{:.6}", lon / 1_000_000.0)
}

// Data

const CITIES: &[&str] = &[
    "New York",
    "Los Angeles",
    "Chicago",
    "Houston",
    "Phoenix",
    "Philadelphia",
    "San Antonio",
    "San Diego",
    "Dallas",
    "San Jose",
    "Austin",
    "Jacksonville",
    "Fort Worth",
    "Columbus",
    "Charlotte",
    "San Francisco",
    "Indianapolis",
    "Seattle",
    "Denver",
    "Washington",
    "Boston",
    "El Paso",
    "Nashville",
    "Detroit",
    "Oklahoma City",
    "Portland",
    "Las Vegas",
    "Memphis",
    "Louisville",
    "Baltimore",
    "Milwaukee",
    "Albuquerque",
    "Tucson",
    "Fresno",
    "Sacramento",
    "Kansas City",
    "Mesa",
    "Atlanta",
    "Miami",
    "Raleigh",
    "Omaha",
    "Colorado Springs",
    "Long Beach",
    "Virginia Beach",
    "Oakland",
    "Minneapolis",
    "Tulsa",
    "Tampa",
    "Arlington",
    "New Orleans",
];

const STREET_PREFIXES: &[&str] = &[
    "Main",
    "Oak",
    "Pine",
    "Maple",
    "Cedar",
    "Elm",
    "Washington",
    "Lake",
    "Hill",
    "Park",
    "Forest",
    "River",
    "Spring",
    "Valley",
    "Green",
    "Woodland",
    "Highland",
    "Sunset",
    "Lakeview",
    "Riverside",
    "Forest",
    "Meadow",
    "Mountain",
    "Church",
];

const STREET_SUFFIXES: &[&str] = &[
    "Street",
    "Avenue",
    "Road",
    "Boulevard",
    "Drive",
    "Lane",
    "Way",
    "Court",
    "Place",
    "Circle",
    "Terrace",
    "Highway",
    "Parkway",
    "Trail",
    "Loop",
    "Square",
];

const STREET_NUMBERS: &[&str] = &[
    "1", "2", "3", "4", "5", "10", "11", "12", "13", "14", "15", "20", "21", "22", "23", "24",
    "25", "30", "31", "32", "33", "34", "35", "40", "41", "42", "43", "44", "45", "50", "51", "52",
    "53", "54", "55", "100", "101", "102", "110", "111", "112", "120", "121", "122", "130", "131",
    "200", "201", "210", "211", "220", "221", "230", "231", "232", "240", "241", "300", "301",
    "310", "311", "320", "321", "330", "331", "332", "340", "341", "400", "401", "410", "411",
    "420", "421", "430", "431", "432", "440", "441", "500", "501", "510", "520", "530", "540",
    "550", "600", "700", "800", "900",
];

const US_STATES: &[&str] = &[
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
];

const COUNTRIES: &[&str] = &[
    "Afghanistan",
    "Albania",
    "Algeria",
    "Argentina",
    "Australia",
    "Austria",
    "Bangladesh",
    "Belgium",
    "Brazil",
    "Canada",
    "Chile",
    "China",
    "Colombia",
    "Croatia",
    "Czech Republic",
    "Denmark",
    "Egypt",
    "Finland",
    "France",
    "Germany",
    "Greece",
    "Hong Kong",
    "Hungary",
    "India",
    "Indonesia",
    "Iran",
    "Ireland",
    "Israel",
    "Italy",
    "Japan",
    "Kenya",
    "Malaysia",
    "Mexico",
    "Morocco",
    "Netherlands",
    "New Zealand",
    "Nigeria",
    "Norway",
    "Pakistan",
    "Peru",
    "Philippines",
    "Poland",
    "Portugal",
    "Romania",
    "Russia",
    "Saudi Arabia",
    "Singapore",
    "South Africa",
    "South Korea",
    "Spain",
    "Sweden",
    "Switzerland",
    "Taiwan",
    "Thailand",
    "Turkey",
    "Ukraine",
    "United Arab Emirates",
    "United Kingdom",
    "United States",
    "Venezuela",
    "Vietnam",
];

const COUNTRY_CODES: &[&str] = &[
    "US", "CA", "GB", "DE", "FR", "JP", "CN", "IN", "BR", "AU", "ES", "IT", "KR", "MX", "NL", "SE",
    "CH", "BE", "AT", "NO", "DK", "FI", "IE", "NZ", "SG", "PT", "PL", "AR", "CL", "CO", "CZ", "HU",
    "RO", "RU", "TH", "TR", "ZA", "ID", "MY", "PH",
];

const TIME_ZONES: &[&str] = &[
    "America/New_York",
    "America/Chicago",
    "America/Denver",
    "America/Los_Angeles",
    "America/Phoenix",
    "America/Anchorage",
    "Pacific/Honolulu",
    "America/Sao_Paulo",
    "Europe/London",
    "Europe/Paris",
    "Europe/Berlin",
    "Europe/Rome",
    "Europe/Madrid",
    "Europe/Amsterdam",
    "Europe/Stockholm",
    "Europe/Vienna",
    "Europe/Brussels",
    "Asia/Tokyo",
    "Asia/Shanghai",
    "Asia/Hong_Kong",
    "Asia/Singapore",
    "Asia/Seoul",
    "Asia/Bangkok",
    "Asia/Dubai",
    "Asia/Mumbai",
    "Asia/Jakarta",
    "Australia/Sydney",
    "Australia/Melbourne",
    "Pacific/Auckland",
    "Africa/Cairo",
    "Africa/Johannesburg",
];

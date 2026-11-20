//! Company generator - generates company names
#![allow(dead_code)]

use crate::base::sample;
use crate::config::FakerConfig;
use crate::locale::{fetch_locale, sample_with_resolve};

/// Generate a random company name
pub fn name() -> String {
    let prefix = fetch_locale("company.name", "en")
        .map(|v| sample_with_resolve(&v, Some("company")))
        .unwrap_or_else(|| {
            format!(
                "{} {}",
                sample(FALLBACK_NAME_PREFIXES),
                sample(FALLBACK_NAME_SUFFIXES)
            )
        });

    // If locale returns full names, use one; otherwise combine prefix + suffix
    if prefix.contains(' ') || prefix.len() > 20 {
        prefix
    } else {
        let suffix = fetch_locale("company.suffix", "en")
            .map(|v| sample_with_resolve(&v, Some("company")))
            .unwrap_or_else(|| "Inc".to_string());
        format!("{} {}", prefix, suffix)
    }
}

/// Generate a random company suffix (Inc, LLC, etc.)
pub fn suffix() -> String {
    fetch_locale("company.suffix", "en")
        .map(|v| sample_with_resolve(&v, Some("company")))
        .unwrap_or_else(|| "Inc".to_string())
}

/// Generate a random company industry
pub fn industry() -> String {
    fetch_locale("company.industry", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Technology".to_string())
}

/// Generate a company catch phrase (three buzzwords)
pub fn catch_phrase() -> String {
    let config = FakerConfig::current();

    fetch_locale("company.buzzwords", "en")
        .map(|v| {
            let all: Vec<String> = v
                .iter()
                .flat_map(|s| s.split(',').map(|p| p.trim().to_string()))
                .collect();
            if all.len() >= 3 {
                let i1 = config.rand_usize(all.len());
                let i2 = config.rand_usize(all.len());
                let i3 = config.rand_usize(all.len());
                format!("{} {} {}", all[i1], all[i2], all[i3])
            } else {
                format!(
                    "{} {} {}",
                    sample(FALLBACK_BUZZWORDS),
                    sample(FALLBACK_BUZZWORDS),
                    sample(FALLBACK_BUZZWORDS)
                )
            }
        })
        .unwrap_or_else(|| {
            format!(
                "{} {} {}",
                sample(FALLBACK_BUZZWORDS),
                sample(FALLBACK_BUZZWORDS),
                sample(FALLBACK_BUZZWORDS)
            )
        })
}

/// Generate a single random buzzword
pub fn buzzword() -> String {
    fetch_locale("company.buzzwords", "en")
        .map(|v| {
            let all: Vec<String> = v
                .iter()
                .flat_map(|s| s.split(',').map(|p| p.trim().to_string()))
                .collect();
            sample(&all)
        })
        .unwrap_or_else(|| sample(FALLBACK_BUZZWORDS).to_string())
}

/// Generate company BS (three random words)
pub fn bs() -> String {
    fetch_locale("company.bs", "en")
        .map(|v| {
            let parts: Vec<String> = v
                .iter()
                .flat_map(|s| s.split(',').map(|p| p.trim().to_string()))
                .collect();
            if parts.len() >= 3 {
                let config = FakerConfig::current();
                let i1 = config.rand_usize(parts.len());
                let i2 = config.rand_usize(parts.len());
                let i3 = config.rand_usize(parts.len());
                format!("{} {} {}", parts[i1], parts[i2], parts[i3])
            } else {
                let p1 = sample(FALLBACK_BS_WORDS[0]);
                let p2 = sample(FALLBACK_BS_WORDS[1]);
                let p3 = sample(FALLBACK_BS_WORDS[2]);
                format!("{} {} {}", p1, p2, p3)
            }
        })
        .unwrap_or_else(|| {
            let p1 = sample(FALLBACK_BS_WORDS[0]);
            let p2 = sample(FALLBACK_BS_WORDS[1]);
            let p3 = sample(FALLBACK_BS_WORDS[2]);
            format!("{} {} {}", p1, p2, p3)
        })
}

/// Generate a company EIN (Employer Identification Number)
pub fn ein() -> String {
    let config = FakerConfig::current();
    let num = config.rand_range(0, 1_000_000_000);
    let formatted = format!("{:09}", num);
    format!("{}-{}", &formatted[..2], &formatted[2..])
}

/// Generate a company DUNS number
pub fn duns_number() -> String {
    let config = FakerConfig::current();
    let num = config.rand_range(0, 1_000_000_000);
    let formatted = format!("{:09}", num);
    format!(
        "{}-{}-{}",
        &formatted[..2],
        &formatted[2..5],
        &formatted[5..]
    )
}

/// Generate a company logo URL
pub fn logo() -> String {
    let config = FakerConfig::current();
    let num = config.rand_range(1, 14);
    format!(
        "https://pigment.github.io/fake-logos/logos/medium/color/{}.png",
        num
    )
}

/// Generate a company type
pub fn company_type() -> String {
    fetch_locale("company.type", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_COMPANY_TYPES).to_string())
}

/// Generate a company profession
pub fn profession() -> String {
    fetch_locale("company.profession", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PROFESSIONS).to_string())
}

/// Generate a company department
pub fn department() -> String {
    fetch_locale("company.department", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_DEPARTMENTS).to_string())
}

const FALLBACK_NAME_PREFIXES: &[&str] = &[
    "Acme",
    "Global",
    "Tech",
    "Data",
    "Cloud",
    "Soft",
    "Net",
    "Web",
    "Future",
    "Alpha",
    "Prime",
    "Summit",
    "Apex",
    "Nexus",
    "Dynamic",
    "Innovative",
    "Advanced",
];

const FALLBACK_NAME_SUFFIXES: &[&str] = &[
    "Solutions",
    "Systems",
    "Technologies",
    "Dynamics",
    "Industries",
    "Group",
    "Corporation",
    "Enterprises",
    "Partners",
    "Services",
    "Labs",
    "Works",
];

const FALLBACK_BS_WORDS: &[&[&str]] = &[
    &[
        "implement",
        "utilize",
        "integrate",
        "streamline",
        "optimize",
    ],
    &["synergistic", "strategic", "vibrant", "robust", "scalable"],
    &[
        "solutions",
        "systems",
        "networks",
        "platforms",
        "infrastructures",
    ],
];

const FALLBACK_COMPANY_SUFFIXES: &[&str] = &[
    "Inc.",
    "LLC",
    "Corp.",
    "Co.",
    "Ltd.",
    "Group",
    "Holdings",
    "Enterprises",
];

const FALLBACK_COMPANY_TYPES: &[&str] = &[
    "Partnership",
    "Corporation",
    "LLC",
    "Sole Proprietorship",
    "Non-Profit",
    "Cooperative",
    "S-Corp",
    "C-Corp",
];

const FALLBACK_PROFESSIONS: &[&str] = &[
    "accountant",
    "actor",
    "actuary",
    "administrator",
    "adviser",
    "analyst",
    "architect",
    "artist",
    "assistant",
    "consultant",
    "coordinator",
    "counselor",
    "developer",
    "director",
    "engineer",
    "executive",
    "manager",
    "officer",
    "producer",
    "professor",
    "specialist",
    "supervisor",
    "teacher",
    "technician",
];

const FALLBACK_DEPARTMENTS: &[&str] = &[
    "Information Technology",
    "Human Resources",
    "Marketing",
    "Sales",
    "Finance",
    "Operations",
    "Legal",
    "Customer Service",
    "Research and Development",
    "Product Management",
    "Engineering",
    "Design",
    "Quality Assurance",
    "Business Development",
    "Supply Chain",
    "Procurement",
    "Logistics",
];

const FALLBACK_BUZZWORDS: &[&str] = &[
    "Adaptive",
    "Advanced",
    "Ameliorated",
    "Automated",
    "Balanced",
    "Centralized",
    "Cloned",
    "Compatible",
    "Configurable",
    "Cross-platform",
    "Customer-focused",
    "Customizable",
    "Decentralized",
    "De-engineered",
    "Devolved",
    "Distributed",
    "Enhanced",
    "Enterprise-wide",
    "Ergonomic",
    "Executive",
];

// --- gem parity generators (automated, locale-backed) ---
fn parity_pick(key: &str) -> String {
    crate::locale::fetch_locale(key, "en")
        .map(|v| crate::base::sample(&v))
        .unwrap_or_default()
}

/// Gem-compatible generator: `company.sic_code`.
pub fn sic_code() -> String {
    parity_pick("company.sic_code")
}

/// Gem-compatible generator: `company.type`.
pub fn r#type() -> String {
    parity_pick("company.type")
}

// --- gem parity generators (algorithmic identifiers) ---

fn config() -> crate::config::FakerConfig {
    crate::config::FakerConfig::current()
}

fn random_digits(len: usize) -> String {
    (0..len)
        .map(|_| char::from(b'0' + config().rand_u32(10) as u8))
        .collect()
}

fn random_number_in(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    min + config().rand_u32(max - min + 1)
}

fn number_string_in(min_digits: usize, max_digits: usize) -> String {
    let len = random_number_in(min_digits as u32, max_digits as u32) as usize;
    let mut result = random_number_in(1, 9).to_string();
    result.push_str(&random_digits(len - 1));
    result
}

fn random_uppercase(len: usize) -> String {
    (0..len)
        .map(|_| char::from(b'A' + config().rand_u32(26) as u8))
        .collect()
}

fn luhn_algorithm(number: &str) -> u32 {
    let mut sum = 0u32;
    for (index, character) in number.chars().rev().enumerate() {
        let digit = character.to_digit(10).unwrap_or(0);
        let multiplied = if index % 2 == 0 { digit * 2 } else { digit };
        sum += multiplied
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>();
    }
    if sum.is_multiple_of(10) {
        0
    } else {
        (sum / 10 + 1) * 10 - sum
    }
}

fn mod11(number: &str) -> Option<u32> {
    let weights = [2, 3, 4, 5, 6, 7, 2, 3, 4, 5, 6, 7, 2, 3, 4, 5, 6, 7];
    let sum: u32 = number
        .chars()
        .rev()
        .enumerate()
        .map(|(index, character)| character.to_digit(10).unwrap_or(0) * weights[index])
        .sum();
    match sum % 11 {
        0 => Some(0),
        1 => None,
        remainder => Some(11 - remainder),
    }
}

fn abn_checksum(abn: &str) -> u32 {
    let weights = [10, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    abn.chars()
        .zip(weights)
        .map(|(character, weight)| character.to_digit(10).unwrap_or(0) * weight)
        .sum()
}

fn weight_sum(array: &[u32], weights: &[u32]) -> u32 {
    array
        .iter()
        .zip(weights)
        .map(|(value, weight)| value * weight)
        .sum()
}

fn collect_regon_sum(array: &[u32]) -> u32 {
    let weights: Vec<u32> = if array.len() == 9 {
        vec![8, 9, 2, 3, 4, 5, 6, 7]
    } else {
        vec![2, 4, 8, 5, 0, 9, 7, 3, 6, 1, 2, 4, 8]
    };
    let sum = weight_sum(array, &weights) % 11;
    if sum == 10 { 0 } else { sum }
}

fn spanish_b_algorithm(value: u32) -> u32 {
    let result = value * 2;
    if result < 10 {
        result
    } else {
        result / 10 + result % 10
    }
}

fn spanish_cif_control_digit(organization_type: char, code: &str) -> String {
    let letters = ['J', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
    let mut control = 0u32;
    for (index, character) in code.chars().enumerate() {
        let value = character.to_digit(10).unwrap_or(0);
        control += if (index + 1) % 2 == 0 {
            value
        } else {
            spanish_b_algorithm(value)
        };
    }
    let mut digit = control % 10;
    if digit != 0 {
        digit = 10 - digit;
    }
    if ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'U', 'V'].contains(&organization_type) {
        digit.to_string()
    } else {
        letters[digit as usize].to_string()
    }
}

fn inn_checksum(factor: &[u32], number: &str) -> String {
    let sum: u32 = factor
        .iter()
        .enumerate()
        .map(|(index, weight)| {
            weight
                * number
                    .chars()
                    .nth(index)
                    .and_then(|c| c.to_digit(10))
                    .unwrap_or(0)
        })
        .sum();
    ((sum % 11) % 10).to_string()
}

fn gst_checksum(base: &str) -> char {
    let chars: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let sum: usize = base
        .chars()
        .enumerate()
        .map(|(index, character)| {
            let value = chars.iter().position(|c| *c == character).unwrap_or(0);
            let product = value * if index % 2 == 1 { 2 } else { 1 };
            product / chars.len() + product % chars.len()
        })
        .sum();
    chars[(chars.len() - (sum % chars.len())) % chars.len()]
}

/// Gem-compatible generator: Spanish organisation number.
pub fn spanish_organisation_number(organization_type: Option<&str>) -> String {
    let letters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'N', 'P', 'Q', 'R', 'S', 'U', 'V', 'W',
    ];
    let org = match organization_type
        .and_then(|value| value.chars().next())
        .filter(|c| letters.contains(c))
    {
        Some(letter) => letter,
        None => crate::base::sample(&letters),
    };
    let code = random_digits(7);
    let control = spanish_cif_control_digit(org, &code);
    format!("{org}{code}{control}")
}

/// Gem-compatible generator: Swedish organisation number.
pub fn swedish_organisation_number() -> String {
    let first = crate::base::sample(&[1u32, 2, 3, 5, 6, 7, 8, 9]);
    let second = config().rand_u32(10);
    let third = random_number_in(2, 9);
    let base = format!("{first}{second}{third}{}", random_digits(6));
    format!("{base}{}", luhn_algorithm(&base))
}

/// Gem-compatible generator: Czech organisation number.
pub fn czech_organisation_number() -> String {
    let weights = [8u32, 7, 6, 5, 4, 3, 2];
    let mut result = Vec::new();
    let mut sum = 0u32;
    for weight in weights {
        let digit = config().rand_u32(10);
        sum += weight * digit;
        result.push(digit);
    }
    result.push((11 - (sum % 11)) % 10);
    result.iter().map(u32::to_string).collect()
}

/// Gem-compatible generator: French SIREN number.
pub fn french_siren_number() -> String {
    let base = random_digits(8);
    format!("{base}{}", luhn_algorithm(&base))
}

/// Gem-compatible generator: French SIRET number.
pub fn french_siret_number() -> String {
    let location = format!("{:04}", config().rand_u32(100));
    let base = format!("{}{location}", french_siren_number());
    format!("{base}{}", luhn_algorithm(&base))
}

/// Gem-compatible generator: Norwegian organisation number.
pub fn norwegian_organisation_number() -> String {
    loop {
        let first = crate::base::sample(&[8u32, 9]);
        let base = format!("{first}{}", random_digits(7));
        if let Some(check) = mod11(&base) {
            return format!("{base}{check}");
        }
    }
}

/// Gem-compatible generator: Australian business number.
pub fn australian_business_number() -> String {
    let base = random_digits(9);
    let check = 99 - (abn_checksum(&format!("00{base}")) % 89);
    format!("{check}{base}")
}

/// Gem-compatible generator: Polish taxpayer identification number.
pub fn polish_taxpayer_identification_number() -> String {
    let weights = [6u32, 5, 7, 2, 3, 4, 5, 6, 7];
    loop {
        let mut result: Vec<u32> = (0..3).map(|_| random_number_in(1, 9)).collect();
        result.extend((0..7).map(|_| config().rand_u32(10)));
        if weight_sum(&result, &weights) % 11 == result[9] {
            return result.iter().map(u32::to_string).collect();
        }
    }
}

/// Gem-compatible generator: Polish register of national economy number.
pub fn polish_register_of_national_economy(length: usize) -> String {
    loop {
        let digits: Vec<u32> = (0..length).map(|_| config().rand_u32(10)).collect();
        if let Some(last) = digits.last()
            && collect_regon_sum(&digits) == *last
        {
            return digits.iter().map(u32::to_string).collect();
        }
    }
}

/// Gem-compatible generator: South African private company registration number.
pub fn south_african_pty_ltd_registration_number() -> String {
    format!("{}/{}/07", random_digits(4), number_string_in(4, 10))
}

/// Gem-compatible generator: South African close corporation registration number.
pub fn south_african_close_corporation_registration_number() -> String {
    let prefix = if config().rand_bool() {
        format!("CK{}", random_digits(2))
    } else {
        random_digits(4)
    };
    format!("{prefix}/{}/23", number_string_in(4, 10))
}

/// Gem-compatible generator: South African listed company registration number.
pub fn south_african_listed_company_registration_number() -> String {
    format!("{}/{}/06", random_digits(4), number_string_in(4, 10))
}

/// Gem-compatible generator: South African trust registration number.
pub fn south_african_trust_registration_number() -> String {
    format!(
        "IT{}/{}",
        random_number_in(10, 9999),
        number_string_in(2, 10)
    )
}

/// Gem-compatible generator: Brazilian company number (CNPJ).
pub fn brazilian_company_number(formatted: bool) -> String {
    let mut digits: Vec<u32> = (0..8).map(|_| config().rand_u32(10)).collect();
    digits.extend([0, 0, 0, random_number_in(1, 9)]);
    let factors = [5u32, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2, 6];
    for _ in 0..2 {
        let checksum: u32 = digits
            .iter()
            .zip(factors.iter().cycle())
            .map(|(digit, factor)| digit * factor)
            .sum::<u32>()
            % 11;
        digits.push(if checksum < 2 { 0 } else { 11 - checksum });
    }
    let number: String = digits.iter().map(u32::to_string).collect();
    if formatted {
        format!(
            "{}.{}.{}/{}-{}",
            &number[0..2],
            &number[2..5],
            &number[5..8],
            &number[8..12],
            &number[12..14]
        )
    } else {
        number
    }
}

/// Gem-compatible generator: Russian tax number (INN).
pub fn russian_tax_number(region: Option<&str>, legal: bool) -> String {
    let n10 = [2u32, 4, 10, 3, 5, 9, 4, 6, 8];
    let n11 = [7u32, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    let n12 = [3u32, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    let region = region
        .map(str::to_string)
        .unwrap_or_else(|| format!("{:02}", random_number_in(1, 92)));
    if legal {
        let number = format!("{}{}", region, random_number_in(1_000_000, 9_999_999));
        format!("{}{}", number, inn_checksum(&n10, &number))
    } else {
        let number = format!("{}{}", region, random_number_in(10_000_000, 99_999_999));
        let first = inn_checksum(&n11, &number);
        let second = inn_checksum(&n12, &format!("{number}{first}"));
        format!("{number}{first}{second}")
    }
}

/// Gem-compatible generator: Indian Goods and Services Tax number.
pub fn indian_gst_number(state_code: Option<&str>) -> String {
    let state = state_code
        .map(str::to_string)
        .unwrap_or_else(|| format!("{:02}", random_number_in(2, 39)));
    let taxpayers = ['A', 'B', 'C', 'F', 'G', 'H', 'L', 'J', 'P', 'T', 'K'];
    let taxpayer = format!(
        "{}{}{}{:04}{}",
        random_uppercase(3),
        crate::base::sample(&taxpayers),
        random_uppercase(1),
        config().rand_u32(10000),
        random_uppercase(1)
    );
    let registration = config().rand_u32(10);
    let base = format!("{state}{taxpayer}{registration}");
    format!("{base}Z{}", gst_checksum(&base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let n = name();
        assert!(!n.is_empty());
    }
}

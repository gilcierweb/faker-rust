//! Company generator - generates company names

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
                &FALLBACK_NAME_PREFIXES[0], &FALLBACK_NAME_SUFFIXES[0]
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
                let i1 = config.rand_range(0, all.len() as u32) as usize;
                let i2 = config.rand_range(0, all.len() as u32) as usize;
                let i3 = config.rand_range(0, all.len() as u32) as usize;
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
                let i1 = config.rand_range(0, parts.len() as u32) as usize;
                let i2 = config.rand_range(0, parts.len() as u32) as usize;
                let i3 = config.rand_range(0, parts.len() as u32) as usize;
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
        .unwrap_or_else(|| sample(&FALLBACK_COMPANY_TYPES).to_string())
}

/// Generate a company profession
pub fn profession() -> String {
    fetch_locale("company.profession", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(&FALLBACK_PROFESSIONS).to_string())
}

/// Generate a company department
pub fn department() -> String {
    fetch_locale("company.department", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(&FALLBACK_DEPARTMENTS).to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let n = name();
        assert!(!n.is_empty());
    }
}

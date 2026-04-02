//! Company generator - generates company names

use crate::base::sample;

/// Generate a random company name
pub fn name() -> String {
    let prefix = sample(&NAME_PREFIXES);
    let suffix = sample(&NAME_SUFFIXES);
    format!("{} {}", prefix, suffix)
}

/// Generate a random company suffix (Inc, LLC, etc.)
pub fn suffix() -> String {
    sample(&COMPANY_SUFFIXES).to_string()
}

/// Generate a random company industry
pub fn industry() -> String {
    sample(&INDUSTRIES).to_string()
}

// Data

const NAME_PREFIXES: &[&str] = &[
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
    "Beta",
    "Delta",
    "Omega",
    "Prime",
    "Summit",
    "Peak",
    "Pinnacle",
    "Apex",
    "Horizon",
    "Stellar",
    "Nova",
    "Quantum",
    "Vector",
    "Nexus",
    "Synergy",
    "Dynamic",
    "Precision",
    "Advanced",
    "Innovative",
    "Strategic",
    "Creative",
];

const NAME_SUFFIXES: &[&str] = &[
    "Solutions",
    "Systems",
    "Technologies",
    "Dynamics",
    "Industries",
    "Group",
    "Corporation",
    "Enterprises",
    "Partners",
    "Consulting",
    "Services",
    "Labs",
    "Works",
    "Hub",
    "Center",
];

const COMPANY_SUFFIXES: &[&str] = &[
    "Inc.",
    "LLC",
    "Corp.",
    "Co.",
    "Ltd.",
    "Group",
    "Holdings",
    "Enterprises",
    "Partners",
    "Solutions",
    "Technologies",
    "Systems",
];

const INDUSTRIES: &[&str] = &[
    "Technology",
    "Healthcare",
    "Finance",
    "Retail",
    "Manufacturing",
    "Education",
    "Consulting",
    "Media",
    "Transportation",
    "Energy",
    "Real Estate",
    "Telecommunications",
    "Hospitality",
    "Insurance",
    "Legal",
    "Marketing",
    "Non-Profit",
    "Construction",
];

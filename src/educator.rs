//! Educator generator - generates school and course names

use crate::base::sample;
use crate::locale::{fetch_locale, sample_with_resolve};

/// Generate a random school name
pub fn school_name() -> String {
    fetch_locale("educator.school_name", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bluemeadow".to_string())
}

/// Generate a random university name (with placeholder resolution)
pub fn university() -> String {
    fetch_locale("educator.university", "en")
        .map(|v| sample_with_resolve(&v, Some("educator")))
        .unwrap_or_else(|| "Bluemeadow University".to_string())
}

/// Generate a secondary school name
pub fn secondary_school() -> String {
    fetch_locale("educator.secondary_school", "en")
        .map(|v| sample_with_resolve(&v, Some("educator")))
        .unwrap_else(|| "Bluemeadow High".to_string())
}

/// Generate a primary school name
pub fn primary_school() -> String {
    fetch_locale("educator.primary_school", "en")
        .map(|v| sample_with_resolve(&v, Some("educator")))
        .unwrap_else(|| "Bluemeadow Elementary School".to_string())
}

/// Generate a campus name
pub fn campus() -> String {
    fetch_locale("educator.campus", "en")
        .map(|v| sample_with_resolve(&v, Some("educator")))
        .unwrap_else(|| "Bluemeadow Campus".to_string())
}

/// Generate a random subject
pub fn subject() -> String {
    fetch_locale("educator.subject", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Science".to_string())
}

/// Generate a degree
pub fn degree() -> String {
    fetch_locale("educator.degree", "en")
        .map(|v| sample_with_resolve(&v, Some("educator")))
        .unwrap_else(|| "Bachelor of Science".to_string())
}

/// Generate a course name
pub fn course_name() -> String {
    fetch_locale("educator.course_name", "en")
        .map(|v| sample_with_resolve(&v, Some("educator")))
        .unwrap_else(|| "Science 101".to_string())
}

/// Generate a university type (College, TAFE, etc.)
pub fn university_type() -> String {
    fetch_locale("educator.tertiary.university_type", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "University".to_string())
}

/// Generate a degree type (Associate Degree in, Bachelor of, etc.)
pub fn degree_type() -> String {
    fetch_locale("educator.tertiary.degree.type", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Bachelor of".to_string())
}

//! Job generator - generates random job titles and fields

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random job field
pub fn field() -> String {
    fetch_locale("job.field", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Technology".to_string())
}

/// Generate a random job seniority level
pub fn seniority() -> String {
    fetch_locale("job.seniority", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Senior".to_string())
}

/// Generate a random job position
pub fn position() -> String {
    fetch_locale("job.position", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Manager".to_string())
}

/// Generate a random job title
pub fn title() -> String {
    let config = crate::config::FakerConfig::current();
    let choice = config.rand_usize(3);

    match choice {
        0 => format!("{} {}", seniority(), position()),
        1 => format!("{} {}", position(), field()),
        _ => format!("{} {}", seniority(), field()),
    }
}

/// Generate a random key skill
pub fn key_skill() -> String {
    fetch_locale("job.key_skill", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Communication".to_string())
}

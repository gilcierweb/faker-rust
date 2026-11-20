//! Mountaineering generator.

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random mountaineer name.
pub fn mountaineer() -> String {
    fetch_locale("mountaineering.mountaineer", "en")
        .map(|v| sample(&v))
        .unwrap_or_default()
}

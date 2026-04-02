//! Commerce generator - generates random commerce-related data

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random department name
pub fn department() -> String {
    fetch_locale("commerce.department", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Electronics".to_string())
}

/// Generate a random product name
pub fn product_name() -> String {
    let config = crate::config::FakerConfig::current();
    let choice = config.rand_usize(3);

    let material = sample(&[
        "Small".to_string(),
        "Medium".to_string(),
        "Large".to_string(),
        "Extra Large".to_string(),
        "Slim".to_string(),
        "Compact".to_string(),
    ]);
    let product = fetch_locale("commerce.product_name.product", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Table".to_string());
    let dept = department();

    match choice {
        0 => format!("{} {}", material, product),
        1 => format!("{} {}", dept, product),
        _ => format!("{}, {} and {}", material, product, dept),
    }
}

/// Generate a random brand name
pub fn brand() -> String {
    fetch_locale("commerce.brand", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Nike".to_string())
}

/// Generate a random color
pub fn color() -> String {
    sample(&[
        "Red".to_string(),
        "Blue".to_string(),
        "Green".to_string(),
        "Yellow".to_string(),
        "Orange".to_string(),
        "Purple".to_string(),
        "Pink".to_string(),
        "Brown".to_string(),
        "Black".to_string(),
        "White".to_string(),
    ])
}

/// Generate a random material
pub fn material() -> String {
    fetch_locale("commerce.material", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Plastic".to_string())
}

/// Generate a random price
pub fn price(range_start: Option<f64>, range_end: Option<f64>) -> String {
    let config = crate::config::FakerConfig::current();
    let start = range_start.unwrap_or(0.0) as f64 * 100.0;
    let end = range_end.unwrap_or(100.0) as f64 * 100.0;
    let price = config.rand_f64() * (end - start) + start;
    format!("{:.2}", price / 100.0)
}

/// Generate a random promotion code
pub fn promotion_code() -> String {
    let config = crate::config::FakerConfig::current();
    let prefix: String = (0..2)
        .map(|_| config.rand_char(&crate::base::U_LETTERS))
        .collect();
    let suffix: String = (0..4)
        .map(|_| config.rand_char(&crate::base::DIGITS))
        .collect();
    format!("{}{}", prefix, suffix)
}

/// Generate a random vendor name
pub fn vendor() -> String {
    fetch_locale("commerce.vendor", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Acme Corp".to_string())
}

//! Localization and configuration example.
//!
//! Demonstrates the thread-local configuration API and the raw locale-data
//! helpers. Generators resolve their data through the configured locale and
//! fall back to English when a translation is unavailable.

use faker_rust::config::FakerConfig;
use faker_rust::locale;

fn show_values(label: &str, values: Option<Vec<String>>) {
    match values {
        Some(values) => println!(
            "  {label}: {} values, first = {:?}",
            values.len(),
            values.first()
        ),
        None => println!("  {label}: no data"),
    }
}

fn main() {
    println!("=== Localization and configuration ===\n");

    // The active locale lives in thread-local configuration.
    FakerConfig::set_locale("en");
    println!("active locale: {}", FakerConfig::locale());
    println!();

    println!("RAW LOCALE DATA");
    show_values(
        "en name.first_name",
        locale::fetch_locale("name.first_name", "en"),
    );
    show_values(
        "ja name.last_name",
        locale::fetch_locale("name.last_name", "ja"),
    );
    println!();

    println!("CONTEXT-SPECIFIC LOOKUPS");
    // Address placeholders are resolved relative to the `address` category.
    for key in ["name.female_first_name", "city_suffix", "city_prefix"] {
        show_values(
            &format!("address/{key}"),
            locale::fetch_locale_with_context(key, "en", Some("address")),
        );
    }
    println!();

    println!("NAMED VALUES AND FALLBACK");
    // Map-shaped locale keys are read with fetch_locale_named.
    let visa = locale::fetch_locale_named("stripe.valid_cards", "visa", "en");
    println!("  stripe.valid_cards.visa: {visa:?}");

    // An unknown locale falls back to English instead of returning no data.
    show_values(
        "xx name.first_name (fallback)",
        locale::fetch_locale("name.first_name", "xx"),
    );
    println!();

    println!("GENERATORS USE THE CONFIGURED LOCALE");
    FakerConfig::set_locale("ja");
    println!("  ja last name: {}", faker_rust::name::last_name());
    FakerConfig::set_locale("xx");
    println!(
        "  xx last name (fallback): {}",
        faker_rust::name::last_name()
    );

    FakerConfig::set_locale("en");
    println!("restored locale: {}", FakerConfig::locale());
}

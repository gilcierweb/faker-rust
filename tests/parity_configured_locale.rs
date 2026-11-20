//! Regression coverage for `Faker::Config.locale` behaviour.
//!
//! The gem resolves locale data through the configured locale, so changing
//! `FakerConfig::set_locale` must change the output of generators that have
//! translated data. Japanese family names are non-ASCII, English ones are not.

use faker_rust::config::FakerConfig;

fn sample_last_names() -> Vec<String> {
    (0..20).map(|_| faker_rust::name::last_name()).collect()
}

#[test]
fn configured_locale_changes_generator_output() {
    FakerConfig::set_locale("en");
    let en = sample_last_names();
    assert!(
        en.iter().all(|name| name.is_ascii()),
        "English locale must produce ASCII names, got {en:?}"
    );

    FakerConfig::set_locale("ja");
    let ja = sample_last_names();
    assert!(
        ja.iter().any(|name| !name.is_ascii()),
        "Japanese locale must produce non-ASCII names, got {ja:?}"
    );

    FakerConfig::set_locale("en");
}

#[test]
fn unknown_locale_falls_back_to_english() {
    FakerConfig::set_locale("xx");
    let values = sample_last_names();
    assert!(
        values.iter().all(|name| name.is_ascii()),
        "unknown locale must fall back to English, got {values:?}"
    );

    FakerConfig::set_locale("en");
}

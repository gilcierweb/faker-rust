//! Coverage for public generators in modules that previously had no tests.

#[test]
fn bible_generators_return_locale_values() {
    assert!(!faker_rust::religion::bible::character().is_empty());
    assert!(!faker_rust::religion::bible::location().is_empty());
    assert!(!faker_rust::religion::bible::quote().is_empty());
}

#[test]
fn shakespeare_generators_return_locale_values() {
    assert!(!faker_rust::quotes::shakespeare::hamlet_quote().is_empty());
    assert!(!faker_rust::quotes::shakespeare::as_you_like_it_quote().is_empty());
    assert!(!faker_rust::quotes::shakespeare::king_richard_iii_quote().is_empty());
    assert!(!faker_rust::quotes::shakespeare::romeo_and_juliet_quote().is_empty());
}

#[test]
fn chiquito_generators_return_locale_values() {
    assert!(!faker_rust::quotes::chiquito::expression().is_empty());
    assert!(!faker_rust::quotes::chiquito::term().is_empty());
    assert!(!faker_rust::quotes::chiquito::joke().is_empty());
    assert!(!faker_rust::quotes::chiquito::sentence().is_empty());
}

#[test]
fn rajnikanth_generator_returns_locale_value() {
    assert!(!faker_rust::quotes::rajnikanth::joke().is_empty());
}

#[test]
fn mountaineering_generator_returns_locale_value() {
    assert!(!faker_rust::sports::mountaineering::mountaineer().is_empty());
}

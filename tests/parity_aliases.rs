//! Parity tests for gem-documented method names that Rust exposes under a
//! normalized name. Each documented name must be callable at its module path.

#[test]
fn parity_aliases_address() {
    assert!(!faker_rust::address::postcode().is_empty());
    assert!(!faker_rust::address::zip().is_empty());
}

#[test]
fn parity_aliases_books_lovecraft() {
    assert!(!faker_rust::books::lovecraft::tome().is_empty());
}

#[test]
fn parity_aliases_color() {
    assert!(!faker_rust::color::hex_color().is_empty());
    assert!(!faker_rust::color::hsl_color().is_empty());
    assert!(!faker_rust::color::rgb_color().is_empty());
}

#[test]
fn parity_aliases_creature_bird() {
    assert!(!faker_rust::creature::bird::color().is_empty());
    assert!(!faker_rust::creature::bird::adjective().is_empty());
}

#[test]
fn parity_aliases_id_number() {
    assert!(!faker_rust::id_number::south_african_id_number().is_empty());
}

#[test]
fn parity_aliases_internet() {
    assert!(!faker_rust::internet::ip_v4_address().is_empty());
    assert!(!faker_rust::internet::ip_v6_address().is_empty());
    assert!(!faker_rust::internet::private_ip_v4_address().is_empty());
}

#[test]
fn parity_aliases_name() {
    assert!(!faker_rust::name::middle_name().is_empty());
}

#[test]
fn parity_aliases_airport() {
    assert!(!faker_rust::travel::airport::iata("large", "united_states").is_empty());
}

#[test]
fn parity_aliases_vehicle() {
    assert!(!faker_rust::vehicle::door_count().is_empty());
    assert!(!faker_rust::vehicle::engine_size().is_empty());
    let kilometrage = faker_rust::vehicle::kilometrage();
    assert!((10_000..90_000).contains(&kilometrage));
}

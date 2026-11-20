//! Crate-wide parity tests for gem-documented parameterized generators.

#[test]
fn parity_dune_saying() {
    assert!(!faker_rust::books::dune::saying(None).is_empty());
    assert!(!faker_rust::books::dune::saying(Some("fremen")).is_empty());
}

#[test]
fn parity_address_dynamic_lookups() {
    assert_eq!(faker_rust::address::country_by_code("US"), "United States");
    assert_eq!(
        faker_rust::address::country_name_to_code("united_states"),
        "US"
    );
}

#[test]
fn parity_driving_licence_usa() {
    assert!(!faker_rust::driving_license::usa_driving_licence("California").is_empty());
}

#[test]
fn parity_finance_dynamic_lookups() {
    assert!(!faker_rust::finance::credit_card(&["visa"]).is_empty());
    assert!(!faker_rust::finance::ticker(&["NYSE"]).is_empty());
    assert!(faker_rust::finance::vat_number("BR").starts_with("BR"));
}

#[test]
fn parity_relationship_familial() {
    assert!(!faker_rust::relationship::familial(None).is_empty());
    assert!(!faker_rust::relationship::familial(Some("direct")).is_empty());
}

#[test]
fn parity_science_generators() {
    assert!(!faker_rust::science::element_state().is_empty());
    assert!(!faker_rust::science::element_subcategory().is_empty());
    assert!(!faker_rust::science::modifier().is_empty());
    assert!(!faker_rust::science::tool(true).is_empty());
    assert!(!faker_rust::science::tool(false).is_empty());
}

#[test]
fn parity_source_generators() {
    assert!(faker_rust::source::hello_world("ruby").contains("Hello World"));
    assert!(faker_rust::source::print("test", "ruby").contains("test"));
    assert!(!faker_rust::source::print_1_to_10("ruby").is_empty());
}

#[test]
fn parity_vehicle_dynamic_lookups() {
    assert!(!faker_rust::vehicle::license_plate("").is_empty());
    assert!(!faker_rust::vehicle::license_plate("CA").is_empty());
    assert!(!faker_rust::vehicle::model("").is_empty());
    assert!(!faker_rust::vehicle::model("Toyota").is_empty());
}

#[test]
fn parity_world_cup_dynamic_lookups() {
    assert!(!faker_rust::world_cup::group("group_A").is_empty());
    assert!(!faker_rust::world_cup::roster("Egypt", "coach").is_empty());
}

#[test]
fn parity_kamen_rider_generators() {
    assert!(!faker_rust::japanese_media::kamen_rider::kamen_rider(&["showa"]).is_empty());
    assert!(!faker_rust::japanese_media::kamen_rider::user(&["showa"]).is_empty());
}

#[test]
fn parity_star_wars_quote() {
    assert!(!faker_rust::movies::star_wars::quote(None).is_empty());
    assert!(!faker_rust::movies::star_wars::quote(Some("luke_skywalker")).is_empty());
}

#[test]
fn parity_measurement_generators() {
    assert!(faker_rust::measurement::height(5).contains(' '));
    assert!(faker_rust::measurement::metric_length(5).contains(' '));
    assert!(faker_rust::measurement::metric_volume(5).contains(' '));
}

#[test]
fn parity_invoice_creditor_reference() {
    let r = faker_rust::invoice::creditor_reference("1234");
    assert!(r.starts_with("RF") && r.ends_with("1234"));
}

#[test]
fn parity_hipster_words() {
    let words = faker_rust::hipster::words(3, false, false);
    assert_eq!(words.len(), 3);
    assert!(words.iter().all(|w| !w.is_empty()));
}

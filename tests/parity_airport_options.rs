//! Regression coverage for `Faker::Travel::Airport.name(size:, region:)` and
//! `Faker::Travel::Airport.iata(size:, region:)`. Both keyword arguments are
//! required in the gem, so the Rust adaptation must accept them and select
//! from the matching locale bucket instead of returning an arbitrary value.

fn locale_bucket(key: &str) -> Vec<String> {
    faker_rust::locale::fetch_locale(key, "en").expect("locale bucket must be present")
}

#[test]
fn airport_name_respects_size_and_region() {
    let us_large = locale_bucket("airport.united_states.large");
    let eu_large = locale_bucket("airport.european_union.large");

    for _ in 0..50 {
        let name = faker_rust::travel::airport::name("large", "united_states");
        assert!(
            us_large.contains(&name),
            "unexpected US large airport: {name}"
        );

        let name = faker_rust::travel::airport::name("large", "european_union");
        assert!(
            eu_large.contains(&name),
            "unexpected EU large airport: {name}"
        );
    }
}

#[test]
fn airport_iata_respects_size_and_region() {
    let us_large = locale_bucket("airport.united_states.iata_code.large");
    let eu_large = locale_bucket("airport.european_union.iata_code.large");

    for _ in 0..50 {
        let code = faker_rust::travel::airport::iata("large", "united_states");
        assert!(
            us_large.contains(&code),
            "unexpected US large IATA code: {code}"
        );

        let code = faker_rust::travel::airport::iata("large", "european_union");
        assert!(
            eu_large.contains(&code),
            "unexpected EU large IATA code: {code}"
        );
    }
}

#[test]
fn airport_zero_argument_helpers_still_work() {
    assert!(!faker_rust::travel::airport::name_any().is_empty());
    let code = faker_rust::travel::airport::iata_code();
    assert_eq!(code.len(), 3);
    assert!(code.chars().all(|c| c.is_ascii_uppercase()));
}

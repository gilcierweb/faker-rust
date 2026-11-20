//! Regression coverage for gem source helpers and algorithmic generators that
//! were absent from `faker/doc` and therefore missed by the documented-API
//! parity audit. Rust adaptations use native types (`u8`, `Vec`, `HashMap`,
//! `bool`, slices) in place of Ruby arrays, hashes and regex objects.

#[test]
fn blockchain_helpers_return_values() {
    let random = faker_rust::blockchain::aeternity::rand_strings(12);
    assert_eq!(random.len(), 12);
    assert!(random.chars().all(|c| c.is_ascii_alphanumeric()));

    let main = faker_rust::blockchain::bitcoin::address_for("main");
    assert!(main.starts_with('1'), "bad main address: {main}");
    assert!(main.len() > 20);

    let testnet = faker_rust::blockchain::bitcoin::address_for("testnet");
    assert!(
        testnet.starts_with('m') || testnet.starts_with('n') || testnet.starts_with('2'),
        "bad testnet address: {testnet}"
    );

    let tz = faker_rust::blockchain::tezos::encode_tz("tz1", 20);
    assert!(tz.starts_with("tz1"), "bad tezos address: {tz}");
}

#[test]
fn address_hash_and_bank_bsb_return_values() {
    let hash = faker_rust::address::full_address_as_hash(&["city", "zip_code"]);
    assert_eq!(hash.len(), 2);
    assert!(hash.contains_key("city"));
    assert!(hash.contains_key("zip_code"));
    assert!(hash.values().all(|value| !value.is_empty()));

    let bsb = faker_rust::bank::bsb_number();
    assert_eq!(bsb.len(), 6);
    assert!(bsb.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn color_and_finance_helpers_return_values() {
    assert!(usize::from(faker_rust::color::single_rgb_color()) < 256);

    let keys = faker_rust::finance::vat_number_keys();
    assert!(keys.contains(&"BR".to_string()));
    assert!(keys.contains(&"GB".to_string()));

    let code = faker_rust::finance::condominium_fiscal_code("IT");
    assert_eq!(code.len(), 11);
    assert!(code.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn hacker_phrases_are_resolved() {
    let phrases = faker_rust::hacker::phrases();
    assert_eq!(phrases.len(), 8);
    assert!(phrases.iter().all(|phrase| !phrase.is_empty()));
    assert!(phrases.iter().all(|phrase| !phrase.contains("#{")));
}

#[test]
fn ssn_valid_has_gem_shape() {
    for _ in 0..100 {
        let value = faker_rust::id_number::ssn_valid();
        let parts: Vec<&str> = value.split('-').collect();
        assert_eq!(parts.len(), 3, "bad ssn: {value}");
        assert_eq!(parts[0].len(), 3);
        assert_eq!(parts[1].len(), 2);
        assert_eq!(parts[2].len(), 4);
        let area: u32 = parts[0].parse().unwrap();
        assert!((1..=665).contains(&area) || (667..=899).contains(&area));
    }
}

#[test]
fn internet_network_helpers_match_gem_ranges() {
    let private_patterns = faker_rust::internet::private_nets_regex();
    assert_eq!(private_patterns.len(), 8);
    let reserved_patterns = faker_rust::internet::reserved_nets_regex();
    assert_eq!(reserved_patterns.len(), 7);

    assert!(faker_rust::internet::private_net_checker("10.0.0.1"));
    assert!(faker_rust::internet::private_net_checker("192.168.1.1"));
    assert!(!faker_rust::internet::private_net_checker("8.8.8.8"));

    assert!(faker_rust::internet::reserved_net_checker("0.0.0.1"));
    assert!(faker_rust::internet::reserved_net_checker("10.0.0.1"));
    assert!(!faker_rust::internet::reserved_net_checker("8.8.8.8"));
}

#[test]
fn invoice_amount_between_stays_in_range() {
    for _ in 0..100 {
        let value = faker_rust::invoice::amount_between(10.0, 20.0);
        assert!((10.0..=20.0).contains(&value), "out of range: {value}");
        assert_eq!((value * 100.0).round() / 100.0, value);
    }
}

#[test]
fn omniauth_initialize_builds_identity() {
    let auth = faker_rust::omniauth::initialize(None, None);
    assert!(!auth.name.is_empty());
    assert!(!auth.email.is_empty());
    assert!(auth.email.contains('@'));
    assert!(!auth.first_name.is_empty());
    assert!(!auth.last_name.is_empty());

    let explicit = faker_rust::omniauth::initialize(Some("Ada Lovelace"), Some("ada@example.com"));
    assert_eq!(explicit.name, "Ada Lovelace");
    assert_eq!(explicit.email, "ada@example.com");
    assert_eq!(explicit.first_name, "Ada");
    assert_eq!(explicit.last_name, "Lovelace");
}

#[test]
fn music_constants_are_exposed() {
    assert_eq!(
        faker_rust::music::music::keys(),
        &["C", "D", "E", "F", "G", "A", "B"]
    );
    assert_eq!(faker_rust::music::music::key_variants(), &["b", "#", ""]);
    assert_eq!(faker_rust::music::music::key_types(), &["", "m"]);
    assert_eq!(faker_rust::music::music::chord_types().len(), 12);
}

#[test]
fn quote_and_driving_licence_helpers_return_values() {
    let fortune = faker_rust::quote::fortune_cookie();
    assert!(!fortune.is_empty());

    let error = faker_rust::driving_license::InvalidStatePassed::initialize("bad state");
    assert_eq!(error.message(), "bad state");
    assert!(!error.to_string().is_empty());
}

#[test]
fn kamen_rider_device_helpers_return_values() {
    assert!(!faker_rust::japanese_media::kamen_rider::collectible_device(&[]).is_empty());
    assert!(!faker_rust::japanese_media::kamen_rider::transformation_device(&[]).is_empty());
    assert!(!faker_rust::japanese_media::kamen_rider::collectible_device(&["reiwa"]).is_empty());
}

//! Crate-wide parity tests for gem-documented algorithmic generators.

#[test]
fn parity_crypto_digests_have_gem_lengths() {
    assert_eq!(faker_rust::crypto::md5().len(), 32);
    assert_eq!(faker_rust::crypto::sha1().len(), 40);
    assert_eq!(faker_rust::crypto::sha256().len(), 64);
    assert_eq!(faker_rust::crypto::sha512().len(), 128);
    assert!(
        faker_rust::crypto::md5()
            .chars()
            .all(|c| c.is_ascii_hexdigit())
    );
}

#[test]
fn parity_number_generators() {
    assert_eq!(faker_rust::number::binary(8).len(), 8);
    assert_eq!(faker_rust::number::decimal_part(6).len(), 6);
    assert!(faker_rust::number::digit() <= 9);
    assert_eq!(faker_rust::number::hexadecimal(6).len(), 6);
    assert_eq!(faker_rust::number::leading_zero_number(5).len(), 5);
    assert!(faker_rust::number::negative(-10.0, -1.0) < 0.0);
    assert!(faker_rust::number::non_zero_digit() >= 1);
    let _ = faker_rust::number::normal(0.0, 1.0);
    assert!(faker_rust::number::positive(1.0, 10.0) > 0.0);
    assert!(faker_rust::number::within(1.0..=5.0) >= 1.0);
}

#[test]
fn parity_device_generators() {
    assert!(faker_rust::device::build_number() >= 1);
    assert!(faker_rust::device::version() >= 1);
}

#[test]
fn parity_internet_generators() {
    assert!(!faker_rust::internet::domain_word().is_empty());
    let ip = faker_rust::internet::public_ip_v4_address();
    assert_eq!(ip.split('.').count(), 4);
}

#[test]
fn parity_stripe_generators() {
    assert_eq!(faker_rust::stripe::month().len(), 2);
    assert_eq!(faker_rust::stripe::year().len(), 4);
    assert_eq!(faker_rust::stripe::ccv("amex").len(), 4);
    assert_eq!(faker_rust::stripe::ccv("visa").len(), 3);
}

#[test]
fn parity_creature_dog_gender() {
    let gender = faker_rust::creature::dog::gender();
    assert!(gender == "Male" || gender == "Female");
}

#[test]
fn parity_bitcoin_testnet_address() {
    let address = faker_rust::blockchain::bitcoin::testnet_address();
    assert!(address.starts_with('m') || address.starts_with('n') || address.starts_with('2'));
}

#[test]
fn parity_beer_numeric_formats() {
    assert!(faker_rust::beer::alcohol().ends_with('%'));
    assert!(faker_rust::beer::blg().ends_with("°Blg"));
    assert!(faker_rust::beer::ibu().ends_with(" IBU"));
}

#[test]
fn parity_app_semantic_version() {
    let v = faker_rust::app::semantic_version();
    assert_eq!(v.split('.').count(), 3);
}

#[test]
fn parity_color_hsla_color() {
    let c = faker_rust::color::hsla_color();
    assert!(c.starts_with("hsla(") && c.ends_with(')'));
}

#[test]
fn parity_music_chord_and_key() {
    assert!(!faker_rust::music::music::chord().is_empty());
    assert!(!faker_rust::music::music::key().is_empty());
}

#[test]
fn parity_chess_generators() {
    assert!(!faker_rust::chess::federation().is_empty());
    let rating = faker_rust::chess::rating(2000, 2900);
    assert!((2000..=2900).contains(&rating));
}

#[test]
fn parity_sport_generator() {
    assert!(!faker_rust::sports::sport::sport().is_empty());
}

#[test]
fn parity_tron_generators() {
    assert!(!faker_rust::movies::tron::program().is_empty());
    assert!(!faker_rust::movies::tron::user().is_empty());
}

#[test]
fn parity_star_wars_wookiee_sentence() {
    assert!(!faker_rust::movies::star_wars::wookiee_sentence().is_empty());
}

#[test]
fn parity_cosmere_knight_radiant() {
    assert!(!faker_rust::cosmere::knight_radiant().is_empty());
}

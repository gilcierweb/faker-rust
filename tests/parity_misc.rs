//! Crate-wide parity tests for remaining data and formatting generators.

#[test]
fn parity_bird_orders() {
    let order = faker_rust::creature::bird::order();
    assert!(!order.is_empty());
    let (chosen_order, name) = faker_rust::creature::bird::order_with_common_name(None);
    assert!(!chosen_order.is_empty() && !name.is_empty());
    assert!(!faker_rust::creature::bird::plausible_common_name().is_empty());
    assert!(!faker_rust::creature::bird::implausible_common_name().is_empty());
}

#[test]
fn parity_crypto_coin_fields() {
    assert!(!faker_rust::crypto_coin::coin_name().is_empty());
    assert!(faker_rust::crypto_coin::url_logo().starts_with("http"));
    assert!(!faker_rust::crypto_coin::acronym().is_empty());
}

#[test]
fn parity_date_between_except() {
    let value = faker_rust::date::between_except("2020-01-01", "2020-12-31", "2020-06-15");
    assert!(value.starts_with("2020-"));
    assert_ne!(value, "2020-06-15");
}

#[test]
fn parity_driving_licence_formats() {
    let british =
        faker_rust::driving_license::british_driving_licence("SMITH", "JS", "M", "1990-01-01");
    assert_eq!(british.len(), 16);
    assert_eq!(
        faker_rust::driving_license::northern_irish_driving_licence().len(),
        8
    );
    assert!(!faker_rust::driving_license::uk_driving_licence().is_empty());
}

#[test]
fn parity_file_dir() {
    let dir = faker_rust::file::dir(3, Some("/tmp"), "/");
    assert!(dir.starts_with("/tmp/"));
    assert_eq!(dir.matches('/').count(), 4);
}

#[test]
fn parity_hipster_text_generators() {
    assert_eq!(faker_rust::hipster::sentences(3, false).len(), 3);
    assert_eq!(faker_rust::hipster::paragraphs(2, false).len(), 2);
    let p = faker_rust::hipster::paragraph_by_chars(120, false);
    assert!(p.ends_with('.') && p.len() >= 100);
}

#[test]
fn parity_lorem_text_generators() {
    let question = faker_rust::lorem::question(4, false, 0, None);
    assert!(question.ends_with('?'));
    assert_eq!(faker_rust::lorem::questions(2, false).len(), 2);
    let p = faker_rust::lorem::paragraph_by_chars(120, false);
    assert!(p.ends_with('.') && p.len() >= 100);
}

#[test]
fn parity_lorem_flickr_urls() {
    let terms = ["cats".to_string()];
    let gray = faker_rust::lorem_flickr::grayscale_image("300x300", &terms, false);
    assert!(gray.starts_with("https://loremflickr.com/g/300/300/"));
    let pixel = faker_rust::lorem_flickr::pixelated_image("300x300", &terms, false);
    assert!(pixel.starts_with("https://loremflickr.com/p/300/300/"));
    let color = faker_rust::lorem_flickr::colorized_image("300x300", "red", &terms, false);
    assert!(color.starts_with("https://loremflickr.com/red/300/300/"));
}

#[test]
fn parity_nhs_british_number() {
    let n = faker_rust::national_health_service::british_number();
    assert_eq!(n.len(), 12);
    assert_eq!(n.chars().nth(3), Some(' '));
    assert_eq!(n.chars().nth(7), Some(' '));
}

#[test]
fn parity_science_branch_selection() {
    assert!(!faker_rust::science::science(&[]).is_empty());
    assert!(!faker_rust::science::science(&["empirical_natural_basic"]).is_empty());
}

#[test]
fn parity_time_between_dates() {
    let v = faker_rust::time::between_dates(
        "2020-01-01T00:00:00Z",
        "2020-12-31T00:00:00Z",
        "all",
        None,
    );
    assert!(!v.is_empty());
}

#[test]
fn parity_vehicle_numeric_generators() {
    assert!(faker_rust::vehicle::mileage(0, 10) <= 10);
    assert_eq!(faker_rust::vehicle::vin().len(), 17);
    let y = faker_rust::vehicle::year();
    assert!((1900..=2100).contains(&y));
}

#[test]
fn parity_chile_rut_generators() {
    let dv = faker_rust::chile_rut::check_digit("12345678");
    assert_eq!(dv.len(), 1);
    assert_eq!(faker_rust::chile_rut::dv("12345678"), dv);
    let full = faker_rust::chile_rut::full_rut(1, 99_999_999, false, false);
    assert!(full.contains('-'));
}

#[test]
fn parity_barcode_ean() {
    assert_eq!(faker_rust::barcode::ean(8).len(), 8);
    assert_eq!(faker_rust::barcode::ean(13).len(), 13);
    assert!(faker_rust::barcode::ean_with_composite_symbology(8).len() >= 8);
}

#[test]
fn parity_code_identifiers() {
    assert_eq!(faker_rust::code::imei().len(), 15);
    assert_eq!(faker_rust::code::npi().len(), 10);
    assert_eq!(faker_rust::code::nric(18, 65).len(), 9);
    assert!(faker_rust::code::rut().contains('-'));
    assert_eq!(faker_rust::code::sin().len(), 9);
}

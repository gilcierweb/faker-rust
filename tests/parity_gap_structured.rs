//! Parity tests for gem-documented structured and stateful generators that the
//! initial audit classified as internal helpers.

#[test]
fn parity_gap_crypto_coin_coin_hash() {
    let value = faker_rust::crypto_coin::coin_hash();
    assert!(value.is_object());
    for key in ["name", "acronym", "url_logo"] {
        let field = value.get(key).and_then(|v| v.as_str()).unwrap_or_default();
        assert!(!field.is_empty(), "empty crypto coin field: {key}");
    }
    let logo = value["url_logo"].as_str().unwrap();
    assert!(logo.starts_with("http"), "unexpected logo URL: {logo}");
}

#[test]
fn parity_gap_chile_rut_last_rut() {
    faker_rust::chile_rut::rut();
    let last = faker_rust::chile_rut::last_rut();
    assert!(last.is_some(), "last_rut must expose the generated RUT");
    assert!(last.unwrap() > 0);
}

#[test]
fn parity_gap_kamen_rider_eras() {
    let eras = faker_rust::japanese_media::kamen_rider::eras();
    assert_eq!(eras, ["showa", "heisei", "reiwa"]);
}

#[test]
fn parity_gap_x_user() {
    let value = faker_rust::x::user();
    assert!(value.is_object());
    let data = value["data"].as_array().expect("data must be an array");
    assert_eq!(data.len(), 1);
    for key in ["author_id", "id", "text"] {
        assert!(data[0].get(key).is_some(), "missing data field: {key}");
    }
    let users = value["includes"]["users"]
        .as_array()
        .expect("includes.users must be an array");
    assert_eq!(users.len(), 1);
    let user = &users[0];
    for key in [
        "public_metrics",
        "username",
        "pinned_tweet_id",
        "entities",
        "description",
        "name",
        "verified",
        "location",
        "id",
        "protected",
        "url",
        "profile_image_url",
        "created_at",
    ] {
        assert!(user.get(key).is_some(), "missing user field: {key}");
    }
    for key in [
        "followers_count",
        "following_count",
        "tweet_count",
        "listed_count",
    ] {
        assert!(
            user["public_metrics"].get(key).is_some(),
            "missing public metric: {key}"
        );
    }
}

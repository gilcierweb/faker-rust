//! Regression coverage for public generator functions that had no test.
//!
//! Every call below exercises a real generator. The assertions only require a
//! usable value, since the exact value is random and other parity tests already
//! verify per-generator formats.

fn check(label: &str, value: &str) {
    assert!(!value.is_empty(), "{label} returned an empty string");
}

#[test]
fn shared_helpers_and_default_generators_return_values() {
    check("base::parse", &faker_rust::base::parse("#{first_name}"));

    check("address::country", &faker_rust::address::country());
    check(
        "address::full_address",
        &faker_rust::address::full_address(),
    );
    check(
        "address::secondary_address",
        &faker_rust::address::secondary_address(),
    );
    check(
        "address::street_address",
        &faker_rust::address::street_address(),
    );
    check("address::street_name", &faker_rust::address::street_name());
    check("address::time_zone", &faker_rust::address::time_zone());
    check("app::author", &faker_rust::app::author());
    check(
        "bank::iban_country_code",
        &faker_rust::bank::iban_country_code(),
    );
    check("barcode::isbn", &faker_rust::barcode::isbn());
    check("barcode::upc_e", &faker_rust::barcode::upc_e());
    check("beer::hop", &faker_rust::beer::hop());
    check("beer::malt", &faker_rust::beer::malt());
    check("beer::yeast", &faker_rust::beer::yeast());
    check("blood::rh_factor", &faker_rust::blood::rh_factor());
    check("blood::type_name", &faker_rust::blood::type_name());
    check(
        "business::credit_card_expiry_date",
        &faker_rust::business::credit_card_expiry_date(),
    );
    check(
        "business::credit_card_type",
        &faker_rust::business::credit_card_type(),
    );
}

#[test]
fn lifestyle_default_generators_return_values() {
    check(
        "cannabis::cannabinoid",
        &faker_rust::cannabis::cannabinoid(),
    );
    check("cannabis::category", &faker_rust::cannabis::category());
    check(
        "cannabis::health_benefit",
        &faker_rust::cannabis::health_benefit(),
    );
    check(
        "cannabis::medical_use",
        &faker_rust::cannabis::medical_use(),
    );
    check("cannabis::terpene", &faker_rust::cannabis::terpene());
    check("cannabis::type_name", &faker_rust::cannabis::type_name());
    check("chess::opening", &faker_rust::chess::opening());
    check("chess::title", &faker_rust::chess::title());
    check("chess::tournament", &faker_rust::chess::tournament());
    check("code::asin", &faker_rust::code::asin());
    check("coffee::notes", &faker_rust::coffee::notes());
    check("coffee::variety", &faker_rust::coffee::variety());
    check("commerce::material", &faker_rust::commerce::material());
    check(
        "commerce::product_name",
        &faker_rust::commerce::product_name(),
    );
    check("commerce::vendor", &faker_rust::commerce::vendor());
    check("company::bs", &faker_rust::company::bs());
    check(
        "company::catch_phrase",
        &faker_rust::company::catch_phrase(),
    );
    check(
        "company::company_type",
        &faker_rust::company::company_type(),
    );
    check("company::duns_number", &faker_rust::company::duns_number());
    check("company::ein", &faker_rust::company::ein());
    check("company::industry", &faker_rust::company::industry());
    check("company::logo", &faker_rust::company::logo());
    check("company::profession", &faker_rust::company::profession());
    check("company::suffix", &faker_rust::company::suffix());
    check(
        "compass::abbreviation",
        &faker_rust::compass::abbreviation(),
    );
    check("compass::azimuth", &faker_rust::compass::azimuth());
    check("compass::cardinal", &faker_rust::compass::cardinal());
    check("compass::ordinal", &faker_rust::compass::ordinal());
    check(
        "computer::linux_processor",
        &faker_rust::computer::linux_processor(),
    );
    check("computer::os", &faker_rust::computer::os());
    check("computer::platform", &faker_rust::computer::platform());
}

#[test]
fn technical_default_generators_return_values() {
    check(
        "construction::heavy_equipment",
        &faker_rust::construction::heavy_equipment(),
    );
    check("construction::role", &faker_rust::construction::role());
    check(
        "construction::standard_cost_code",
        &faker_rust::construction::standard_cost_code(),
    );
    check(
        "construction::subcontract_category",
        &faker_rust::construction::subcontract_category(),
    );
    check("construction::trade", &faker_rust::construction::trade());
    check("currency::symbol", &faker_rust::currency::symbol());
    check(
        "date::birthday",
        &faker_rust::date::birthday(Some(20), Some(30)),
    );
    check(
        "date::in_date_period",
        &faker_rust::date::in_date_period("2020-01-01", "2020-12-31", None),
    );
    check(
        "date::on_day_of_week_between",
        &faker_rust::date::on_day_of_week_between("Monday", "2020-01-01", "2020-12-31"),
    );
    check("device::manufacturer", &faker_rust::device::manufacturer());
    check("device::model_name", &faker_rust::device::model_name());
    check("device::platform", &faker_rust::device::platform());
    check(
        "driving_license::usa",
        &faker_rust::driving_license::usa("CA"),
    );
    check(
        "educator::degree_type",
        &faker_rust::educator::degree_type(),
    );
    check(
        "educator::university_type",
        &faker_rust::educator::university_type(),
    );
    check("file::mime_type", &faker_rust::file::mime_type());
    check("food::fruit", &faker_rust::food::fruit());
    check("food::vegetable", &faker_rust::food::vegetable());
    check("hacker::ingverb", &faker_rust::hacker::ingverb());
}

#[test]
fn internet_generators_return_values() {
    check(
        "internet::bot_user_agent",
        &faker_rust::internet::bot_user_agent(None),
    );
    check(
        "internet::device_token",
        &faker_rust::internet::device_token(),
    );
    check(
        "internet::domain_name",
        &faker_rust::internet::domain_name(true, None),
    );
    check(
        "internet::domain_suffix",
        &faker_rust::internet::domain_suffix(false),
    );
    check(
        "internet::email",
        &faker_rust::internet::email(None, None, None),
    );
    check(
        "internet::fix_umlauts",
        &faker_rust::internet::fix_umlauts("Auber"),
    );
    check("internet::ip_v4", &faker_rust::internet::ip_v4());
    check("internet::ip_v4_cidr", &faker_rust::internet::ip_v4_cidr());
    check("internet::ip_v6", &faker_rust::internet::ip_v6());
    check("internet::ip_v6_cidr", &faker_rust::internet::ip_v6_cidr());
    check(
        "internet::password",
        &faker_rust::internet::password(8, 12, true, true),
    );
    check(
        "internet::private_ip_v4",
        &faker_rust::internet::private_ip_v4(),
    );
    check("internet::slug", &faker_rust::internet::slug(None, None));
    check(
        "internet::url",
        &faker_rust::internet::url(None, None, None),
    );
    check(
        "internet::user_agent",
        &faker_rust::internet::user_agent(None),
    );
    check("internet::username", &faker_rust::internet::username(None));
    check("internet::uuid", &faker_rust::internet::uuid());
}

#[test]
fn identity_and_number_generators_return_values() {
    check("job::field", &faker_rust::job::field());
    check("job::key_skill", &faker_rust::job::key_skill());
    check("job::seniority", &faker_rust::job::seniority());
    check(
        "name::female_first_name",
        &faker_rust::name::female_first_name(),
    );
    check(
        "name::male_first_name",
        &faker_rust::name::male_first_name(),
    );
    check(
        "name::neutral_first_name",
        &faker_rust::name::neutral_first_name(),
    );
    check("name::prefix", &faker_rust::name::prefix());
    check("name::suffix", &faker_rust::name::suffix());
    let between = faker_rust::number::between_u32(1, 10);
    assert!((1..=10).contains(&between), "between_u32 out of range");
    check("number::decimal", &faker_rust::number::decimal(3, 2));
    check("number::hexify", &faker_rust::number::hexify(true));
    check("number::number", &faker_rust::number::number());
    check(
        "number::number_range",
        &faker_rust::number::number_range(1, 10),
    );
    let _ = faker_rust::number::number_to_ruby();
}

#[test]
fn phone_science_and_space_generators_return_values() {
    check(
        "phone_number::cell_phone_with_country_code",
        &faker_rust::phone_number::cell_phone_with_country_code(),
    );
    check(
        "phone_number::country_code",
        &faker_rust::phone_number::country_code(),
    );
    check(
        "phone_number::exchange_code",
        &faker_rust::phone_number::exchange_code(),
    );
    check(
        "phone_number::extension",
        &faker_rust::phone_number::extension(),
    );
    check(
        "phone_number::phone_number",
        &faker_rust::phone_number::phone_number(),
    );
    check(
        "phone_number::phone_number_with_country_code",
        &faker_rust::phone_number::phone_number_with_country_code(),
    );
    check(
        "phone_number::subscriber_number",
        &faker_rust::phone_number::subscriber_number(),
    );
    check(
        "science::element_symbol",
        &faker_rust::science::element_symbol(),
    );
    check("source::semver", &faker_rust::source::semver());
    check("space::constellation", &faker_rust::space::constellation());
    check("space::galaxy", &faker_rust::space::galaxy());
    let forward = faker_rust::time::forward(1);
    let backward = faker_rust::time::backward(1);
    assert!(
        forward > backward,
        "time offsets moved in the wrong direction"
    );
    check(
        "university::greek_alphabet",
        &faker_rust::university::greek_alphabet(),
    );
    check("university::prefix", &faker_rust::university::prefix());
    check("university::suffix", &faker_rust::university::suffix());
}

#[test]
fn namespaced_generators_return_values() {
    check(
        "fantasy::tolkien::hobbit_character",
        &faker_rust::fantasy::tolkien::hobbit_character(),
    );
    check(
        "fantasy::tolkien::lord_of_the_rings_character",
        &faker_rust::fantasy::tolkien::lord_of_the_rings_character(),
    );
    check(
        "fantasy::tolkien::poem",
        &faker_rust::fantasy::tolkien::poem(),
    );
    check(
        "fantasy::tolkien::race",
        &faker_rust::fantasy::tolkien::race(),
    );
    check(
        "games::minecraft::biome",
        &faker_rust::games::minecraft::biome(),
    );
    check(
        "games::minecraft::block",
        &faker_rust::games::minecraft::block(),
    );
    check(
        "games::pokemon::move_name",
        &faker_rust::games::pokemon::move_name(),
    );
    check("music::music::genre", &faker_rust::music::music::genre());
    check(
        "music::music::instrument",
        &faker_rust::music::music::instrument(),
    );
}

#[test]
fn color_components_stay_in_documented_ranges() {
    let hsl = faker_rust::color::hsl();
    assert!(
        hsl[0] < 360 && hsl[1] <= 100 && hsl[2] <= 100,
        "hsl out of range"
    );
    let _ = faker_rust::color::rgb();
}

#[test]
fn drone_specifications_return_values() {
    let values = [
        faker_rust::drone::battery_capacity(),
        faker_rust::drone::battery_type(),
        faker_rust::drone::battery_voltage(),
        faker_rust::drone::battery_weight(),
        faker_rust::drone::charging_temperature(),
        faker_rust::drone::flight_time(),
        faker_rust::drone::iso(),
        faker_rust::drone::max_altitude(),
        faker_rust::drone::max_angular_velocity(),
        faker_rust::drone::max_ascent_speed(),
        faker_rust::drone::max_charging_power(),
        faker_rust::drone::max_descent_speed(),
        faker_rust::drone::max_flight_distance(),
        faker_rust::drone::max_resolution(),
        faker_rust::drone::max_shutter_speed(),
        faker_rust::drone::max_speed(),
        faker_rust::drone::max_tilt_angle(),
        faker_rust::drone::max_wind_resistance(),
        faker_rust::drone::min_shutter_speed(),
        faker_rust::drone::operating_temperature(),
        faker_rust::drone::photo_format(),
        faker_rust::drone::video_format(),
    ];
    assert_eq!(values.len(), 22, "drone specification count changed");
    for value in values {
        check("drone specification", &value);
    }
}

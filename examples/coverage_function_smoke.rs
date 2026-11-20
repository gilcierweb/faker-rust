//! Runs every generator that previously lacked a runnable example.
//!
//! This is a smoke example: it proves that each documented generator is
//! reachable from the public API and produces a value at runtime.

fn main() {
    let samples: Vec<(&str, String)> = vec![
        ("address::full_address", faker_rust::address::full_address()),
        ("address::time_zone", faker_rust::address::time_zone()),
        (
            "bank::iban_country_code",
            faker_rust::bank::iban_country_code(),
        ),
        ("barcode::upc_e", faker_rust::barcode::upc_e()),
        ("beer::hop", faker_rust::beer::hop()),
        ("beer::malt", faker_rust::beer::malt()),
        ("beer::yeast", faker_rust::beer::yeast()),
        ("blood::rh_factor", faker_rust::blood::rh_factor()),
        ("chess::tournament", faker_rust::chess::tournament()),
        ("coffee::notes", faker_rust::coffee::notes()),
        ("commerce::vendor", faker_rust::commerce::vendor()),
        ("company::company_type", faker_rust::company::company_type()),
        ("company::duns_number", faker_rust::company::duns_number()),
        ("company::ein", faker_rust::company::ein()),
        ("company::logo", faker_rust::company::logo()),
        ("company::profession", faker_rust::company::profession()),
        ("compass::cardinal", faker_rust::compass::cardinal()),
        ("compass::ordinal", faker_rust::compass::ordinal()),
        (
            "computer::linux_processor",
            faker_rust::computer::linux_processor(),
        ),
        (
            "construction::heavy_equipment",
            faker_rust::construction::heavy_equipment(),
        ),
        ("construction::role", faker_rust::construction::role()),
        (
            "construction::standard_cost_code",
            faker_rust::construction::standard_cost_code(),
        ),
        (
            "construction::subcontract_category",
            faker_rust::construction::subcontract_category(),
        ),
        (
            "date::birthday",
            faker_rust::date::birthday(Some(20), Some(30)),
        ),
        (
            "date::in_date_period",
            faker_rust::date::in_date_period("2020-01-01", "2020-12-31", None),
        ),
        (
            "date::on_day_of_week_between",
            faker_rust::date::on_day_of_week_between("Monday", "2020-01-01", "2020-12-31"),
        ),
        (
            "driving_license::usa",
            faker_rust::driving_license::usa("CA"),
        ),
        ("educator::degree_type", faker_rust::educator::degree_type()),
        (
            "educator::university_type",
            faker_rust::educator::university_type(),
        ),
        (
            "fantasy::tolkien::hobbit_character",
            faker_rust::fantasy::tolkien::hobbit_character(),
        ),
        (
            "fantasy::tolkien::lord_of_the_rings_character",
            faker_rust::fantasy::tolkien::lord_of_the_rings_character(),
        ),
        (
            "fantasy::tolkien::poem",
            faker_rust::fantasy::tolkien::poem(),
        ),
        ("food::fruit", faker_rust::food::fruit()),
        ("food::vegetable", faker_rust::food::vegetable()),
        (
            "games::minecraft::biome",
            faker_rust::games::minecraft::biome(),
        ),
        (
            "internet::bot_user_agent",
            faker_rust::internet::bot_user_agent(None),
        ),
        (
            "internet::device_token",
            faker_rust::internet::device_token(),
        ),
        (
            "internet::domain_suffix",
            faker_rust::internet::domain_suffix(false),
        ),
        (
            "internet::fix_umlauts",
            faker_rust::internet::fix_umlauts("Auber"),
        ),
        ("internet::ip_v4_cidr", faker_rust::internet::ip_v4_cidr()),
        ("internet::ip_v6_cidr", faker_rust::internet::ip_v6_cidr()),
        (
            "internet::private_ip_v4",
            faker_rust::internet::private_ip_v4(),
        ),
        ("internet::slug", faker_rust::internet::slug(None, None)),
        ("internet::uuid", faker_rust::internet::uuid()),
        ("job::key_skill", faker_rust::job::key_skill()),
        ("job::seniority", faker_rust::job::seniority()),
        (
            "music::music::instrument",
            faker_rust::music::music::instrument(),
        ),
        (
            "name::female_first_name",
            faker_rust::name::female_first_name(),
        ),
        ("name::male_first_name", faker_rust::name::male_first_name()),
        (
            "name::neutral_first_name",
            faker_rust::name::neutral_first_name(),
        ),
        (
            "number::between_u32",
            faker_rust::number::between_u32(1, 10).to_string(),
        ),
        ("number::hexify", faker_rust::number::hexify(true)),
        ("number::number", faker_rust::number::number()),
        (
            "number::number_to_ruby",
            faker_rust::number::number_to_ruby().to_string(),
        ),
        (
            "phone_number::cell_phone_with_country_code",
            faker_rust::phone_number::cell_phone_with_country_code(),
        ),
        (
            "phone_number::exchange_code",
            faker_rust::phone_number::exchange_code(),
        ),
        (
            "phone_number::phone_number_with_country_code",
            faker_rust::phone_number::phone_number_with_country_code(),
        ),
        (
            "phone_number::subscriber_number",
            faker_rust::phone_number::subscriber_number(),
        ),
        ("source::semver", faker_rust::source::semver()),
        (
            "university::greek_alphabet",
            faker_rust::university::greek_alphabet(),
        ),
    ];

    assert_eq!(samples.len(), 60, "smoke example lost a generator call");
    for (name, value) in samples {
        assert!(!value.is_empty(), "{name} produced an empty value");
        println!("{name}: {value}");
    }
}

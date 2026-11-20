//! Crate-wide parity tests: gem-documented public generators resolve from locale data.

#[test]
fn parity_books_culture_series_culture_ship() {
    let value = faker_rust::books::culture_series::culture_ship();
    assert!(
        !value.is_empty(),
        "books/culture_series.culture_ship returned empty (locale key culture_series.culture_ships)"
    );
}

#[test]
fn parity_books_culture_series_culture_ship_class() {
    let value = faker_rust::books::culture_series::culture_ship_class();
    assert!(
        !value.is_empty(),
        "books/culture_series.culture_ship_class returned empty (locale key culture_series.culture_ship_classes)"
    );
}

#[test]
fn parity_books_culture_series_culture_ship_class_abv() {
    let value = faker_rust::books::culture_series::culture_ship_class_abv();
    assert!(
        !value.is_empty(),
        "books/culture_series.culture_ship_class_abv returned empty (locale key culture_series.culture_ship_class_abvs)"
    );
}

#[test]
fn parity_books_culture_series_planet() {
    let value = faker_rust::books::culture_series::planet();
    assert!(
        !value.is_empty(),
        "books/culture_series.planet returned empty (locale key culture_series.planets)"
    );
}

#[test]
fn parity_books_dune_city() {
    let value = faker_rust::books::dune::city();
    assert!(
        !value.is_empty(),
        "books/dune.city returned empty (locale key dune.cities)"
    );
}

#[test]
fn parity_books_lovecraft_deity() {
    let value = faker_rust::books::lovecraft::deity();
    assert!(
        !value.is_empty(),
        "books/lovecraft.deity returned empty (locale key lovecraft.deity)"
    );
}

#[test]
fn parity_books_lovecraft_word() {
    let value = faker_rust::books::lovecraft::word();
    assert!(
        !value.is_empty(),
        "books/lovecraft.word returned empty (locale key lovecraft.words)"
    );
}

#[test]
fn parity_books_the_kingkiller_chronicle_book() {
    let value = faker_rust::books::the_kingkiller_chronicle::book();
    assert!(
        !value.is_empty(),
        "books/the_kingkiller_chronicle.book returned empty (locale key books.the_kingkiller_chronicle.books)"
    );
}

#[test]
fn parity_books_the_kingkiller_chronicle_creature() {
    let value = faker_rust::books::the_kingkiller_chronicle::creature();
    assert!(
        !value.is_empty(),
        "books/the_kingkiller_chronicle.creature returned empty (locale key books.the_kingkiller_chronicle.creatures)"
    );
}

#[test]
fn parity_creature_bird_anatomy_past_tense() {
    let value = faker_rust::creature::bird::anatomy_past_tense();
    assert!(
        !value.is_empty(),
        "creature/bird.anatomy_past_tense returned empty (locale key creature.bird.anatomy_past_tense)"
    );
}

#[test]
fn parity_creature_bird_emotional_adjective() {
    let value = faker_rust::creature::bird::emotional_adjective();
    assert!(
        !value.is_empty(),
        "creature/bird.emotional_adjective returned empty (locale key creature.bird.emotional_adjectives)"
    );
}

#[test]
fn parity_creature_bird_geo() {
    let value = faker_rust::creature::bird::geo();
    assert!(
        !value.is_empty(),
        "creature/bird.geo returned empty (locale key creature.bird.geo)"
    );
}

#[test]
fn parity_creature_bird_silly_adjective() {
    let value = faker_rust::creature::bird::silly_adjective();
    assert!(
        !value.is_empty(),
        "creature/bird.silly_adjective returned empty (locale key creature.bird.silly_adjectives)"
    );
}

#[test]
fn parity_default_address_building_number() {
    let value = faker_rust::address::building_number();
    assert!(
        !value.is_empty(),
        "default/address.building_number returned empty (locale key address.building_number)"
    );
}

#[test]
fn parity_default_address_city_prefix() {
    let value = faker_rust::address::city_prefix();
    assert!(
        !value.is_empty(),
        "default/address.city_prefix returned empty (locale key address.city_prefix)"
    );
}

#[test]
fn parity_default_address_city_suffix() {
    let value = faker_rust::address::city_suffix();
    assert!(
        !value.is_empty(),
        "default/address.city_suffix returned empty (locale key address.city_suffix)"
    );
}

#[test]
fn parity_default_address_community() {
    let value = faker_rust::address::community();
    assert!(
        !value.is_empty(),
        "default/address.community returned empty (locale key address.community)"
    );
}

#[test]
fn parity_default_address_country_code_long() {
    let value = faker_rust::address::country_code_long();
    assert!(
        !value.is_empty(),
        "default/address.country_code_long returned empty (locale key address.country_code_long)"
    );
}

#[test]
fn parity_default_address_mail_box() {
    let value = faker_rust::address::mail_box();
    assert!(
        !value.is_empty(),
        "default/address.mail_box returned empty (locale key address.mail_box)"
    );
}

#[test]
fn parity_default_address_state() {
    let value = faker_rust::address::state();
    assert!(
        !value.is_empty(),
        "default/address.state returned empty (locale key address.state)"
    );
}

#[test]
fn parity_default_address_state_abbr() {
    let value = faker_rust::address::state_abbr();
    assert!(
        !value.is_empty(),
        "default/address.state_abbr returned empty (locale key address.state_abbr)"
    );
}

#[test]
fn parity_default_address_street_suffix() {
    let value = faker_rust::address::street_suffix();
    assert!(
        !value.is_empty(),
        "default/address.street_suffix returned empty (locale key address.street_suffix)"
    );
}

#[test]
fn parity_default_beer_malts() {
    let value = faker_rust::beer::malts();
    assert!(
        !value.is_empty(),
        "default/beer.malts returned empty (locale key beer.malt)"
    );
}

#[test]
fn parity_default_blood_type() {
    let value = faker_rust::blood::r#type();
    assert!(
        !value.is_empty(),
        "default/blood.type returned empty (locale key blood.type)"
    );
}

#[test]
fn parity_default_cannabis_buzzword() {
    let value = faker_rust::cannabis::buzzword();
    assert!(
        !value.is_empty(),
        "default/cannabis.buzzword returned empty (locale key cannabis.buzzwords)"
    );
}

#[test]
fn parity_default_cannabis_cannabinoid_abbreviation() {
    let value = faker_rust::cannabis::cannabinoid_abbreviation();
    assert!(
        !value.is_empty(),
        "default/cannabis.cannabinoid_abbreviation returned empty (locale key cannabis.cannabinoid_abbreviations)"
    );
}

#[test]
fn parity_default_cannabis_type() {
    let value = faker_rust::cannabis::r#type();
    assert!(
        !value.is_empty(),
        "default/cannabis.type returned empty (locale key cannabis.types)"
    );
}

#[test]
fn parity_default_coffee_intensifier() {
    let value = faker_rust::coffee::intensifier();
    assert!(
        !value.is_empty(),
        "default/coffee.intensifier returned empty (locale key coffee.intensifier)"
    );
}

#[test]
fn parity_default_coffee_origin() {
    let value = faker_rust::coffee::origin();
    assert!(
        !value.is_empty(),
        "default/coffee.origin returned empty (locale key coffee.country)"
    );
}

#[test]
fn parity_default_color_color_name() {
    let value = faker_rust::color::color_name();
    assert!(
        !value.is_empty(),
        "default/color.color_name returned empty (locale key color.name)"
    );
}

#[test]
fn parity_default_company_sic_code() {
    let value = faker_rust::company::sic_code();
    assert!(
        !value.is_empty(),
        "default/company.sic_code returned empty (locale key company.sic_code)"
    );
}

#[test]
fn parity_default_company_type() {
    let value = faker_rust::company::r#type();
    assert!(
        !value.is_empty(),
        "default/company.type returned empty (locale key company.type)"
    );
}

#[test]
fn parity_default_compass_cardinal_abbreviation() {
    let value = faker_rust::compass::cardinal_abbreviation();
    assert!(
        !value.is_empty(),
        "default/compass.cardinal_abbreviation returned empty (locale key compass.cardinal.abbreviation)"
    );
}

#[test]
fn parity_default_compass_cardinal_azimuth() {
    let value = faker_rust::compass::cardinal_azimuth();
    assert!(
        !value.is_empty(),
        "default/compass.cardinal_azimuth returned empty (locale key compass.cardinal.azimuth)"
    );
}

#[test]
fn parity_default_compass_half_wind() {
    let value = faker_rust::compass::half_wind();
    assert!(
        !value.is_empty(),
        "default/compass.half_wind returned empty (locale key compass.half-wind.word)"
    );
}

#[test]
fn parity_default_compass_half_wind_abbreviation() {
    let value = faker_rust::compass::half_wind_abbreviation();
    assert!(
        !value.is_empty(),
        "default/compass.half_wind_abbreviation returned empty (locale key compass.half-wind.abbreviation)"
    );
}

#[test]
fn parity_default_compass_half_wind_azimuth() {
    let value = faker_rust::compass::half_wind_azimuth();
    assert!(
        !value.is_empty(),
        "default/compass.half_wind_azimuth returned empty (locale key compass.half-wind.azimuth)"
    );
}

#[test]
fn parity_default_compass_ordinal_abbreviation() {
    let value = faker_rust::compass::ordinal_abbreviation();
    assert!(
        !value.is_empty(),
        "default/compass.ordinal_abbreviation returned empty (locale key compass.ordinal.abbreviation)"
    );
}

#[test]
fn parity_default_compass_ordinal_azimuth() {
    let value = faker_rust::compass::ordinal_azimuth();
    assert!(
        !value.is_empty(),
        "default/compass.ordinal_azimuth returned empty (locale key compass.ordinal.azimuth)"
    );
}

#[test]
fn parity_default_compass_quarter_wind() {
    let value = faker_rust::compass::quarter_wind();
    assert!(
        !value.is_empty(),
        "default/compass.quarter_wind returned empty (locale key compass.quarter-wind.word)"
    );
}

#[test]
fn parity_default_compass_quarter_wind_abbreviation() {
    let value = faker_rust::compass::quarter_wind_abbreviation();
    assert!(
        !value.is_empty(),
        "default/compass.quarter_wind_abbreviation returned empty (locale key compass.quarter-wind.abbreviation)"
    );
}

#[test]
fn parity_default_compass_quarter_wind_azimuth() {
    let value = faker_rust::compass::quarter_wind_azimuth();
    assert!(
        !value.is_empty(),
        "default/compass.quarter_wind_azimuth returned empty (locale key compass.quarter-wind.azimuth)"
    );
}

#[test]
fn parity_default_computer_stack() {
    let value = faker_rust::computer::stack();
    assert!(
        !value.is_empty(),
        "default/computer.stack returned empty (locale key computer.platform)"
    );
}

#[test]
fn parity_default_computer_type() {
    let value = faker_rust::computer::r#type();
    assert!(
        !value.is_empty(),
        "default/computer.type returned empty (locale key computer.type)"
    );
}

#[test]
fn parity_default_cosmere_allomancer() {
    let value = faker_rust::cosmere::allomancer();
    assert!(
        !value.is_empty(),
        "default/cosmere.allomancer returned empty (locale key cosmere.allomancers)"
    );
}

#[test]
fn parity_default_cosmere_aon() {
    let value = faker_rust::cosmere::aon();
    assert!(
        !value.is_empty(),
        "default/cosmere.aon returned empty (locale key cosmere.aons)"
    );
}

#[test]
fn parity_default_cosmere_feruchemist() {
    let value = faker_rust::cosmere::feruchemist();
    assert!(
        !value.is_empty(),
        "default/cosmere.feruchemist returned empty (locale key cosmere.feruchemists)"
    );
}

#[test]
fn parity_default_cosmere_herald() {
    let value = faker_rust::cosmere::herald();
    assert!(
        !value.is_empty(),
        "default/cosmere.herald returned empty (locale key cosmere.heralds)"
    );
}

#[test]
fn parity_default_cosmere_metal() {
    let value = faker_rust::cosmere::metal();
    assert!(
        !value.is_empty(),
        "default/cosmere.metal returned empty (locale key cosmere.metals)"
    );
}

#[test]
fn parity_default_cosmere_shard() {
    let value = faker_rust::cosmere::shard();
    assert!(
        !value.is_empty(),
        "default/cosmere.shard returned empty (locale key cosmere.shards)"
    );
}

#[test]
fn parity_default_cosmere_shard_world() {
    let value = faker_rust::cosmere::shard_world();
    assert!(
        !value.is_empty(),
        "default/cosmere.shard_world returned empty (locale key cosmere.shard_worlds)"
    );
}

#[test]
fn parity_default_cosmere_spren() {
    let value = faker_rust::cosmere::spren();
    assert!(
        !value.is_empty(),
        "default/cosmere.spren returned empty (locale key cosmere.sprens)"
    );
}

#[test]
fn parity_default_cosmere_surge() {
    let value = faker_rust::cosmere::surge();
    assert!(
        !value.is_empty(),
        "default/cosmere.surge returned empty (locale key cosmere.surges)"
    );
}

#[test]
fn parity_default_crypto_coin_coin_array() {
    let value = faker_rust::crypto_coin::coin_array();
    assert!(
        !value.is_empty(),
        "default/crypto_coin.coin_array returned empty (locale key crypto_coin.coin)"
    );
}

#[test]
fn parity_default_dc_comics_heroine() {
    let value = faker_rust::dc_comics::heroine();
    assert!(
        !value.is_empty(),
        "default/dc_comics.heroine returned empty (locale key dc_comics.heroine)"
    );
}

#[test]
fn parity_default_dc_comics_name() {
    let value = faker_rust::dc_comics::name();
    assert!(
        !value.is_empty(),
        "default/dc_comics.name returned empty (locale key dc_comics.name)"
    );
}

#[test]
fn parity_default_demographic_educational_attainment() {
    let value = faker_rust::demographic::educational_attainment();
    assert!(
        !value.is_empty(),
        "default/demographic.educational_attainment returned empty (locale key demographic.educational_attainment)"
    );
}

#[test]
fn parity_default_electrical_components_electromechanical() {
    let value = faker_rust::electrical_components::electromechanical();
    assert!(
        !value.is_empty(),
        "default/electrical_components.electromechanical returned empty (locale key electrical_components.electromechanical)"
    );
}

#[test]
fn parity_default_finance_stock_market() {
    let value = faker_rust::finance::stock_market();
    assert!(
        !value.is_empty(),
        "default/finance.stock_market returned empty (locale key finance.stock_market)"
    );
}

#[test]
fn parity_default_food_allergen() {
    let value = faker_rust::food::allergen();
    assert!(
        !value.is_empty(),
        "default/food.allergen returned empty (locale key food.allergens)"
    );
}

#[test]
fn parity_default_food_ethnic_category() {
    let value = faker_rust::food::ethnic_category();
    assert!(
        !value.is_empty(),
        "default/food.ethnic_category returned empty (locale key food.ethnic_category)"
    );
}

#[test]
fn parity_default_food_fruits() {
    let value = faker_rust::food::fruits();
    assert!(
        !value.is_empty(),
        "default/food.fruits returned empty (locale key food.fruits)"
    );
}

#[test]
fn parity_default_food_measurement() {
    let value = faker_rust::food::measurement();
    assert!(
        !value.is_empty(),
        "default/food.measurement returned empty (locale key food.measurement_sizes)"
    );
}

#[test]
fn parity_default_food_metric_measurement() {
    let value = faker_rust::food::metric_measurement();
    assert!(
        !value.is_empty(),
        "default/food.metric_measurement returned empty (locale key food.metric_measurements)"
    );
}

#[test]
fn parity_default_food_spice() {
    let value = faker_rust::food::spice();
    assert!(
        !value.is_empty(),
        "default/food.spice returned empty (locale key food.spices)"
    );
}

#[test]
fn parity_default_food_vegetables() {
    let value = faker_rust::food::vegetables();
    assert!(
        !value.is_empty(),
        "default/food.vegetables returned empty (locale key food.vegetables)"
    );
}

#[test]
fn parity_default_funny_name_four_word_name() {
    let value = faker_rust::funny_name::four_word_name();
    assert!(
        !value.is_empty(),
        "default/funny_name.four_word_name returned empty (locale key funny_name.name)"
    );
}

#[test]
fn parity_default_funny_name_name_with_initial() {
    let value = faker_rust::funny_name::name_with_initial();
    assert!(
        !value.is_empty(),
        "default/funny_name.name_with_initial returned empty (locale key funny_name.name)"
    );
}

#[test]
fn parity_default_funny_name_three_word_name() {
    let value = faker_rust::funny_name::three_word_name();
    assert!(
        !value.is_empty(),
        "default/funny_name.three_word_name returned empty (locale key funny_name.name)"
    );
}

#[test]
fn parity_default_funny_name_two_word_name() {
    let value = faker_rust::funny_name::two_word_name();
    assert!(
        !value.is_empty(),
        "default/funny_name.two_word_name returned empty (locale key funny_name.name)"
    );
}

#[test]
fn parity_default_gender_type() {
    let value = faker_rust::gender::r#type();
    assert!(
        !value.is_empty(),
        "default/gender.type returned empty (locale key gender.types)"
    );
}

#[test]
fn parity_default_hacker_say_something_smart() {
    let value = faker_rust::hacker::say_something_smart();
    assert!(
        !value.is_empty(),
        "default/hacker.say_something_smart returned empty (locale key hacker.abbreviation)"
    );
}

#[test]
fn parity_default_industry_segments_sub_sector() {
    let value = faker_rust::industry_segments::sub_sector();
    assert!(
        !value.is_empty(),
        "default/industry_segments.sub_sector returned empty (locale key industry_segments.sub_sector)"
    );
}

#[test]
fn parity_default_industry_segments_super_sector() {
    let value = faker_rust::industry_segments::super_sector();
    assert!(
        !value.is_empty(),
        "default/industry_segments.super_sector returned empty (locale key industry_segments.super_sector)"
    );
}

#[test]
fn parity_default_kpop_boy_bands() {
    let value = faker_rust::kpop::boy_bands();
    assert!(
        !value.is_empty(),
        "default/kpop.boy_bands returned empty (locale key kpop.boy_bands)"
    );
}

#[test]
fn parity_default_kpop_girl_groups() {
    let value = faker_rust::kpop::girl_groups();
    assert!(
        !value.is_empty(),
        "default/kpop.girl_groups returned empty (locale key kpop.girl_groups)"
    );
}

#[test]
fn parity_default_kpop_i_groups() {
    let value = faker_rust::kpop::i_groups();
    assert!(
        !value.is_empty(),
        "default/kpop.i_groups returned empty (locale key kpop.i_groups)"
    );
}

#[test]
fn parity_default_kpop_ii_groups() {
    let value = faker_rust::kpop::ii_groups();
    assert!(
        !value.is_empty(),
        "default/kpop.ii_groups returned empty (locale key kpop.ii_groups)"
    );
}

#[test]
fn parity_default_kpop_iii_groups() {
    let value = faker_rust::kpop::iii_groups();
    assert!(
        !value.is_empty(),
        "default/kpop.iii_groups returned empty (locale key kpop.iii_groups)"
    );
}

#[test]
fn parity_default_lorem_multibyte() {
    let value = faker_rust::lorem::multibyte();
    assert!(
        !value.is_empty(),
        "default/lorem.multibyte returned empty (locale key lorem.multibyte)"
    );
}

#[test]
fn parity_default_marketing_buzzwords() {
    let value = faker_rust::marketing::buzzwords();
    assert!(
        !value.is_empty(),
        "default/marketing.buzzwords returned empty (locale key marketing.buzzwords)"
    );
}

#[test]
fn parity_default_nation_national_sport() {
    let value = faker_rust::nation::national_sport();
    assert!(
        !value.is_empty(),
        "default/nation.national_sport returned empty (locale key team.sport)"
    );
}

#[test]
fn parity_default_restaurant_description() {
    let value = faker_rust::restaurant::description();
    assert!(
        !value.is_empty(),
        "default/restaurant.description returned empty (locale key restaurant.description)"
    );
}

#[test]
fn parity_default_restaurant_type() {
    let value = faker_rust::restaurant::r#type();
    assert!(
        !value.is_empty(),
        "default/restaurant.type returned empty (locale key restaurant.type)"
    );
}

#[test]
fn parity_default_slack_emoji_activity() {
    let value = faker_rust::slack_emoji::activity();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.activity returned empty (locale key slack_emoji.activity)"
    );
}

#[test]
fn parity_default_slack_emoji_celebration() {
    let value = faker_rust::slack_emoji::celebration();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.celebration returned empty (locale key slack_emoji.celebration)"
    );
}

#[test]
fn parity_default_slack_emoji_custom() {
    let value = faker_rust::slack_emoji::custom();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.custom returned empty (locale key slack_emoji.custom)"
    );
}

#[test]
fn parity_default_slack_emoji_food_and_drink() {
    let value = faker_rust::slack_emoji::food_and_drink();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.food_and_drink returned empty (locale key slack_emoji.food_and_drink)"
    );
}

#[test]
fn parity_default_slack_emoji_nature() {
    let value = faker_rust::slack_emoji::nature();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.nature returned empty (locale key slack_emoji.nature)"
    );
}

#[test]
fn parity_default_slack_emoji_objects_and_symbols() {
    let value = faker_rust::slack_emoji::objects_and_symbols();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.objects_and_symbols returned empty (locale key slack_emoji.objects_and_symbols)"
    );
}

#[test]
fn parity_default_slack_emoji_travel_and_places() {
    let value = faker_rust::slack_emoji::travel_and_places();
    assert!(
        !value.is_empty(),
        "default/slack_emoji.travel_and_places returned empty (locale key slack_emoji.travel_and_places)"
    );
}

#[test]
fn parity_default_space_agency() {
    let value = faker_rust::space::agency();
    assert!(
        !value.is_empty(),
        "default/space.agency returned empty (locale key space.agency)"
    );
}

#[test]
fn parity_default_space_agency_abv() {
    let value = faker_rust::space::agency_abv();
    assert!(
        !value.is_empty(),
        "default/space.agency_abv returned empty (locale key space.agency_abv)"
    );
}

#[test]
fn parity_default_space_company() {
    let value = faker_rust::space::company();
    assert!(
        !value.is_empty(),
        "default/space.company returned empty (locale key space.company)"
    );
}

#[test]
fn parity_default_space_distance_measurement() {
    let value = faker_rust::space::distance_measurement();
    assert!(
        !value.is_empty(),
        "default/space.distance_measurement returned empty (locale key space.distance_measurement)"
    );
}

#[test]
fn parity_default_space_launch_vehicle() {
    let value = faker_rust::space::launch_vehicle();
    assert!(
        !value.is_empty(),
        "default/space.launch_vehicle returned empty (locale key space.launch_vehicle)"
    );
}

#[test]
fn parity_default_space_meteorite() {
    let value = faker_rust::space::meteorite();
    assert!(
        !value.is_empty(),
        "default/space.meteorite returned empty (locale key space.meteorite)"
    );
}

#[test]
fn parity_default_space_nasa_space_craft() {
    let value = faker_rust::space::nasa_space_craft();
    assert!(
        !value.is_empty(),
        "default/space.nasa_space_craft returned empty (locale key space.nasa_space_craft)"
    );
}

#[test]
fn parity_default_space_nebula() {
    let value = faker_rust::space::nebula();
    assert!(
        !value.is_empty(),
        "default/space.nebula returned empty (locale key space.nebula)"
    );
}

#[test]
fn parity_default_space_star() {
    let value = faker_rust::space::star();
    assert!(
        !value.is_empty(),
        "default/space.star returned empty (locale key space.star)"
    );
}

#[test]
fn parity_default_space_star_cluster() {
    let value = faker_rust::space::star_cluster();
    assert!(
        !value.is_empty(),
        "default/space.star_cluster returned empty (locale key space.star_cluster)"
    );
}

#[test]
fn parity_default_subscription_payment_term() {
    let value = faker_rust::subscription::payment_term();
    assert!(
        !value.is_empty(),
        "default/subscription.payment_term returned empty (locale key subscription.payment_terms)"
    );
}

#[test]
fn parity_default_subscription_subscription_term() {
    let value = faker_rust::subscription::subscription_term();
    assert!(
        !value.is_empty(),
        "default/subscription.subscription_term returned empty (locale key subscription.subscription_terms)"
    );
}

#[test]
fn parity_default_tea_type() {
    let value = faker_rust::tea::r#type();
    assert!(
        !value.is_empty(),
        "default/tea.type returned empty (locale key tea.type)"
    );
}

#[test]
fn parity_default_team_creature() {
    let value = faker_rust::team::creature();
    assert!(
        !value.is_empty(),
        "default/team.creature returned empty (locale key team.creature)"
    );
}

#[test]
fn parity_default_theater_adult_musical() {
    let value = faker_rust::theater::adult_musical();
    assert!(
        !value.is_empty(),
        "default/theater.adult_musical returned empty (locale key theater.adult_musical)"
    );
}

#[test]
fn parity_default_theater_kids_musical() {
    let value = faker_rust::theater::kids_musical();
    assert!(
        !value.is_empty(),
        "default/theater.kids_musical returned empty (locale key theater.kids_musical)"
    );
}

#[test]
fn parity_default_vehicle_car_options() {
    let value = faker_rust::vehicle::car_options();
    assert!(
        !value.is_empty(),
        "default/vehicle.car_options returned empty (locale key vehicle.car_options)"
    );
}

#[test]
fn parity_default_vehicle_doors() {
    let value = faker_rust::vehicle::doors();
    assert!(
        !value.is_empty(),
        "default/vehicle.doors returned empty (locale key vehicle.doors)"
    );
}

#[test]
fn parity_default_vehicle_engine() {
    let value = faker_rust::vehicle::engine();
    assert!(
        !value.is_empty(),
        "default/vehicle.engine returned empty (locale key vehicle.cylinder_engine)"
    );
}

#[test]
fn parity_default_vehicle_make_and_model() {
    let value = faker_rust::vehicle::make_and_model();
    assert!(
        !value.is_empty(),
        "default/vehicle.make_and_model returned empty (locale key vehicle.makes)"
    );
}

#[test]
fn parity_default_vehicle_singapore_license_plate() {
    let value = faker_rust::vehicle::singapore_license_plate();
    assert!(
        !value.is_empty(),
        "default/vehicle.singapore_license_plate returned empty (locale key vehicle.license_plate)"
    );
}

#[test]
fn parity_default_vehicle_standard_specs() {
    let value = faker_rust::vehicle::standard_specs();
    assert!(
        !value.is_empty(),
        "default/vehicle.standard_specs returned empty (locale key vehicle.standard_specs)"
    );
}

#[test]
fn parity_default_vehicle_version() {
    let value = faker_rust::vehicle::version();
    assert!(
        !value.is_empty(),
        "default/vehicle.version returned empty (locale key vehicle.version)"
    );
}

#[test]
fn parity_default_world_cup_city() {
    let value = faker_rust::world_cup::city();
    assert!(
        !value.is_empty(),
        "default/world_cup.city returned empty (locale key world_cup.cities)"
    );
}

#[test]
fn parity_games_clash_of_clans_defensive_building() {
    let value = faker_rust::games::clash_of_clans::defensive_building();
    assert!(
        !value.is_empty(),
        "games/clash_of_clans.defensive_building returned empty (locale key games.clash_of_clans.defensive_buildings)"
    );
}

#[test]
fn parity_games_clash_of_clans_rank() {
    let value = faker_rust::games::clash_of_clans::rank();
    assert!(
        !value.is_empty(),
        "games/clash_of_clans.rank returned empty (locale key games.clash_of_clans.ranks)"
    );
}

#[test]
fn parity_games_dnd_first_name() {
    let value = faker_rust::games::dnd::first_name();
    assert!(
        !value.is_empty(),
        "games/dnd.first_name returned empty (locale key dnd.name.first_name)"
    );
}

#[test]
fn parity_games_dnd_last_name() {
    let value = faker_rust::games::dnd::last_name();
    assert!(
        !value.is_empty(),
        "games/dnd.last_name returned empty (locale key dnd.name.last_name)"
    );
}

#[test]
fn parity_games_dnd_name() {
    let value = faker_rust::games::dnd::name();
    assert!(
        !value.is_empty(),
        "games/dnd.name returned empty (locale key dnd.name.first_name)"
    );
}

#[test]
fn parity_games_dnd_title_name() {
    let value = faker_rust::games::dnd::title_name();
    assert!(
        !value.is_empty(),
        "games/dnd.title_name returned empty (locale key dnd.name.first_name)"
    );
}

#[test]
fn parity_games_final_fantasy_xiv_data_center() {
    let value = faker_rust::games::final_fantasy_xiv::data_center();
    assert!(
        !value.is_empty(),
        "games/final_fantasy_xiv.data_center returned empty (locale key games.final_fantasy_xiv.data_centers)"
    );
}

#[test]
fn parity_games_final_fantasy_xiv_zone() {
    let value = faker_rust::games::final_fantasy_xiv::zone();
    assert!(
        !value.is_empty(),
        "games/final_fantasy_xiv.zone returned empty (locale key games.final_fantasy_xiv.zones)"
    );
}

#[test]
fn parity_games_league_of_legends_masteries() {
    let value = faker_rust::games::league_of_legends::masteries();
    assert!(
        !value.is_empty(),
        "games/league_of_legends.masteries returned empty (locale key games.league_of_legends.masteries)"
    );
}

#[test]
fn parity_games_minecraft_achievement() {
    let value = faker_rust::games::minecraft::achievement();
    assert!(
        !value.is_empty(),
        "games/minecraft.achievement returned empty (locale key games.minecraft.achievement)"
    );
}

#[test]
fn parity_games_minecraft_enchantment() {
    let value = faker_rust::games::minecraft::enchantment();
    assert!(
        !value.is_empty(),
        "games/minecraft.enchantment returned empty (locale key games.minecraft.enchantment)"
    );
}

#[test]
fn parity_games_minecraft_game_mode() {
    let value = faker_rust::games::minecraft::game_mode();
    assert!(
        !value.is_empty(),
        "games/minecraft.game_mode returned empty (locale key games.minecraft.game_mode)"
    );
}

#[test]
fn parity_games_minecraft_mob() {
    let value = faker_rust::games::minecraft::mob();
    assert!(
        !value.is_empty(),
        "games/minecraft.mob returned empty (locale key games.minecraft.mobs)"
    );
}

#[test]
fn parity_games_minecraft_status_effect() {
    let value = faker_rust::games::minecraft::status_effect();
    assert!(
        !value.is_empty(),
        "games/minecraft.status_effect returned empty (locale key games.minecraft.status_effect)"
    );
}

#[test]
fn parity_games_pokemon_move() {
    let value = faker_rust::games::pokemon::r#move();
    assert!(
        !value.is_empty(),
        "games/pokemon.move returned empty (locale key games.pokemon.moves)"
    );
}

#[test]
fn parity_games_street_fighter_move() {
    let value = faker_rust::games::street_fighter::r#move();
    assert!(
        !value.is_empty(),
        "games/street_fighter.move returned empty (locale key games.street_fighter.moves)"
    );
}

#[test]
fn parity_games_witcher_book() {
    let value = faker_rust::games::witcher::book();
    assert!(
        !value.is_empty(),
        "games/witcher.book returned empty (locale key games.witcher.books)"
    );
}

#[test]
fn parity_games_witcher_potion() {
    let value = faker_rust::games::witcher::potion();
    assert!(
        !value.is_empty(),
        "games/witcher.potion returned empty (locale key games.witcher.potions)"
    );
}

#[test]
fn parity_games_witcher_sign() {
    let value = faker_rust::games::witcher::sign();
    assert!(
        !value.is_empty(),
        "games/witcher.sign returned empty (locale key games.witcher.signs)"
    );
}

#[test]
fn parity_games_witcher_witcher() {
    let value = faker_rust::games::witcher::witcher();
    assert!(
        !value.is_empty(),
        "games/witcher.witcher returned empty (locale key games.witcher.witchers)"
    );
}

#[test]
fn parity_japanese_media_conan_vehicle() {
    let value = faker_rust::japanese_media::conan::vehicle();
    assert!(
        !value.is_empty(),
        "japanese_media/conan.vehicle returned empty (locale key conan.vehicles)"
    );
}

#[test]
fn parity_japanese_media_cowboy_bebop_episode() {
    let value = faker_rust::japanese_media::cowboy_bebop::episode();
    assert!(
        !value.is_empty(),
        "japanese_media/cowboy_bebop.episode returned empty (locale key cowboy_bebop.episode)"
    );
}

#[test]
fn parity_japanese_media_cowboy_bebop_song() {
    let value = faker_rust::japanese_media::cowboy_bebop::song();
    assert!(
        !value.is_empty(),
        "japanese_media/cowboy_bebop.song returned empty (locale key cowboy_bebop.song)"
    );
}

#[test]
fn parity_japanese_media_naruto_demon() {
    let value = faker_rust::japanese_media::naruto::demon();
    assert!(
        !value.is_empty(),
        "japanese_media/naruto.demon returned empty (locale key naruto.demons)"
    );
}

#[test]
fn parity_japanese_media_one_piece_akuma_no_mi() {
    let value = faker_rust::japanese_media::one_piece::akuma_no_mi();
    assert!(
        !value.is_empty(),
        "japanese_media/one_piece.akuma_no_mi returned empty (locale key one_piece.akuma_no_mi)"
    );
}

#[test]
fn parity_japanese_media_one_piece_island() {
    let value = faker_rust::japanese_media::one_piece::island();
    assert!(
        !value.is_empty(),
        "japanese_media/one_piece.island returned empty (locale key one_piece.islands)"
    );
}

#[test]
fn parity_japanese_media_sword_art_online_game_name() {
    let value = faker_rust::japanese_media::sword_art_online::game_name();
    assert!(
        !value.is_empty(),
        "japanese_media/sword_art_online.game_name returned empty (locale key sword_art_online.game_name)"
    );
}

#[test]
fn parity_japanese_media_sword_art_online_item() {
    let value = faker_rust::japanese_media::sword_art_online::item();
    assert!(
        !value.is_empty(),
        "japanese_media/sword_art_online.item returned empty (locale key sword_art_online.item)"
    );
}

#[test]
fn parity_japanese_media_sword_art_online_real_name() {
    let value = faker_rust::japanese_media::sword_art_online::real_name();
    assert!(
        !value.is_empty(),
        "japanese_media/sword_art_online.real_name returned empty (locale key sword_art_online.real_name)"
    );
}

#[test]
fn parity_movies_avatar_date() {
    let value = faker_rust::movies::avatar::date();
    assert!(
        !value.is_empty(),
        "movies/avatar.date returned empty (locale key avatar.dates)"
    );
}

#[test]
fn parity_movies_avatar_quote() {
    let value = faker_rust::movies::avatar::quote();
    assert!(
        !value.is_empty(),
        "movies/avatar.quote returned empty (locale key avatar.quotes)"
    );
}

#[test]
fn parity_movies_departed_actor() {
    let value = faker_rust::movies::departed::actor();
    assert!(
        !value.is_empty(),
        "movies/departed.actor returned empty (locale key departed.actors)"
    );
}

#[test]
fn parity_movies_ghostbusters_actor() {
    let value = faker_rust::movies::ghostbusters::actor();
    assert!(
        !value.is_empty(),
        "movies/ghostbusters.actor returned empty (locale key ghostbusters.actors)"
    );
}

#[test]
fn parity_movies_harry_potter_book() {
    let value = faker_rust::movies::harry_potter::book();
    assert!(
        !value.is_empty(),
        "movies/harry_potter.book returned empty (locale key harry_potter.books)"
    );
}

#[test]
fn parity_movies_harry_potter_house() {
    let value = faker_rust::movies::harry_potter::house();
    assert!(
        !value.is_empty(),
        "movies/harry_potter.house returned empty (locale key harry_potter.houses)"
    );
}

#[test]
fn parity_movies_harry_potter_location() {
    let value = faker_rust::movies::harry_potter::location();
    assert!(
        !value.is_empty(),
        "movies/harry_potter.location returned empty (locale key harry_potter.locations)"
    );
}

#[test]
fn parity_movies_harry_potter_quote() {
    let value = faker_rust::movies::harry_potter::quote();
    assert!(
        !value.is_empty(),
        "movies/harry_potter.quote returned empty (locale key harry_potter.quotes)"
    );
}

#[test]
fn parity_movies_harry_potter_spell() {
    let value = faker_rust::movies::harry_potter::spell();
    assert!(
        !value.is_empty(),
        "movies/harry_potter.spell returned empty (locale key harry_potter.spells)"
    );
}

#[test]
fn parity_movies_hitchhikers_guide_to_the_galaxy_marvin_quote() {
    let value = faker_rust::movies::hitchhikers_guide_to_the_galaxy::marvin_quote();
    assert!(
        !value.is_empty(),
        "movies/hitchhikers_guide_to_the_galaxy.marvin_quote returned empty (locale key hitchhikers_guide_to_the_galaxy.marvin_quote)"
    );
}

#[test]
fn parity_movies_hitchhikers_guide_to_the_galaxy_specie() {
    let value = faker_rust::movies::hitchhikers_guide_to_the_galaxy::specie();
    assert!(
        !value.is_empty(),
        "movies/hitchhikers_guide_to_the_galaxy.specie returned empty (locale key hitchhikers_guide_to_the_galaxy.species)"
    );
}

#[test]
fn parity_movies_hobbit_thorins_company() {
    let value = faker_rust::movies::hobbit::thorins_company();
    assert!(
        !value.is_empty(),
        "movies/hobbit.thorins_company returned empty (locale key tolkien.hobbit.thorins_company)"
    );
}

#[test]
fn parity_movies_lebowski_actor() {
    let value = faker_rust::movies::lebowski::actor();
    assert!(
        !value.is_empty(),
        "movies/lebowski.actor returned empty (locale key lebowski.actors)"
    );
}

#[test]
fn parity_movies_star_wars_call_number() {
    let value = faker_rust::movies::star_wars::call_number();
    assert!(
        !value.is_empty(),
        "movies/star_wars.call_number returned empty (locale key star_wars.call_numbers)"
    );
}

#[test]
fn parity_movies_star_wars_call_squadron() {
    let value = faker_rust::movies::star_wars::call_squadron();
    assert!(
        !value.is_empty(),
        "movies/star_wars.call_squadron returned empty (locale key star_wars.call_squadrons)"
    );
}

#[test]
fn parity_movies_star_wars_droid() {
    let value = faker_rust::movies::star_wars::droid();
    assert!(
        !value.is_empty(),
        "movies/star_wars.droid returned empty (locale key star_wars.droids)"
    );
}

#[test]
fn parity_movies_star_wars_planet() {
    let value = faker_rust::movies::star_wars::planet();
    assert!(
        !value.is_empty(),
        "movies/star_wars.planet returned empty (locale key star_wars.planets)"
    );
}

#[test]
fn parity_movies_star_wars_specie() {
    let value = faker_rust::movies::star_wars::specie();
    assert!(
        !value.is_empty(),
        "movies/star_wars.specie returned empty (locale key star_wars.species)"
    );
}

#[test]
fn parity_movies_star_wars_vehicle() {
    let value = faker_rust::movies::star_wars::vehicle();
    assert!(
        !value.is_empty(),
        "movies/star_wars.vehicle returned empty (locale key star_wars.vehicles)"
    );
}

#[test]
fn parity_movies_the_room_actor() {
    let value = faker_rust::movies::the_room::actor();
    assert!(
        !value.is_empty(),
        "movies/the_room.actor returned empty (locale key the_room.actors)"
    );
}

#[test]
fn parity_movies_the_room_location() {
    let value = faker_rust::movies::the_room::location();
    assert!(
        !value.is_empty(),
        "movies/the_room.location returned empty (locale key the_room.locations)"
    );
}

#[test]
fn parity_movies_tron_game() {
    let value = faker_rust::movies::tron::game();
    assert!(
        !value.is_empty(),
        "movies/tron.game returned empty (locale key tron.games)"
    );
}

#[test]
fn parity_movies_tron_tagline() {
    let value = faker_rust::movies::tron::tagline();
    assert!(
        !value.is_empty(),
        "movies/tron.tagline returned empty (locale key tron.taglines)"
    );
}

#[test]
fn parity_movies_tron_vehicle() {
    let value = faker_rust::movies::tron::vehicle();
    assert!(
        !value.is_empty(),
        "movies/tron.vehicle returned empty (locale key tron.vehicles)"
    );
}

#[test]
fn parity_movies_v_for_vendetta_speech() {
    let value = faker_rust::movies::v_for_vendetta::speech();
    assert!(
        !value.is_empty(),
        "movies/v_for_vendetta.speech returned empty (locale key v_for_vendetta.speeches)"
    );
}

#[test]
fn parity_music_grateful_dead_player() {
    let value = faker_rust::music::grateful_dead::player();
    assert!(
        !value.is_empty(),
        "music/grateful_dead.player returned empty (locale key grateful_dead.players)"
    );
}

#[test]
fn parity_music_hiphop_subgenres() {
    let value = faker_rust::music::hiphop::subgenres();
    assert!(
        !value.is_empty(),
        "music/hiphop.subgenres returned empty (locale key music.hiphop.subgenres)"
    );
}

#[test]
fn parity_music_music_mambo_no_5() {
    let value = faker_rust::music::music::mambo_no_5();
    assert!(
        !value.is_empty(),
        "music/music.mambo_no_5 returned empty (locale key music.mambo_no_5)"
    );
}

#[test]
fn parity_music_opera_beethoven() {
    let value = faker_rust::music::opera::beethoven();
    assert!(
        !value.is_empty(),
        "music/opera.beethoven returned empty (locale key opera.german.by_ludwig_van_beethoven)"
    );
}

#[test]
fn parity_music_opera_bellini() {
    let value = faker_rust::music::opera::bellini();
    assert!(
        !value.is_empty(),
        "music/opera.bellini returned empty (locale key opera.italian.by_vincenzo_bellini)"
    );
}

#[test]
fn parity_music_opera_berg() {
    let value = faker_rust::music::opera::berg();
    assert!(
        !value.is_empty(),
        "music/opera.berg returned empty (locale key opera.german.by_alban_berg)"
    );
}

#[test]
fn parity_music_opera_berlioz() {
    let value = faker_rust::music::opera::berlioz();
    assert!(
        !value.is_empty(),
        "music/opera.berlioz returned empty (locale key opera.french.by_hector_berlioz)"
    );
}

#[test]
fn parity_music_opera_bizet() {
    let value = faker_rust::music::opera::bizet();
    assert!(
        !value.is_empty(),
        "music/opera.bizet returned empty (locale key opera.french.by_georges_bizet)"
    );
}

#[test]
fn parity_music_opera_donizetti() {
    let value = faker_rust::music::opera::donizetti();
    assert!(
        !value.is_empty(),
        "music/opera.donizetti returned empty (locale key opera.italian.by_gaetano_donizetti)"
    );
}

#[test]
fn parity_music_opera_gluck() {
    let value = faker_rust::music::opera::gluck();
    assert!(
        !value.is_empty(),
        "music/opera.gluck returned empty (locale key opera.italian.by_christoph_willibald_gluck)"
    );
}

#[test]
fn parity_music_opera_gluck_french() {
    let value = faker_rust::music::opera::gluck_french();
    assert!(
        !value.is_empty(),
        "music/opera.gluck_french returned empty (locale key opera.french.by_christoph_willibald_gluck)"
    );
}

#[test]
fn parity_music_opera_gluck_italian() {
    let value = faker_rust::music::opera::gluck_italian();
    assert!(
        !value.is_empty(),
        "music/opera.gluck_italian returned empty (locale key opera.italian.by_christoph_willibald_gluck)"
    );
}

#[test]
fn parity_music_opera_gounod() {
    let value = faker_rust::music::opera::gounod();
    assert!(
        !value.is_empty(),
        "music/opera.gounod returned empty (locale key opera.french.by_charles_gounod)"
    );
}

#[test]
fn parity_music_opera_mozart() {
    let value = faker_rust::music::opera::mozart();
    assert!(
        !value.is_empty(),
        "music/opera.mozart returned empty (locale key opera.italian.by_wolfgang_amadeus_mozart)"
    );
}

#[test]
fn parity_music_opera_mozart_german() {
    let value = faker_rust::music::opera::mozart_german();
    assert!(
        !value.is_empty(),
        "music/opera.mozart_german returned empty (locale key opera.german.by_wolfgang_amadeus_mozart)"
    );
}

#[test]
fn parity_music_opera_mozart_italian() {
    let value = faker_rust::music::opera::mozart_italian();
    assert!(
        !value.is_empty(),
        "music/opera.mozart_italian returned empty (locale key opera.italian.by_wolfgang_amadeus_mozart)"
    );
}

#[test]
fn parity_music_opera_ravel() {
    let value = faker_rust::music::opera::ravel();
    assert!(
        !value.is_empty(),
        "music/opera.ravel returned empty (locale key opera.french.by_maurice_ravel)"
    );
}

#[test]
fn parity_music_opera_rossini() {
    let value = faker_rust::music::opera::rossini();
    assert!(
        !value.is_empty(),
        "music/opera.rossini returned empty (locale key opera.italian.by_gioacchino_rossini)"
    );
}

#[test]
fn parity_music_opera_saint_saens() {
    let value = faker_rust::music::opera::saint_saens();
    assert!(
        !value.is_empty(),
        "music/opera.saint_saens returned empty (locale key opera.french.by_camille_saint_saens)"
    );
}

#[test]
fn parity_music_opera_schubert() {
    let value = faker_rust::music::opera::schubert();
    assert!(
        !value.is_empty(),
        "music/opera.schubert returned empty (locale key opera.german.by_franz_schubert)"
    );
}

#[test]
fn parity_music_opera_schumann() {
    let value = faker_rust::music::opera::schumann();
    assert!(
        !value.is_empty(),
        "music/opera.schumann returned empty (locale key opera.german.by_robert_schumann)"
    );
}

#[test]
fn parity_music_opera_strauss() {
    let value = faker_rust::music::opera::strauss();
    assert!(
        !value.is_empty(),
        "music/opera.strauss returned empty (locale key opera.german.by_richard_strauss)"
    );
}

#[test]
fn parity_music_opera_verdi() {
    let value = faker_rust::music::opera::verdi();
    assert!(
        !value.is_empty(),
        "music/opera.verdi returned empty (locale key opera.italian.by_giuseppe_verdi)"
    );
}

#[test]
fn parity_music_opera_wagner() {
    let value = faker_rust::music::opera::wagner();
    assert!(
        !value.is_empty(),
        "music/opera.wagner returned empty (locale key opera.german.by_richard_wagner)"
    );
}

#[test]
fn parity_music_opera_weber() {
    let value = faker_rust::music::opera::weber();
    assert!(
        !value.is_empty(),
        "music/opera.weber returned empty (locale key opera.german.by_carl_maria_von_weber)"
    );
}

#[test]
fn parity_music_pearl_jam_album() {
    let value = faker_rust::music::pearl_jam::album();
    assert!(
        !value.is_empty(),
        "music/pearl_jam.album returned empty (locale key pearl_jam.albums)"
    );
}

#[test]
fn parity_music_pearl_jam_musician() {
    let value = faker_rust::music::pearl_jam::musician();
    assert!(
        !value.is_empty(),
        "music/pearl_jam.musician returned empty (locale key pearl_jam.musicians)"
    );
}

#[test]
fn parity_music_phish_album() {
    let value = faker_rust::music::phish::album();
    assert!(
        !value.is_empty(),
        "music/phish.album returned empty (locale key phish.albums)"
    );
}

#[test]
fn parity_music_phish_musician() {
    let value = faker_rust::music::phish::musician();
    assert!(
        !value.is_empty(),
        "music/phish.musician returned empty (locale key phish.musicians)"
    );
}

#[test]
fn parity_music_prince_lyric() {
    let value = faker_rust::music::prince::lyric();
    assert!(
        !value.is_empty(),
        "music/prince.lyric returned empty (locale key prince.lyric)"
    );
}

#[test]
fn parity_music_rock_band_song() {
    let value = faker_rust::music::rock_band::song();
    assert!(
        !value.is_empty(),
        "music/rock_band.song returned empty (locale key rock_band.song)"
    );
}

#[test]
fn parity_music_rush_player() {
    let value = faker_rust::music::rush::player();
    assert!(
        !value.is_empty(),
        "music/rush.player returned empty (locale key rush.players)"
    );
}

#[test]
fn parity_music_smashing_pumpkins_lyric() {
    let value = faker_rust::music::smashing_pumpkins::lyric();
    assert!(
        !value.is_empty(),
        "music/smashing_pumpkins.lyric returned empty (locale key smashing_pumpkins.lyric)"
    );
}

#[test]
fn parity_music_smashing_pumpkins_musician() {
    let value = faker_rust::music::smashing_pumpkins::musician();
    assert!(
        !value.is_empty(),
        "music/smashing_pumpkins.musician returned empty (locale key smashing_pumpkins.musicians)"
    );
}

#[test]
fn parity_quotes_quote_famous_last_words() {
    let value = faker_rust::quote::famous_last_words();
    assert!(
        !value.is_empty(),
        "quotes/quote.famous_last_words returned empty (locale key quote.famous_last_words)"
    );
}

#[test]
fn parity_quotes_quote_mitch_hedberg() {
    let value = faker_rust::quote::mitch_hedberg();
    assert!(
        !value.is_empty(),
        "quotes/quote.mitch_hedberg returned empty (locale key quote.mitch_hedberg)"
    );
}

#[test]
fn parity_quotes_quote_most_interesting_man_in_the_world() {
    let value = faker_rust::quote::most_interesting_man_in_the_world();
    assert!(
        !value.is_empty(),
        "quotes/quote.most_interesting_man_in_the_world returned empty (locale key quote.most_interesting_man_in_the_world)"
    );
}

#[test]
fn parity_quotes_quote_singular_siegler() {
    let value = faker_rust::quote::singular_siegler();
    assert!(
        !value.is_empty(),
        "quotes/quote.singular_siegler returned empty (locale key quote.singular_siegler)"
    );
}

#[test]
fn parity_quotes_quote_yoda() {
    let value = faker_rust::quote::yoda();
    assert!(
        !value.is_empty(),
        "quotes/quote.yoda returned empty (locale key quote.yoda)"
    );
}

#[test]
fn parity_sports_football_coach() {
    let value = faker_rust::sports::football::coach();
    assert!(
        !value.is_empty(),
        "sports/football.coach returned empty (locale key football.coaches)"
    );
}

#[test]
fn parity_sports_sport_ancient_olympics_sport() {
    let value = faker_rust::sports::sport::ancient_olympics_sport();
    assert!(
        !value.is_empty(),
        "sports/sport.ancient_olympics_sport returned empty (locale key sport.ancient_olympics)"
    );
}

#[test]
fn parity_sports_sport_summer_olympics_sport() {
    let value = faker_rust::sports::sport::summer_olympics_sport();
    assert!(
        !value.is_empty(),
        "sports/sport.summer_olympics_sport returned empty (locale key sport.summer_olympics)"
    );
}

#[test]
fn parity_sports_sport_summer_paralympics_sport() {
    let value = faker_rust::sports::sport::summer_paralympics_sport();
    assert!(
        !value.is_empty(),
        "sports/sport.summer_paralympics_sport returned empty (locale key sport.summer_paralympics)"
    );
}

#[test]
fn parity_sports_sport_unusual_sport() {
    let value = faker_rust::sports::sport::unusual_sport();
    assert!(
        !value.is_empty(),
        "sports/sport.unusual_sport returned empty (locale key sport.unusual)"
    );
}

#[test]
fn parity_sports_sport_winter_olympics_sport() {
    let value = faker_rust::sports::sport::winter_olympics_sport();
    assert!(
        !value.is_empty(),
        "sports/sport.winter_olympics_sport returned empty (locale key sport.winter_olympics)"
    );
}

#[test]
fn parity_sports_sport_winter_paralympics_sport() {
    let value = faker_rust::sports::sport::winter_paralympics_sport();
    assert!(
        !value.is_empty(),
        "sports/sport.winter_paralympics_sport returned empty (locale key sport.winter_paralympics)"
    );
}

#[test]
fn parity_sports_volleyball_coach() {
    let value = faker_rust::sports::volleyball::coach();
    assert!(
        !value.is_empty(),
        "sports/volleyball.coach returned empty (locale key volleyball.coach)"
    );
}

#[test]
fn parity_sports_volleyball_formation() {
    let value = faker_rust::sports::volleyball::formation();
    assert!(
        !value.is_empty(),
        "sports/volleyball.formation returned empty (locale key volleyball.formation)"
    );
}

#[test]
fn parity_tv_shows_archer_location() {
    let value = faker_rust::tv_shows::archer::location();
    assert!(
        !value.is_empty(),
        "tv_shows/archer.location returned empty (locale key archer.locations)"
    );
}

#[test]
fn parity_tv_shows_bojack_horseman_tongue_twister() {
    let value = faker_rust::tv_shows::bojack_horseman::tongue_twister();
    assert!(
        !value.is_empty(),
        "tv_shows/bojack_horseman.tongue_twister returned empty (locale key bojack_horseman.tongue_twisters)"
    );
}

#[test]
fn parity_tv_shows_community_characters() {
    let value = faker_rust::tv_shows::community::characters();
    assert!(
        !value.is_empty(),
        "tv_shows/community.characters returned empty (locale key community.characters)"
    );
}

#[test]
fn parity_tv_shows_community_quotes() {
    let value = faker_rust::tv_shows::community::quotes();
    assert!(
        !value.is_empty(),
        "tv_shows/community.quotes returned empty (locale key community.quotes)"
    );
}

#[test]
fn parity_tv_shows_dr_who_specie() {
    let value = faker_rust::tv_shows::dr_who::specie();
    assert!(
        !value.is_empty(),
        "tv_shows/dr_who.specie returned empty (locale key dr_who.species)"
    );
}

#[test]
fn parity_tv_shows_dumb_and_dumber_actor() {
    let value = faker_rust::tv_shows::dumb_and_dumber::actor();
    assert!(
        !value.is_empty(),
        "tv_shows/dumb_and_dumber.actor returned empty (locale key dumb_and_dumber.actors)"
    );
}

#[test]
fn parity_tv_shows_final_space_vehicle() {
    let value = faker_rust::tv_shows::final_space::vehicle();
    assert!(
        !value.is_empty(),
        "tv_shows/final_space.vehicle returned empty (locale key final_space.vehicles)"
    );
}

#[test]
fn parity_tv_shows_how_i_met_your_mother_quote() {
    let value = faker_rust::tv_shows::how_i_met_your_mother::quote();
    assert!(
        !value.is_empty(),
        "tv_shows/how_i_met_your_mother.quote returned empty (locale key how_i_met_your_mother.quote)"
    );
}

#[test]
fn parity_tv_shows_seinfeld_business() {
    let value = faker_rust::tv_shows::seinfeld::business();
    assert!(
        !value.is_empty(),
        "tv_shows/seinfeld.business returned empty (locale key seinfeld.business)"
    );
}

#[test]
fn parity_tv_shows_south_park_episode_name() {
    let value = faker_rust::tv_shows::south_park::episode_name();
    assert!(
        !value.is_empty(),
        "tv_shows/south_park.episode_name returned empty (locale key south_park.episodes)"
    );
}

#[test]
fn parity_tv_shows_spongebob_episode() {
    let value = faker_rust::tv_shows::spongebob::episode();
    assert!(
        !value.is_empty(),
        "tv_shows/spongebob.episode returned empty (locale key spongebob.episodes)"
    );
}

#[test]
fn parity_tv_shows_star_trek_specie() {
    let value = faker_rust::tv_shows::star_trek::specie();
    assert!(
        !value.is_empty(),
        "tv_shows/star_trek.specie returned empty (locale key star_trek.specie)"
    );
}

#[test]
fn parity_tv_shows_star_trek_villain() {
    let value = faker_rust::tv_shows::star_trek::villain();
    assert!(
        !value.is_empty(),
        "tv_shows/star_trek.villain returned empty (locale key star_trek.villain)"
    );
}

#[test]
fn parity_tv_shows_supernatural_weapon() {
    let value = faker_rust::tv_shows::supernatural::weapon();
    assert!(
        !value.is_empty(),
        "tv_shows/supernatural.weapon returned empty (locale key supernatural.weapon)"
    );
}

#[test]
fn parity_tv_shows_the_expanse_ship() {
    let value = faker_rust::tv_shows::the_expanse::ship();
    assert!(
        !value.is_empty(),
        "tv_shows/the_expanse.ship returned empty (locale key the_expanse.ships)"
    );
}

#[test]
fn parity_tv_shows_the_it_crowd_actor() {
    let value = faker_rust::tv_shows::the_it_crowd::actor();
    assert!(
        !value.is_empty(),
        "tv_shows/the_it_crowd.actor returned empty (locale key the_it_crowd.actors)"
    );
}

#[test]
fn parity_tv_shows_the_thick_of_it_department() {
    let value = faker_rust::tv_shows::the_thick_of_it::department();
    assert!(
        !value.is_empty(),
        "tv_shows/the_thick_of_it.department returned empty (locale key the_thick_of_it.departments)"
    );
}

#[test]
fn parity_tv_shows_the_thick_of_it_position() {
    let value = faker_rust::tv_shows::the_thick_of_it::position();
    assert!(
        !value.is_empty(),
        "tv_shows/the_thick_of_it.position returned empty (locale key the_thick_of_it.positions)"
    );
}

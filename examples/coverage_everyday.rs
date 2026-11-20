//! Everyday generator coverage example.
//!
//! Exercises every public top-level generator in the default category that was
//! not demonstrated by the earlier examples. Each call uses the real Rust API
//! path and prints the returned value.

fn main() {
    println!("=== Everyday generators ===\n");

    println!("TEXT AND IDENTIFIERS");
    println!(
        "  alphanumeric: {:?}",
        faker_rust::alphanumeric::alphanumeric(8)
    );
    println!("  alpha:        {:?}", faker_rust::alphanumeric::alpha(8));
    println!("  numeric:      {:?}", faker_rust::alphanumeric::numeric(8));

    println!("\nANCIENT AND APPLIANCES");
    println!("  god:          {:?}", faker_rust::ancient::god());
    println!("  primordial:   {:?}", faker_rust::ancient::primordial());
    println!("  appliance:    {:?}", faker_rust::appliance::brand());
    println!("  equipment:    {:?}", faker_rust::appliance::equipment());
    println!("  artist:       {:?}", faker_rust::artist::name());

    println!("\nCAMERA AND COIN");
    println!("  camera brand: {:?}", faker_rust::camera::brand());
    println!("  camera model: {:?}", faker_rust::camera::model());
    println!("  coin flip:    {:?}", faker_rust::coin::flip());
    println!("  coin name:    {:?}", faker_rust::coin::name());

    println!("\nCOMPASS AND CONSTRUCTION");
    println!("  direction:    {:?}", faker_rust::compass::direction());
    println!("  abbreviation: {:?}", faker_rust::compass::abbreviation());
    println!("  azimuth:      {:?}", faker_rust::compass::azimuth());
    println!("  material:     {:?}", faker_rust::construction::material());
    println!("  trade:        {:?}", faker_rust::construction::trade());

    println!("\nDEMOGRAPHIC AND DESSERT");
    println!("  race:         {:?}", faker_rust::demographic::race());
    println!("  demonym:      {:?}", faker_rust::demographic::demonym());
    println!("  dessert:      {:?}", faker_rust::dessert::variety());
    println!("  topping:      {:?}", faker_rust::dessert::topping());

    println!("\nEDUCATION AND EMOTION");
    println!("  school:       {:?}", faker_rust::educator::school_name());
    println!("  university:   {:?}", faker_rust::educator::university());
    println!("  adjective:    {:?}", faker_rust::emotion::adjective());
    println!("  noun:         {:?}", faker_rust::emotion::noun());

    println!("\nNAMES AND HOBBIES");
    println!("  funny name:   {:?}", faker_rust::funny_name::name());
    println!(
        "  four words:   {:?}",
        faker_rust::funny_name::four_word_name()
    );
    println!("  hobby:        {:?}", faker_rust::hobby::activity());
    println!("  furniture:    {:?}", faker_rust::house::furniture());
    println!("  room:         {:?}", faker_rust::house::room());

    println!("\nSTRUCTURED OUTPUT");
    println!("  json object:  {:?}", faker_rust::json::object());
    println!("  json array:   {:?}", faker_rust::json::array());
    println!("  md headers:   {:?}", faker_rust::markdown::headers());
    println!("  md table row: {:?}", faker_rust::markdown::table_row());

    println!("\nNATION AND ALPHABET");
    println!("  nationality:  {:?}", faker_rust::nation::nationality());
    println!("  capital:      {:?}", faker_rust::nation::capital_city());
    println!(
        "  code word:    {:?}",
        faker_rust::nato_phonetic_alphabet::code_word()
    );

    println!("\nRELATIONSHIPS AND FOOD");
    println!("  direct:       {:?}", faker_rust::relationship::direct());
    println!("  in law:       {:?}", faker_rust::relationship::in_law());
    println!("  restaurant:   {:?}", faker_rust::restaurant::name());
    println!("  review:       {:?}", faker_rust::restaurant::review());
    println!("  tea:          {:?}", faker_rust::tea::variety());

    println!("\nTEAMS AND THEATER");
    println!("  team:         {:?}", faker_rust::team::name());
    println!("  mascot:       {:?}", faker_rust::team::mascot());
    println!("  theater:      {:?}", faker_rust::theater::name());
    println!("  play:         {:?}", faker_rust::theater::play());

    println!("\nQUOTES");
    println!("  famous:       {:?}", faker_rust::quote::famous());
    println!("  matz:         {:?}", faker_rust::quote::matz());
}

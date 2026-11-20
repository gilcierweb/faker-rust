//! Gem-documented method-name aliases example.
//!
//! Shows the exact method names documented by the Ruby Faker gem. Each Rust
//! module also keeps its original normalized name.

fn main() {
    println!("=== Gem-documented API aliases ===\n");

    println!("ADDRESS AND BOOKS");
    println!("  postcode: {:?}", faker_rust::address::postcode());
    println!("  zip:      {:?}", faker_rust::address::zip());
    println!("  tome:     {:?}", faker_rust::books::lovecraft::tome());

    println!("\nCOLOR AND BIRD");
    println!("  hex:      {:?}", faker_rust::color::hex_color());
    println!("  hsl:      {:?}", faker_rust::color::hsl_color());
    println!("  rgb:      {:?}", faker_rust::color::rgb_color());
    println!("  bird col: {:?}", faker_rust::creature::bird::color());
    println!("  bird adj: {:?}", faker_rust::creature::bird::adjective());

    println!("\nIDENTIFIERS AND INTERNET");
    println!(
        "  za id:    {:?}",
        faker_rust::id_number::south_african_id_number()
    );
    println!("  ipv4:     {:?}", faker_rust::internet::ip_v4_address());
    println!("  ipv6:     {:?}", faker_rust::internet::ip_v6_address());
    println!(
        "  private:  {:?}",
        faker_rust::internet::private_ip_v4_address()
    );
    println!("  middle:   {:?}", faker_rust::name::middle_name());

    println!("\nTRAVEL AND VEHICLE");
    println!("  iata:     {:?}", faker_rust::travel::airport::iata());
    println!("  doors:    {:?}", faker_rust::vehicle::door_count());
    println!("  engine:   {:?}", faker_rust::vehicle::engine_size());
    println!("  km:       {:?}", faker_rust::vehicle::kilometrage());
}

//! Compatibility and helper generator coverage example.
//!
//! Exercises public modules that have no dedicated documentation page but are
//! part of the crate API: base helpers, compatibility re-exports and a few
//! catalogue modules.

fn main() {
    println!("=== Compatibility and helper generators ===\n");

    println!("BASE HELPERS");
    println!("  numerify:  {:?}", faker_rust::base::numerify("###-##"));
    println!("  letterify: {:?}", faker_rust::base::letterify("???-???"));
    println!("  bothify:   {:?}", faker_rust::base::bothify("??##"));
    println!("  parse:     {:?}", faker_rust::base::parse("#{###}"));

    println!("\nCATALOGUES");
    println!("  culture:   {:?}", faker_rust::culture::planet());
    println!("  humor:     {:?}", faker_rust::humor::name());
    println!("  locations: {:?}", faker_rust::locations::community());

    println!("\nGAMES");
    println!("  control:   {:?}", faker_rust::games::control::character());
    println!("  tarkov:    {:?}", faker_rust::games::tarkov::location());
    println!(
        "  touhou:    {:?}",
        faker_rust::games::touhou_project::game()
    );

    println!("\nMOVIES");
    println!(
        "  hackers:   {:?}",
        faker_rust::movies::hackers::character()
    );
    println!(
        "  guide:     {:?}",
        faker_rust::movies::hitchhikers_guide::character()
    );
    println!(
        "  the room:  {:?}",
        faker_rust::movies::the_room::character()
    );

    println!("\nTV SHOW ALIASES");
    println!(
        "  fresh:     {:?}",
        faker_rust::tv_shows::fresh_prince::character()
    );
    println!(
        "  himym:     {:?}",
        faker_rust::tv_shows::himym::character()
    );
    println!("  ru paul:   {:?}", faker_rust::tv_shows::ru_paul::queen());
}

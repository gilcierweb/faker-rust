//! Movie, music and regional generator coverage example.
//!
//! Exercises every public generator in these categories that was not
//! demonstrated by the earlier examples.

fn main() {
    println!("=== Movie, music and location generators ===\n");

    println!("MOVIES");
    println!(
        "  back to future:   {:?}",
        faker_rust::movies::back_to_the_future::character()
    );
    println!(
        "  departed:         {:?}",
        faker_rust::movies::departed::quote()
    );
    println!(
        "  ghostbusters:     {:?}",
        faker_rust::movies::ghostbusters::character()
    );
    println!(
        "  hitchhiker:       {:?}",
        faker_rust::movies::hitchhikers_guide_to_the_galaxy::character()
    );
    println!(
        "  hobbit:           {:?}",
        faker_rust::movies::hobbit::character()
    );
    println!(
        "  how to train:     {:?}",
        faker_rust::movies::how_to_train_your_dragon::dragon()
    );
    println!(
        "  princess bride:   {:?}",
        faker_rust::movies::princess_bride::character()
    );
    println!(
        "  tron:             {:?}",
        faker_rust::movies::tron::character()
    );
    println!(
        "  v for vendetta:   {:?}",
        faker_rust::movies::v_for_vendetta::quote()
    );

    println!("\nMUSIC");
    println!(
        "  grateful dead:    {:?}",
        faker_rust::music::grateful_dead::song()
    );
    println!(
        "  hiphop:           {:?}",
        faker_rust::music::hiphop::artist()
    );
    println!("  phish:            {:?}", faker_rust::music::phish::song());
    println!("  rush:             {:?}", faker_rust::music::rush::song());
    println!(
        "  smash pumpkins:   {:?}",
        faker_rust::music::smashing_pumpkins::song()
    );
    println!(
        "  umphreys mcgee:   {:?}",
        faker_rust::music::umphreys_mcgee::song()
    );

    println!("\nREGIONAL");
    println!(
        "  australia:        {:?}",
        faker_rust::locations::australia::location()
    );
    println!(
        "  animal:           {:?}",
        faker_rust::locations::australia::animal()
    );
}

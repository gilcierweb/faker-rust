//! Game and fantasy generator coverage example.
//!
//! Exercises every public game/fantasy generator that was not demonstrated by
//! the earlier examples.

fn main() {
    println!("=== Fantasy and game generators ===\n");

    println!("FANTASY");
    println!(
        "  tolkien race:     {:?}",
        faker_rust::fantasy::tolkien::race()
    );
    println!(
        "  tolkien location: {:?}",
        faker_rust::fantasy::tolkien::location()
    );

    println!("\nCLASH OF CLANS / DOTA");
    println!(
        "  clash troop:      {:?}",
        faker_rust::games::clash_of_clans::troop()
    );
    println!("  dota hero:        {:?}", faker_rust::games::dota::hero());
    println!("  dota item:        {:?}", faker_rust::games::dota::item());

    println!("\nELDER SCROLLS / FALLOUT / HALF-LIFE");
    println!(
        "  elder race:       {:?}",
        faker_rust::games::elder_scrolls::race()
    );
    println!(
        "  fallout faction:  {:?}",
        faker_rust::games::fallout::faction()
    );
    println!(
        "  half-life enemy:  {:?}",
        faker_rust::games::half_life::enemy()
    );

    println!("\nHEROES / LEAGUE OF LEGENDS");
    println!(
        "  heroes name:      {:?}",
        faker_rust::games::heroes::name()
    );
    println!(
        "  heroes specialty: {:?}",
        faker_rust::games::heroes::specialty()
    );
    println!(
        "  storm hero:       {:?}",
        faker_rust::games::heroes_of_the_storm::hero()
    );
    println!(
        "  lol champion:     {:?}",
        faker_rust::games::league_of_legends::champion()
    );

    println!("\nMYST / OVERWATCH / SONIC");
    println!("  myst game:        {:?}", faker_rust::games::myst::game());
    println!(
        "  overwatch hero:   {:?}",
        faker_rust::games::overwatch::hero()
    );
    println!(
        "  sonic zone:       {:?}",
        faker_rust::games::sonic_the_hedgehog::zone()
    );

    println!("\nGENERIC GAME");
    println!("  game title:    {:?}", faker_rust::games::game::title());
    println!("  game genre:    {:?}", faker_rust::games::game::genre());
    println!("  game platform: {:?}", faker_rust::games::game::platform());

    println!("\nSMASH BROS / WARHAMMER / WITCHER");
    println!(
        "  smash fighter:    {:?}",
        faker_rust::games::super_smash_bros::fighter()
    );
    println!(
        "  warhammer hero:   {:?}",
        faker_rust::games::warhammer_fantasy::hero()
    );
    println!(
        "  witcher monster:  {:?}",
        faker_rust::games::witcher::monster()
    );
}

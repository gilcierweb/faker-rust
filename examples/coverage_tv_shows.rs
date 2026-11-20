//! TV show generator coverage example.
//!
//! Exercises every public TV show generator that was not demonstrated by the
//! earlier examples. One representative generator call per show keeps the
//! example readable while covering the whole module family.

fn main() {
    println!("=== TV show generators ===\n");

    println!(
        "  aqua teen:        {:?}",
        faker_rust::tv_shows::aqua_teen_hunger_force::character()
    );
    println!(
        "  archer:           {:?}",
        faker_rust::tv_shows::archer::character()
    );
    println!(
        "  big bang theory:  {:?}",
        faker_rust::tv_shows::big_bang_theory::character()
    );
    println!(
        "  bojack:           {:?}",
        faker_rust::tv_shows::bojack_horseman::character()
    );
    println!(
        "  brooklyn 99:      {:?}",
        faker_rust::tv_shows::brooklyn_nine_nine::character()
    );
    println!(
        "  buffy:            {:?}",
        faker_rust::tv_shows::buffy::character()
    );
    println!(
        "  community:        {:?}",
        faker_rust::tv_shows::community::character()
    );
    println!(
        "  dr who:           {:?}",
        faker_rust::tv_shows::dr_who::character()
    );
    println!(
        "  dumb and dumber:  {:?}",
        faker_rust::tv_shows::dumb_and_dumber::character()
    );
    println!(
        "  family guy:       {:?}",
        faker_rust::tv_shows::family_guy::character()
    );
    println!(
        "  final space:      {:?}",
        faker_rust::tv_shows::final_space::character()
    );
    println!(
        "  futurama:         {:?}",
        faker_rust::tv_shows::futurama::character()
    );
    println!(
        "  hey arnold:       {:?}",
        faker_rust::tv_shows::hey_arnold::character()
    );
    println!(
        "  himym:            {:?}",
        faker_rust::tv_shows::how_i_met_your_mother::character()
    );
    println!(
        "  michael scott:    {:?}",
        faker_rust::tv_shows::michael_scott::quote()
    );
    println!(
        "  new girl:         {:?}",
        faker_rust::tv_shows::new_girl::character()
    );
    println!(
        "  parks and rec:    {:?}",
        faker_rust::tv_shows::parks_and_rec::character()
    );
    println!(
        "  seinfeld:         {:?}",
        faker_rust::tv_shows::seinfeld::character()
    );
    println!(
        "  silicon valley:   {:?}",
        faker_rust::tv_shows::silicon_valley::character()
    );
    println!(
        "  south park:       {:?}",
        faker_rust::tv_shows::south_park::character()
    );
    println!(
        "  spongebob:        {:?}",
        faker_rust::tv_shows::spongebob::character()
    );
    println!(
        "  stargate:         {:?}",
        faker_rust::tv_shows::stargate::character()
    );
    println!(
        "  stranger things:  {:?}",
        faker_rust::tv_shows::stranger_things::character()
    );
    println!(
        "  suits:            {:?}",
        faker_rust::tv_shows::suits::character()
    );
    println!(
        "  the expanse:      {:?}",
        faker_rust::tv_shows::the_expanse::character()
    );
    println!(
        "  it crowd:         {:?}",
        faker_rust::tv_shows::the_it_crowd::character()
    );
    println!(
        "  thick of it:      {:?}",
        faker_rust::tv_shows::the_thick_of_it::character()
    );
    println!(
        "  twin peaks:       {:?}",
        faker_rust::tv_shows::twin_peaks::character()
    );
    println!(
        "  venture bros:     {:?}",
        faker_rust::tv_shows::venture_bros::character()
    );
}

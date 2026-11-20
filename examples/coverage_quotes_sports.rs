//! Quote, religion and sports generator coverage example.
//!
//! Exercises every public generator in these categories that was not
//! demonstrated by the earlier examples.

fn main() {
    println!("=== Quote, religion and sports generators ===\n");

    println!("QUOTES");
    println!(
        "  chiquito:         {:?}",
        faker_rust::quotes::chiquito::expression()
    );
    println!(
        "  rajnikanth:       {:?}",
        faker_rust::quotes::rajnikanth::joke()
    );
    println!(
        "  hamlet:           {:?}",
        faker_rust::quotes::shakespeare::hamlet_quote()
    );

    println!("\nRELIGION");
    println!(
        "  bible character:  {:?}",
        faker_rust::religion::bible::character()
    );
    println!(
        "  bible location:   {:?}",
        faker_rust::religion::bible::location()
    );

    println!("\nSPORTS");
    println!("  chess player:     {:?}", faker_rust::chess::player());
    println!("  chess opening:    {:?}", faker_rust::chess::opening());
    println!(
        "  mountaineer:      {:?}",
        faker_rust::sports::mountaineering::mountaineer()
    );
}

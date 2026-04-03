//! Education & Science Example
//! Demonstrates educational and scientific data generators

use faker_rust::{
    color, greek_philosophers, measurement, mountain, science,
    space, university, verb, adjective,
};

fn main() {
    println!("=== Faker-Rust Education & Science Examples ===\n");

    // University
    println!("🎓 UNIVERSITY:");
    println!("  Name:               {}", university::name());
    println!("  Prefix:             {}", university::prefix());
    println!("  Suffix:             {}", university::suffix());
    println!();

    // Science
    println!("🔬 SCIENCE:");
    println!("  Element:            {}", science::element());
    println!("  Element Symbol:     {}", science::element_symbol());
    println!("  Scientist:          {}", science::scientist());
    println!();

    // Space
    println!("🚀 SPACE:");
    println!("  Planet:             {}", space::planet());
    println!("  Moon:               {}", space::moon());
    println!("  Galaxy:             {}", space::galaxy());
    println!("  Constellation:      {}", space::constellation());
    println!();

    // Measurement
    println!("📏 MEASUREMENT:");
    println!("  Length:             {}", measurement::length());
    println!("  Weight:             {}", measurement::weight());
    println!("  Volume:             {}", measurement::volume());
    println!("  Temperature:        {}", measurement::temperature());
    println!("  Metric Height:      {}", measurement::metric_height());
    println!("  Imperial Height:    {}", measurement::imperial_height());
    println!("  Metric Weight:      {}", measurement::metric_weight());
    println!("  Imperial Weight:    {}", measurement::imperial_weight());
    println!();

    // Mountain
    println!("🏔️ MOUNTAINS:");
    println!("  Name:               {}", mountain::name());
    println!("  Range:              {}", mountain::range());
    println!();

    // Colors
    println!("🎨 COLORS:");
    println!("  Color Name:         {}", color::name());
    println!("  Hex:                {}", color::hex());
    println!("  RGB:                {:?}", color::rgb());
    println!("  HSL:                {:?}", color::hsl());
    println!();

    // Greek Philosophers
    println!("🏛️ GREEK PHILOSOPHERS:");
    println!("  Name:               {}", greek_philosophers::name());
    println!("  Quote:              {}", greek_philosophers::quote());
    println!();

    // Words
    println!("🔤 WORDS:");
    println!("  Adjective:          {}", adjective::adjective());
    println!("  Verb:               {}", verb::verb());
    println!("  Verb (Past):        {}", verb::past());
    println!("  Verb (Ing Form):    {}", verb::ing_form());
    println!();
}

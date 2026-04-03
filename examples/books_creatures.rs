//! Books & Creatures Example
//! Demonstrates Book and Animal generators

use faker_rust::{
    books, creature,
};

fn main() {
    println!("=== Faker-Rust Books & Creatures Examples ===\n");

    // Books
    println!("📚 BOOKS:");
    println!("  Book Title:          {}", books::book::title());
    println!("  Author:              {}", books::book::author());
    println!("  Publisher:           {}", books::book::publisher());
    println!();

    // Dune
    println!("🌌 DUNE:");
    println!("  Character:           {}", books::dune::character());
    println!("  Planet:              {}", books::dune::planet());
    println!("  Quote:               {}", books::dune::quote());
    println!("  Title:               {}", books::dune::title());
    println!();

    // Lovecraft
    println!("🐙 LOVECRAFT:");
    println!("  Creature:            {}", books::lovecraft::creature());
    println!("  Location:            {}", books::lovecraft::location());
    println!("  Book:                {}", books::lovecraft::book());
    println!();

    // Culture Series
    println!("🚀 CULTURE SERIES:");
    println!("  Book:                {}", books::culture_series::book());
    println!("  Ship:                {}", books::culture_series::ship());
    println!("  Character:           {}", books::culture_series::character());
    println!();

    // The Kingkiller Chronicle
    println!("🎵 KINGKILLER CHRONICLE:");
    println!("  Character:           {}", books::the_kingkiller_chronicle::character());
    println!("  Location:            {}", books::the_kingkiller_chronicle::location());
    println!();

    // Creatures
    println!("🐾 CREATURES:");
    println!("  Animal:              {}", creature::animal::name());
    println!("  Dog Breed:           {}", creature::dog::breed());
    println!("  Cat Breed:           {}", creature::cat::breed());
    println!("  Horse Breed:         {}", creature::horse::breed());
    println!("  Bird:                {}", creature::bird::common_name());
    println!();
}

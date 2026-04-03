//! Eccentric Data & Pop Culture Example
//! Demonstrates diverse and niche data generators available in faker-rust

use faker_rust::{
    cannabis, chuck_norris, esport, greek_philosophers, hipster, kpop, supernatural,
};

fn main() {
    println!("=== Faker-Rust Eccentric & Pop Culture Data ===\n");

    // Cannabis
    println!("🌿 CANNABIS:");
    println!("  Strain:              {}", cannabis::strain());
    println!("  Cannabinoid:         {}", cannabis::cannabinoid());
    println!("  Terpene:             {}", cannabis::terpene());
    println!("  Medical Use:         {}", cannabis::medical_use());
    println!("  Health Benefit:      {}", cannabis::health_benefit());
    println!("  Category:            {}", cannabis::category());
    println!("  Type:                {}", cannabis::type_name());
    println!("  Brand:               {}", cannabis::brand());
    println!();

    // Chuck Norris
    println!("🥋 CHUCK NORRIS:");
    println!("  Fact:                {}", chuck_norris::fact());
    println!();

    // Esports
    println!("🎮 ESPORTS:");
    println!("  Player:              {}", esport::player());
    println!("  Team:                {}", esport::team());
    println!("  Event:               {}", esport::event());
    println!("  League:              {}", esport::league());
    println!("  Game:                {}", esport::game());
    println!();

    // Greek Philosophers
    println!("🏛️ GREEK PHILOSOPHERS:");
    println!("  Name:                {}", greek_philosophers::name());
    println!("  School:              {}", greek_philosophers::school());
    println!("  Quote:               {}", greek_philosophers::quote());
    println!();

    // Hipster
    println!("👓 HIPSTER:");
    println!("  Word:                {}", hipster::word());
    println!("  Sentence:            {}", hipster::sentence());
    println!("  Paragraph:           {}", hipster::paragraph());
    println!();

    // K-Pop
    println!("🎤 K-POP:");
    println!("  Group:               {}", kpop::group());
    println!("  Song:                {}", kpop::song());
    println!("  Solo Artist:         {}", kpop::solo());
    println!();

    // Supernatural
    println!("👻 SUPERNATURAL:");
    println!("  Character:           {}", supernatural::character());
    println!("  Creature:            {}", supernatural::creature());
    println!("  Weapon:              {}", supernatural::weapon());
    println!("  Location:            {}", supernatural::location());
    println!();
}

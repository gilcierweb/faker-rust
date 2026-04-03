//! Sports & Events Example
//! Demonstrates sports, world cup, and event-related generators

use faker_rust::{
    sports, world_cup, esport,
};

fn main() {
    println!("=== Faker-Rust Sports & Events Examples ===\n");

    // Football
    println!("⚽ FOOTBALL (SOCCER):");
    println!("  Team:               {}", sports::football::team());
    println!("  Player:             {}", sports::football::player());
    println!("  Competition:        {}", sports::football::competition());
    println!("  Position:           {}", sports::football::position());
    println!();

    // Basketball
    println!("🏀 BASKETBALL:");
    println!("  Team:               {}", sports::basketball::team());
    println!("  Player:             {}", sports::basketball::player());
    println!("  Position:           {}", sports::basketball::position());
    println!();

    // Baseball
    println!("⚾ BASEBALL:");
    println!("  Team:               {}", sports::baseball::team());
    println!("  Player:             {}", sports::baseball::player());
    println!();

    // Volleyball
    println!("🏐 VOLLEYBALL:");
    println!("  Team:               {}", sports::volleyball::team());
    println!("  Player:             {}", sports::volleyball::player());
    println!();

    // World Cup
    println!("🏆 WORLD CUP:");
    println!("  Year:               {}", world_cup::year());
    println!("  Host:               {}", world_cup::host());
    println!("  Winner:             {}", world_cup::winner());
    println!("  Stadium:            {}", world_cup::stadium());
    println!("  Team:               {}", world_cup::team());
    println!();

    // Esports
    println!("🎮 ESPORTS:");
    println!("  Player:             {}", esport::player());
    println!("  Team:               {}", esport::team());
    println!("  League:             {}", esport::league());
    println!("  Game:               {}", esport::game());
    println!("  Event:              {}", esport::event());
    println!();
}

//! Sports & Blockchain Example
//! Demonstrates Sports and Cryptocurrency generators

use faker_rust::{
    blockchain, sports,
};

fn main() {
    println!("=== Faker-Rust Sports & Blockchain Examples ===\n");

    // Sports
    println!("⚽ SPORTS:");
    println!("  Sport Name:          {}", sports::sport::name());
    println!("  Summer Olympics:     {}", sports::sport::summer_olympics());
    println!("  Winter Olympics:     {}", sports::sport::winter_olympics());
    println!();

    // Football
    println!("🏈 FOOTBALL (Soccer):");
    println!("  Team:                {}", sports::football::team());
    println!("  Player:              {}", sports::football::player());
    println!("  Position:            {}", sports::football::position());
    println!("  Competition:         {}", sports::football::competition());
    println!();

    // Basketball
    println!("🏀 BASKETBALL:");
    println!("  Team:                {}", sports::basketball::team());
    println!("  Player:              {}", sports::basketball::player());
    println!("  Position:            {}", sports::basketball::position());
    println!("  Coach:               {}", sports::basketball::coach());
    println!();

    // Baseball
    println!("⚾ BASEBALL:");
    println!("  Team:                {}", sports::baseball::team());
    println!("  Player:              {}", sports::baseball::player());
    println!("  Position:            {}", sports::baseball::position());
    println!("  Stadium:             {}", sports::baseball::stadium());
    println!();

    // Volleyball
    println!("🏐 VOLLEYBALL:");
    println!("  Team:                {}", sports::volleyball::team());
    println!("  Player:              {}", sports::volleyball::player());
    println!("  Position:            {}", sports::volleyball::position());
    println!();

    // World Cup
    println!("🏆 WORLD CUP:");
    println!("  Year:                {}", sports::world_cup::year());
    println!("  Host:                {}", sports::world_cup::host());
    println!("  Winner:              {}", sports::world_cup::winner());
    println!("  Stadium:             {}", sports::world_cup::stadium());
    println!();

    // Blockchain
    println!("⛓️ BLOCKCHAIN:");
    println!("  Bitcoin Address:     {}", blockchain::bitcoin::address());
    println!("  Ethereum Address:    {}", blockchain::ethereum::address());
    println!();

    // Tezos
    println!("🌮 TEZOS:");
    println!("  Account:             {}", blockchain::tezos::account());
    println!("  Contract:            {}", blockchain::tezos::contract());
    println!("  Operation:           {}", blockchain::tezos::operation());
    println!("  Block:               {}", blockchain::tezos::block());
    println!("  Signature:           {}", blockchain::tezos::signature());
    println!();

    // Aeternity
    println!("🔮 AETERNITY:");
    println!("  Address:             {}", blockchain::aeternity::address());
    println!("  Transaction:         {}", blockchain::aeternity::transaction());
    println!("  Contract:            {}", blockchain::aeternity::contract());
    println!("  Oracle:              {}", blockchain::aeternity::oracle());
    println!();
}

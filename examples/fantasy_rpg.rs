//! Fantasy & RPG Example
//! Demonstrates fantasy, games, DnD, and superhero generators

use faker_rust::{
    fantasy, games, superhero, cosmere,
    dc_comics, religion, quotes,
};

fn main() {
    println!("=== Faker-Rust Fantasy & RPG Examples ===\n");

    // Fantasy
    println!("🏰 FANTASY WORLD:");
    println!("  Tolkien Race:       {}", fantasy::tolkien_race());
    println!("  Creature:           {}", fantasy::tolkien_race());
    println!();

    // DnD
    println!("🎲 DUNGEONS & DRAGONS:");
    println!("  Class:              {}", games::dnd::klass());
    println!("  Race:               {}", games::dnd::race());
    println!("  Background:         {}", games::dnd::background());
    println!("  Alignment:          {}", games::dnd::alignment());
    println!("  Monster:            {}", games::dnd::monster());
    println!("  Melee Weapon:       {}", games::dnd::melee_weapon());
    println!("  Ranged Weapon:      {}", games::dnd::ranged_weapon());
    println!("  City:               {}", games::dnd::city());
    println!("  Language:           {}", games::dnd::language());
    println!();

    // Games - More RPGs
    println!("🎮 RPG GAMES:");
    println!("  WoW Race:           {}", games::world_of_warcraft::race());
    println!("  FFXIV Job:          {}", games::final_fantasy_xiv::job());
    println!("  Zelda Character:    {}", games::zelda::character());
    println!("  Zelda Location:     {}", games::zelda::location());
    println!("  Zelda Item:         {}", games::zelda::item());
    println!();

    // Superhero
    println!("🦸 SUPERHERO:");
    println!("  Power:              {}", superhero::power());
    println!("  Name:               {}", superhero::name());
    println!("  Prefix:             {}", superhero::prefix());
    println!("  Suffix:             {}", superhero::suffix());
    println!("  Descriptor:         {}", superhero::descriptor());
    println!();

    // DC Comics
    println!("⚡ DC COMICS:");
    println!("  Hero:               {}", dc_comics::hero());
    println!("  Title:              {}", dc_comics::title());
    println!("  Villain:            {}", dc_comics::villain());
    println!();

    // Cosmere
    println!("🌟 COSMERE:");
    println!("  Character:          {}", cosmere::character());
    println!("  Location:           {}", cosmere::location());
    println!("  Book:               {}", cosmere::book());
    println!();

    // Religion
    println!("🙏 RELIGION:");
    println!("  Name:               {}", religion::name());
    println!("  Figure:             {}", religion::figure());
    println!("  Text:               {}", religion::text());
    println!("  Practice:           {}", religion::practice());
    println!("  Holiday:            {}", religion::holiday());
    println!();

    // Quotes
    println!("💬 QUOTES:");
    println!("  Famous Last Words:  {}", quotes::famous_last_words());
    println!("  Motivational:       {}", quotes::motivational());
    println!("  Philosophical:      {}", quotes::philosophical());
    println!("  Shakespeare:        {}", quotes::shakespeare());
    println!("  Movie:              {}", quotes::movie());
    println!();
}

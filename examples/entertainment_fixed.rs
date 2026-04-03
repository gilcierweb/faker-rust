//! Entertainment Example
//! Demonstrates Movies, TV Shows, Games, and Music generators

use faker_rust::{
    games, movies, music, tv_shows,
};

fn main() {
    println!("=== Faker-Rust Entertainment Examples ===\n");

    // Movies
    println!("🎬 MOVIES:");
    println!("  Star Wars Character:    {}", movies::star_wars::character());
    println!("  Harry Potter Character: {}", movies::harry_potter::character());
    println!("  LOTR Character:         {}", movies::lord_of_the_rings::character());
    println!("  Movie Quote:            {}", movies::lebowski::quote());
    println!();

    // TV Shows
    println!("📺 TV SHOWS:");
    println!("  Game of Thrones House:  {}", tv_shows::game_of_thrones::house());
    println!("  GoT Character:          {}", tv_shows::game_of_thrones::character());
    println!("  Friends Character:      {}", tv_shows::friends::character());
    println!("  Simpsons Character:     {}", tv_shows::simpsons::character());
    println!("  Breaking Bad Character: {}", tv_shows::breaking_bad::character());
    println!("  Rick and Morty Quote:   {}", tv_shows::rick_and_morty::quote());
    println!("  The Office Quote:       {}", tv_shows::the_office::quote());
    println!("  Star Trek Character:    {}", tv_shows::star_trek::character());
    println!();

    // Games
    println!("🎮 GAMES:");
    println!("  Pokemon:                {}", games::pokemon::name());
    println!("  Pokemon Location:       {}", games::pokemon::location());
    println!("  Pokemon Move:           {}", games::pokemon::move_name());
    println!("  Zelda Character:        {}", games::zelda::character());
    println!("  Mario Character:        {}", games::super_mario::character());
    println!("  Minecraft Item:         {}", games::minecraft::item());
    println!("  DnD Race:               {}", games::dnd::race());
    println!("  DnD Klass:              {}", games::dnd::klass());
    println!("  WoW Race:               {}", games::world_of_warcraft::race());
    println!("  Street Fighter:         {}", games::street_fighter::character());
    println!("  Final Fantasy XIV Job:  {}", games::final_fantasy_xiv::job());
    println!();

    // Music
    println!("🎵 MUSIC:");
    println!("  Rock Band:              {}", music::rock_band::name());
    println!("  Opera:                  {}", music::opera::name());
    println!("  Prince Song:            {}", music::prince::song());
    println!("  Pearl Jam Song:         {}", music::pearl_jam::song());
    println!("  Bossa Nova Artist:      {}", music::bossa_nova::artist());
    println!();
}

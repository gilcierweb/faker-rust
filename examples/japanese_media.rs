//! Japanese Media Example
//! Demonstrates Anime and Manga generators

use faker::japanese_media;

fn main() {
    println!("=== Faker-Rust Japanese Media Examples ===\n");

    // One Piece
    println!("☠️ ONE PIECE:");
    println!("  Character:      {}", japanese_media::one_piece::character());
    println!("  Devil Fruit:    {}", japanese_media::one_piece::devil_fruit());
    println!("  Location:       {}", japanese_media::one_piece::location());
    println!("  Quote:          {}", japanese_media::one_piece::quote());
    println!();

    // Naruto
    println!("🥷 NARUTO:");
    println!("  Character:      {}", japanese_media::naruto::character());
    println!("  Village:        {}", japanese_media::naruto::village());
    println!("  Jutsu:          {}", japanese_media::naruto::jutsu());
    println!("  Eye Technique:  {}", japanese_media::naruto::eye());
    println!();

    // Dragon Ball
    println!("🐉 DRAGON BALL:");
    println!("  Character:      {}", japanese_media::dragon_ball::character());
    println!("  Race:           {}", japanese_media::dragon_ball::race());
    println!("  Planet:         {}", japanese_media::dragon_ball::planet());
    println!("  Transformation: {}", japanese_media::dragon_ball::transformation());
    println!();

    // Studio Ghibli
    println!("🏰 STUDIO GHIBLI:");
    println!("  Movie:          {}", japanese_media::studio_ghibli::movie());
    println!("  Character:      {}", japanese_media::studio_ghibli::character());
    println!("  Quote:          {}", japanese_media::studio_ghibli::quote());
    println!();

    // Sword Art Online
    println!("⚔️ SWORD ART ONLINE:");
    println!("  Character:      {}", japanese_media::sword_art_online::character());
    println!("  Game:           {}", japanese_media::sword_art_online::game());
    println!("  Location:       {}", japanese_media::sword_art_online::location());
    println!();

    // Detective Conan
    println!("🕵️ DETECTIVE CONAN:");
    println!("  Character:      {}", japanese_media::conan::character());
    println!("  Gadget:         {}", japanese_media::conan::gadget());
    println!();

    // Cowboy Bebop
    println!("🚀 COWBOY BEBOP:");
    println!("  Character:      {}", japanese_media::cowboy_bebop::character());
    println!("  Quote:          {}", japanese_media::cowboy_bebop::quote());
    println!();

    // Doraemon
    println!("🔔 DORAEMON:");
    println!("  Character:      {}", japanese_media::doraemon::character());
    println!("  Gadget:         {}", japanese_media::doraemon::gadget());
    println!();

    // Fullmetal Alchemist
    println!("⚗️ FULLMETAL ALCHEMIST:");
    println!("  Character:      {}", japanese_media::fullmetal_alchemist::character());
    println!("  Ability:        {}", japanese_media::fullmetal_alchemist::ability());
    println!();

    // Kamen Rider
    println!("🏍️ KAMEN RIDER:");
    println!("  Series:         {}", japanese_media::kamen_rider::series());
    println!("  Character:      {}", japanese_media::kamen_rider::character());
    println!("  Device:         {}", japanese_media::kamen_rider::device());
    println!();
}

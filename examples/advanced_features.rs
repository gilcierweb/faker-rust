//! Advanced Features Example
//! Demonstrates seeded generation and other advanced features

use faker::{
    address, company, internet, name, Faker,
};

fn main() {
    println!("=== Faker-Rust Advanced Features Examples ===\n");

    // Demonstrate deterministic generation with seed
    println!("🎲 DETERMINISTIC GENERATION (Seeded):");
    println!("Setting seed to 12345...\n");
    
    Faker::set_seed(12345);
    let name1 = name::name();
    let email1 = internet::email(None, None, None);
    let city1 = address::city();
    
    println!("First run:");
    println!("  Name:   {}", name1);
    println!("  Email:  {}", email1);
    println!("  City:   {}", city1);
    println!();

    // Reset with same seed - should produce same results
    Faker::set_seed(12345);
    let name2 = name::name();
    let email2 = internet::email(None, None, None);
    let city2 = address::city();
    
    println!("Second run (same seed):");
    println!("  Name:   {}", name2);
    println!("  Email:  {}", email2);
    println!("  City:   {}", city2);
    println!();

    // Verify they're the same
    if name1 == name2 && email1 == email2 && city1 == city2 {
        println!("✅ SUCCESS: Same seed produces identical results!");
    } else {
        println!("❌ ERROR: Results differ despite same seed");
    }
    println!();

    // Different seed produces different results
    println!("Different seed (54321):");
    Faker::set_seed(54321);
    println!("  Name:   {}", name::name());
    println!("  Email:  {}", internet::email(None, None, None));
    println!("  City:   {}", address::city());
    println!();

    // Batch generation
    println!("📊 BATCH GENERATION:");
    println!("Generating 5 random companies:");
    for i in 1..=5 {
        println!("  {}. {} - {}", i, company::name(), company::catch_phrase());
    }
    println!();

    // Generate fake user profiles
    println!("👤 FAKE USER PROFILES:");
    for i in 1..=3 {
        println!("User {}:", i);
        println!("  Name:     {}", name::name());
        println!("  Email:    {}", internet::email(None, None, None));
        println!("  Username: {}", internet::username(None));
        println!("  Address:  {}", address::street_address());
        println!("  Company:  {}", company::name());
        println!();
    }
}

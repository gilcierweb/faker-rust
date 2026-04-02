//! Basic Usage Example
//! Demonstrates core generators: Name, Address, Internet, Phone, Company

use faker::{
    address, company, internet, name, phone_number,
    Faker,
};

fn main() {
    println!("=== Faker-Rust Basic Usage Examples ===\n");

    // Name generators
    println!("👤 NAMES:");
    println!("  Full Name:      {}", name::name());
    println!("  First Name:     {}", name::first_name());
    println!("  Last Name:      {}", name::last_name());
    println!("  Name with Middle: {}", name::name_with_middle());
    println!("  Prefix:         {}", name::prefix());
    println!("  Suffix:         {}", name::suffix());
    println!("  Initials (3):   {}", name::initials(3));
    println!();

    // Address generators
    println!("📍 ADDRESSES:");
    println!("  Street Address: {}", address::street_address());
    println!("  Street Name:    {}", address::street_name());
    println!("  City:           {}", address::city());
    println!("  ZIP Code:       {}", address::zip_code());
    println!("  Country:        {}", address::country());
    println!("  Secondary:      {}", address::secondary_address());
    println!();

    // Internet generators
    println!("🌐 INTERNET:");
    println!("  Email:          {}", internet::email(None, None, None));
    println!("  Domain:         {}", internet::domain_name(false, None));
    println!("  URL:            {}", internet::url(None, None, None));
    println!("  Username:       {}", internet::username(None));
    println!("  Password:       {}", internet::password(12, 20, true, true));
    println!();

    // Phone numbers
    println!("📞 PHONE NUMBERS:");
    println!("  Phone:          {}", phone_number::phone_number());
    println!("  Cell Phone:     {}", phone_number::cell_phone());
    println!();

    // Company generators
    println!("🏢 COMPANIES:");
    println!("  Company Name:   {}", company::name());
    println!("  Industry:       {}", company::industry());
    println!("  Catch Phrase:   {}", company::catch_phrase());
    println!("  BS:             {}", company::bs());
    println!();

    // Seeded generation for reproducibility
    println!("🎲 SEEDED GENERATION (Deterministic):");
    Faker::set_seed(12345);
    println!("  Seeded Name 1:  {}", name::name());
    Faker::set_seed(12345);
    println!("  Seeded Name 2:  {}", name::name());
    println!("  (Same seed = Same output)");
}

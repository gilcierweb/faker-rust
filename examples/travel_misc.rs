//! Travel & Miscellaneous Example
//! Demonstrates Travel and other utility generators

use faker_rust::{
    bank, barcode, beer, blood, boolean, coffee, color, currency, date, file,
    food, gender, hacker, job, lorem, military, number, science, space,
    time, travel, university, vehicle,
};

fn main() {
    println!("=== Faker-Rust Travel & Miscellaneous Examples ===\n");

    // Travel
    println!("✈️ TRAVEL:");
    println!("  Airport Name:        {}", travel::airport::name());
    println!("  IATA Code:           {}", travel::airport::iata_code());
    println!("  ICAO Code:           {}", travel::airport::icao_code());
    println!("  Airport City:        {}", travel::airport::city());
    println!();

    println!("🚂 TRAIN STATIONS:");
    println!("  Station Name:        {}", travel::train_station::name());
    println!("  Station City:        {}", travel::train_station::city());
    println!("  Railway Line:        {}", travel::train_station::line());
    println!();

    // Banking & Finance
    println!("💰 BANKING:");
    println!("  Bank Name:           {}", bank::name());
    println!("  Account Number:      {}", bank::account_number());
    println!("  IBAN:                {}", bank::iban());
    println!("  SWIFT/BIC:           {}", bank::swift_bic());
    println!();

    println!("💱 CURRENCY:");
    println!("  Currency Code:       {}", currency::code());
    println!("  Currency Name:       {}", currency::name());
    println!("  Currency Symbol:     {}", currency::symbol());
    println!();

    // Codes & Numbers
    println!("📊 CODES & NUMBERS:");
    println!("  EAN-13:              {}", barcode::ean_13());
    println!("  UPC-A:               {}", barcode::upc_a());
    println!("  ISBN:                {}", barcode::isbn());
    println!("  Number (1-100):      {}", number::number_range(1, 100));
    println!("  Decimal:             {}", number::decimal(2, 2));
    println!();

    // Personal attributes
    println!("🧬 PERSONAL ATTRIBUTES:");
    println!("  Blood Group:         {}", blood::group());
    println!("  Gender:              {}", gender::gender());
    println!("  Boolean:             {}", boolean::boolean());
    println!();

    // Food & Drink
    println!("🍔 FOOD & DRINK:");
    println!("  Food Dish:           {}", food::dish());
    println!("  Ingredient:          {}", food::ingredient());
    println!("  Beer Style:          {}", beer::style());
    println!("  Beer Name:           {}", beer::name());
    println!("  Coffee Blend:        {}", coffee::blend_name());
    println!("  Coffee Country:      {}", coffee::country());
    println!();

    // Science & Space
    println!("🔬 SCIENCE & SPACE:");
    println!("  Element:             {}", science::element());
    println!("  Planet:              {}", space::planet());
    println!("  Moon:                {}", space::moon());
    println!("  Galaxy:              {}", space::galaxy());
    println!();

    // Colors
    println!("🎨 COLORS:");
    println!("  Color Name:          {}", color::name());
    println!("  Hex Color:           {}", color::hex());
    println!("  RGB:                 {:?}", color::rgb());
    println!();

    // Lorem Ipsum
    println!("📝 LOREM IPSUM:");
    println!("  Word:                {}", lorem::word());
    println!("  Words (5):           {}", lorem::words(5));
    println!("  Sentence:            {}", lorem::sentence(None));
    println!();

    // Education & Work
    println!("🎓 EDUCATION & WORK:");
    println!("  University:          {}", university::name());
    println!("  Job Title:           {}", job::title());
    println!("  Job Field:           {}", job::field());
    println!();

    // Military & Vehicles
    println!("🪖 MILITARY & VEHICLES:");
    println!("  Army Rank:           {}", military::army_rank());
    println!("  Marines Rank:        {}", military::marines_rank());
    println!("  Vehicle Make:        {}", vehicle::make());
    println!("  Vehicle Type:        {}", vehicle::car_type());
    println!();

    // Files & Dates
    println!("📁 FILES & DATES:");
    println!("  File Extension:      {}", file::extension());
    println!("  MIME Type:           {}", file::mime_type());
    println!("  Date Backward:       {}", date::backward(None, None, None));
    println!("  Date Forward:        {}", date::forward(None, None, None));
    println!();

    // Time & Hacker
    println!("⏰ TIME & HACKER:");
    println!("  Time Forward (1):    {}", time::forward(1));
    println!("  Hacker Phrase:       {}", hacker::phrase());
    println!();
}

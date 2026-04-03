//! Business & Finance Example
//! Demonstrates business, finance, banking, and crypto generators

use faker_rust::{
    bank, blockchain, business, commerce, company, currency, finance,
    invoice, industry_segments, stripe, subscription,
};

fn main() {
    println!("=== Faker-Rust Business & Finance Examples ===\n");

    // Banking
    println!("🏦 BANKING:");
    println!("  Bank Name:          {}", bank::name());
    println!("  SWIFT/BIC:          {}", bank::swift_bic());
    println!("  IBAN:               {}", bank::iban());
    println!("  Account Number:     {}", bank::account_number());
    println!();

    // Credit Cards
    println!("💳 CREDIT CARDS:");
    println!("  Card Number:        {}", business::credit_card_number());
    println!("  Card Type:          {}", business::credit_card_type());
    println!("  Expiry Date:        {}", business::credit_card_expiry_date());
    println!();

    // Company
    println!("🏢 COMPANIES:");
    println!("  Company Name:       {}", company::name());
    println!("  Industry:           {}", company::industry());
    println!("  Suffix:             {}", company::suffix());
    println!("  Catch Phrase:       {}", company::catch_phrase());
    println!("  BS:                 {}", company::bs());
    println!();

    // Industry Segments
    println!("📊 INDUSTRY SEGMENTS:");
    println!("  Industry:           {}", industry_segments::industry());
    println!("  Sector:             {}", industry_segments::sector());
    println!("  Subsector:          {}", industry_segments::subsector());
    println!();

    // Commerce
    println!("🛒 COMMERCE:");
    println!("  Department:         {}", commerce::department());
    println!("  Product Name:       {}", commerce::product_name());
    println!("  Price:              {}", commerce::price(None, None));
    println!("  Promotion Code:     {}", commerce::promotion_code());
    println!();

    // Currency
    println!("💰 CURRENCY:");
    println!("  Currency Name:      {}", currency::name());
    println!("  Currency Code:      {}", currency::code());
    println!("  Currency Symbol:    {}", currency::symbol());
    println!();

    // Invoice
    println!("📄 INVOICE:");
    println!("  Reference:          {}", invoice::reference());
    println!("  Amount:             {}", invoice::amount());
    println!("  Line Item:          {}", invoice::line_item());
    println!();

    // Blockchain & Crypto
    println!("🔗 BLOCKCHAIN:");
    println!("  Bitcoin Address:    {}", blockchain::bitcoin::address());
    println!("  Ethereum Address:   {}", blockchain::ethereum::address());
    println!("  Tezos Account:      {}", blockchain::tezos::account());
    println!();

    // Finance
    println!("� FINANCE:");
    println!("  Stock Ticker:       {}", finance::stock_ticker());
    println!("  Market Index:       {}", finance::market_index());
    println!("  Currency Pair:      {}", finance::currency_pair());
    println!();

    // Stripe
    println!("� STRIPE PAYMENTS:");
    println!("  Plan:               {}", stripe::plan());
    println!("  Product:            {}", stripe::product());
    println!("  Subscription Status: {}", stripe::subscription_status());
    println!();

    // Subscription
    println!("� SUBSCRIPTION:");
    println!("  Plan:               {}", subscription::plan());
    println!("  Status:             {}", subscription::status());
    println!("  Payment Method:     {}", subscription::payment_method());
    println!();
}

//! Technology & Programming Example
//! Demonstrates technology, internet, programming, and device generators

use faker_rust::{
    app, code, computer, device, electrical_components, hacker, html,
    internet, internet_http, programming_language, source, string, types,
    drone, lorem_flickr, placeholdit,
};

fn main() {
    println!("=== Faker-Rust Technology & Programming Examples ===\n");

    // App
    println!("📱 APPLICATIONS:");
    println!("  App Name:           {}", app::name());
    println!("  App Author:         {}", app::author());
    println!("  App Version:        {}", app::version());
    println!();

    // Internet
    println!("🌐 INTERNET:");
    println!("  Email:              {}", internet::email(None, None, None));
    println!("  Domain:             {}", internet::domain_name(false, None));
    println!("  URL:                {}", internet::url(None, None, None));
    println!("  Username:           {}", internet::username(None));
    println!("  Password:           {}", internet::password(12, 20, true, true));
    println!("  IPv4:               {}", internet::ip_v4());
    println!("  IPv6:               {}", internet::ip_v6());
    println!("  MAC Address:        {}", internet::mac_address(None));
    println!("  User Agent:         {}", internet::user_agent(None));
    println!();

    // Internet HTTP
    println!("📡 HTTP:");
    println!("  HTTP Method:        {}", internet_http::method());
    println!("  Status Code:        {}", internet_http::status_code());
    println!("  Content Type:       {}", internet_http::content_type());
    println!("  Header:             {}", internet_http::header());
    println!();

    // Code
    println!("💻 CODE:");
    println!("  ISBN:               {}", code::isbn());
    println!("  EAN:                {}", code::ean());
    println!("  ASIN:               {}", code::asin());
    println!();

    // Computer
    println!("🖥️ COMPUTER:");
    println!("  Platform:           {}", computer::platform());
    println!("  OS:                 {}", computer::os());
    println!();

    // Device
    println!("📟 DEVICES:");
    println!("  Model Name:         {}", device::model_name());
    println!("  Serial:             {}", device::serial());
    println!("  Manufacturer:       {}", device::manufacturer());
    println!();

    // Programming Languages
    println!("🔤 PROGRAMMING LANGUAGES:");
    println!("  Name:               {}", programming_language::name());
    println!("  Creator:            {}", programming_language::creator());
    println!("  ");
    println!("  RUBY TYPES:");
    println!("    {}", types::ruby());
    println!("  JAVASCRIPT TYPES:");
    println!("    {}", types::javascript());
    println!("  SQL TYPES:");
    println!("    {}", types::sql());
    println!("  RUST TYPES:");
    println!("    {}", types::rust());
    println!();

    // String
    println!("🔡 STRING UTILS:");
    println!("  Random (10):        {}", string::random(10));
    println!("  Uppercase (10):     {}", string::uppercase(10));
    println!("  Lowercase (10):     {}", string::lowercase(10));
    println!("  Alphanumeric (10):  {}", string::alphanumeric(10));
    println!();

    // Hacker
    println!("👨‍💻 HACKER:");
    println!("  Abbreviation:       {}", hacker::abbreviation());
    println!("  Adjective:          {}", hacker::adjective());
    println!("  Noun:               {}", hacker::noun());
    println!("  Verb:               {}", hacker::verb());
    println!("  Ing Verb:           {}", hacker::ingverb());
    println!("  Phrase:             {}", hacker::phrase());
    println!();

    // HTML
    println!("🌐 HTML:");
    println!("  Tag:                {}", html::tag());
    println!("  Attribute:          {}", html::attribute());
    println!();

    // Source Code
    println!("📝 SOURCE CODE:");
    println!("  Git Branch:         {}", source::git_branch());
    println!("  Commit Message:     {}", source::git_commit_message());
    println!("  Version:            {}", source::version());
    println!("  Filename:           {}", source::filename());
    println!("  Directory:          {}", source::directory());
    println!();

    // Electrical Components
    println!("⚡ ELECTRICAL COMPONENTS:");
    println!("  Component:          {}", electrical_components::component());
    println!("  Active:             {}", electrical_components::active());
    println!("  Passive:            {}", electrical_components::passive());
    println!();

    // Drone
    println!("🚁 DRONE:");
    println!("  Name:               {}", drone::name());
    println!();

    // Image Placeholders
    println!("🖼️ IMAGE PLACEHOLDERS:");
    println!("  Lorem Flickr:       {}", lorem_flickr::image());
    println!("  Placeholdit:        {}", placeholdit::image());
    println!();
}

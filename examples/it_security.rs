//! IT Security & Auth Example
//! Demonstrates vulnerability identifiers, auth tokens, crypto, and hacking data

use faker_rust::{crypto, hacker, internet, omniauth, vulnerability_identifier, slack_emoji};

fn main() {
    println!("=== Faker-Rust IT Security & Auth Examples ===\n");

    // Vulnerability Identifier
    println!("🔒 VULNERABILITIES (CVE/CWE):");
    println!("  CVE:                 {}", vulnerability_identifier::cve());
    println!("  CWE:                 {}", vulnerability_identifier::cwe());
    println!("  GHSA:                {}", vulnerability_identifier::ghsa());
    println!();

    // OmniAuth / Auth profiles
    println!("🔑 OMNIAUTH (SSO profiles):");
    println!("  Provider:            {}", omniauth::provider());
    println!("  UID:                 {}", omniauth::uid());
    println!("  Name:                {}", omniauth::name());
    println!("  Directory:           {}", omniauth::directory());
    println!();

    // Cryptography / Hashes
    println!("🔐 CRYPTOCURRENCY:");
    println!("  Coin:                {}", crypto::coin());
    println!("  Name:                {}", crypto::name());
    println!("  Symbol:              {}", crypto::symbol());
    println!();

    // Hackers
    println!("👨‍💻 HACKER JARGON:");
    println!("  Abbreviation:        {}", hacker::abbreviation());
    println!("  Adjective:           {}", hacker::adjective());
    println!("  Noun:                {}", hacker::noun());
    println!("  Verb:                {}", hacker::verb());
    println!("  Phrase:              {}", hacker::phrase());
    println!();

    // Internet (User Agent, Password)
    println!("🌐 INTERNET SECURITY:");
    println!("  Password (strong):   {}", internet::password(16, 24, true, true));
    println!("  MAC Address:         {}", internet::mac_address(None));
    println!("  IPv4:                {}", internet::ip_v4());
    println!("  IPv6:                {}", internet::ip_v6());
    println!();
    
    // Slack Emoji
    println!("💬 INTERNET CHAT:");
    println!("  Slack Emoji:         {}", slack_emoji::emoji());
    println!("  Custom Emoji:        {}", slack_emoji::custom_emoji());
    println!("  People Emoji:        {}", slack_emoji::people());
    println!();
}

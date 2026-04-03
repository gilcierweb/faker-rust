//! Social Media & Web Example
//! Demonstrates X (Twitter), Slack Emoji, Lorem, and other web-related generators

use faker_rust::{
    x, slack_emoji, lorem, lorem_flickr, placeholdit, omniauth,
    internet_http, marketing,
};

fn main() {
    println!("=== Faker-Rust Social Media & Web Examples ===\n");

    // X (Twitter)
    println!("🐦 X (TWITTER):");
    println!("  Screen Name:        {}", x::screen_name());
    println!("  Hashtag:            {}", x::hashtag());
    println!("  Tweet:              {}", x::tweet());
    println!();

    // Slack Emoji
    println!("💬 SLACK EMOJI:");
    println!("  Emoji:              :{}:", slack_emoji::emoji());
    println!("  Custom Emoji:       :{}:", slack_emoji::custom_emoji());
    println!("  People:             :{}:", slack_emoji::people());
    println!();

    // Lorem Ipsum
    println!("📄 LOREM IPSUM:");
    println!("  Word:               {}", lorem::word());
    println!("  Words (5):          {}", lorem::words(5));
    println!("  Sentence:           {}", lorem::sentence(None));
    println!("  Paragraph:          {}", lorem::paragraph(None));
    println!();

    // Marketing
    println!("📢 MARKETING:");
    println!("  Buzzword:           {}", marketing::buzzword());
    println!();

    // Image Placeholders
    println!("🖼️ IMAGE PLACEHOLDERS:");
    println!("  Lorem Flickr:");
    println!("    {}", lorem_flickr::image());
    println!("  Lorem Flickr Grayscale:");
    println!("    {}", lorem_flickr::grayscale());
    println!("  Lorem Flickr Sized:");
    println!("    {}", lorem_flickr::sized(300, 200));
    println!();
    println!("  Placeholdit:");
    println!("    {}", placeholdit::image());
    println!();

    // OmniAuth
    println!("🔐 OMNIAUTH:");
    println!("  Provider:           {}", omniauth::provider());
    println!("  UID:                {}", omniauth::uid());
    println!("  Name:               {}", omniauth::name());
    println!();

    // HTTP
    println!("🌐 HTTP:");
    println!("  Method:             {}", internet_http::method());
    println!("  Status Code:        {}", internet_http::status_code());
    println!("  Content Type:       {}", internet_http::content_type());
    println!("  Header:             {}", internet_http::header());
    println!();
}

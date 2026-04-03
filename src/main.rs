fn main() {
    println!("=== Debug: Testing user_agent vendor lookup ===");

    // Test user_agent with specific vendors
    let ua_chrome = faker_rust::internet::user_agent(Some("chrome"));
    let ua_firefox = faker_rust::internet::user_agent(Some("firefox"));
    let ua_safari = faker_rust::internet::user_agent(Some("safari"));

    println!("Chrome: {}", ua_chrome);
    println!("Firefox: {}", ua_firefox);
    println!("Safari: {}", ua_safari);

    // Show they're different
    if ua_chrome.contains("Chrome") && ua_firefox.contains("Firefox") {
        println!("SUCCESS: Different vendors return different user agents");
    } else {
        println!("ISSUE: All returning same type");
    }

    println!("\n=== Testing bot_user_agent ===");
    let bot_google = faker_rust::internet::bot_user_agent(Some("googlebot"));
    let bot_bing = faker_rust::internet::bot_user_agent(Some("bingbot"));

    println!("Googlebot: {}", bot_google);
    println!("Bingbot: {}", bot_bing);
}

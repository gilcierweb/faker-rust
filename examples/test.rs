fn main() {
    println!("=== Testing faker-rust ===");
    println!();

    println!("Name: {}", faker::name::name());
    println!("Email: {}", faker::internet::email(None, None, None));
    println!("City: {}", faker::address::city());
    println!("Company: {}", faker::company::name());
    println!("Phone: {}", faker::phone_number::phone_number());
    println!("Job title: {}", faker::job::title());
    println!("University: {}", faker::university::name());
    println!("Bank: {}", faker::bank::name());
    println!("Gender: {}", faker::gender::gender());
    println!(
        "Commerce: {} - {}",
        faker::commerce::department(),
        faker::commerce::product_name()
    );

    // New modules
    println!("\n--- New Modules ---");
    println!("Book title: {}", faker::books::book::title());
    println!("Book author: {}", faker::books::book::author());
    println!("Movie: {}", faker::movies::movie::title());
    println!("Music band: {}", faker::music::music::band());
    println!("Music album: {}", faker::music::music::album());
    println!("Music genre: {}", faker::music::music::genre());
    println!("Game: {}", faker::games::game::title());
    println!("Device: {}", faker::device::model_name());
    println!("Vehicle: {}", faker::vehicle::manufacturer());
    println!("Color: {}", faker::color::name());
    println!("Boolean: {}", faker::boolean::boolean());
    println!("App name: {}", faker::app::name());
    println!("Crypto coin: {}", faker::crypto::coin());
    println!("File extension: {}", faker::file::extension());
    println!("Food dish: {}", faker::food::dish());
    println!("Science element: {}", faker::science::element());
    println!("Space planet: {}", faker::space::planet());
    println!("Avatar character: {}", faker::avatar::character());
    println!("Beer name: {}", faker::beer::name());
    println!("Coffee blend: {}", faker::coffee::blend_name());
    println!("Chuck Norris fact: {}", faker::chuck_norris::fact());
    println!("Code ISBN: {}", faker::code::isbn());
    println!("Currency name: {}", faker::currency::name());
    println!("Time (now): {}", faker::time::backward(1));

    println!("\nAll tests passed!");
}

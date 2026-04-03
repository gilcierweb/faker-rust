fn main() {
    println!("=== Testing faker-rust ===");
    println!();

    println!("Name: {}", faker_rust::name::name());
    println!("Email: {}", faker_rust::internet::email(None, None, None));
    println!("City: {}", faker_rust::address::city());
    println!("Company: {}", faker_rust::company::name());
    println!("Phone: {}", faker_rust::phone_number::phone_number());
    println!("Job title: {}", faker_rust::job::title());
    println!("University: {}", faker_rust::university::name());
    println!("Bank: {}", faker_rust::bank::name());
    println!("Gender: {}", faker_rust::gender::gender());
    println!(
        "Commerce: {} - {}",
        faker_rust::commerce::department(),
        faker_rust::commerce::product_name()
    );

    // New modules
    println!("\n--- New Modules ---");
    println!("Book title: {}", faker_rust::books::book::title());
    println!("Book author: {}", faker_rust::books::book::author());
    println!("Movie: {}", faker_rust::movies::movie::title());
    println!("Music band: {}", faker_rust::music::music::band());
    println!("Music album: {}", faker_rust::music::music::album());
    println!("Music genre: {}", faker_rust::music::music::genre());
    println!("Game: {}", faker_rust::games::game::title());
    println!("Device: {}", faker_rust::device::model_name());
    println!("Vehicle: {}", faker_rust::vehicle::manufacturer());
    println!("Color: {}", faker_rust::color::name());
    println!("Boolean: {}", faker_rust::boolean::boolean());
    println!("App name: {}", faker_rust::app::name());
    println!("Crypto coin: {}", faker_rust::crypto::coin());
    println!("File extension: {}", faker_rust::file::extension());
    println!("Food dish: {}", faker_rust::food::dish());
    println!("Science element: {}", faker_rust::science::element());
    println!("Space planet: {}", faker_rust::space::planet());
    println!("Avatar character: {}", faker_rust::avatar::character());
    println!("Beer name: {}", faker_rust::beer::name());
    println!("Coffee blend: {}", faker_rust::coffee::blend_name());
    println!("Chuck Norris fact: {}", faker_rust::chuck_norris::fact());
    println!("Code ISBN: {}", faker_rust::code::isbn());
    println!("Currency name: {}", faker_rust::currency::name());
    println!("Time (now): {}", faker_rust::time::backward(1));

    println!("\nAll tests passed!");
}

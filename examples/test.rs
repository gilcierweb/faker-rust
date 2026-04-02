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

    println!("\nAll tests passed!");
}

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

    println!("\nAll tests passed!");
}

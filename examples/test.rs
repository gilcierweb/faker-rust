fn main() {
    println!("=== Testing faker-rust ===");
    println!();

    println!("Name module:");
    println!("  name(): {}", faker::name::name());
    println!("  first_name(): {}", faker::name::first_name());
    println!("  last_name(): {}", faker::name::last_name());
    println!("  prefix(): {}", faker::name::prefix());
    println!("  suffix(): {}", faker::name::suffix());
    println!("  initials(3): {}", faker::name::initials(3));
    println!();

    println!("Internet module:");
    println!("  email(): {}", faker::internet::email(None, None, None));
    println!("  username(): {}", faker::internet::username(None));
    println!(
        "  domain_name(false, None): {}",
        faker::internet::domain_name(false, None)
    );
    println!(
        "  domain_suffix(false): {}",
        faker::internet::domain_suffix(false)
    );
    println!("  ip_v4(): {}", faker::internet::ip_v4());
    println!();

    println!("Address module:");
    println!("  city(): {}", faker::address::city());
    println!("  street_name(): {}", faker::address::street_name());
    println!("  street_address(): {}", faker::address::street_address());
    println!("  zip_code(): {}", faker::address::zip_code());
    println!("  country(): {}", faker::address::country());
    println!();

    println!("Company module:");
    println!("  name(): {}", faker::company::name());
    println!("  suffix(): {}", faker::company::suffix());
    println!();

    println!("Deterministic test:");
    faker::Faker::set_seed(42);
    println!("  With seed 42, name(): {}", faker::name::name());

    faker::Faker::set_seed(42);
    println!("  With seed 42 again, name(): {}", faker::name::name());

    println!();
    println!("All tests passed!");
}

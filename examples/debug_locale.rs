use faker::locale;

fn main() {
    println!("=== Testing context-specific lookups ===\n");

    // What does resolve_inner call when resolving city?
    // context="address", inner="female_first_name"
    let r1 = locale::fetch_locale_with_context("female_first_name", "en", Some("address"));
    println!(
        "fetch_locale_with_context('female_first_name', 'en', Some('address')): {} items",
        r1.as_ref().map(|v| v.len()).unwrap_or(0)
    );

    // For city suffix with context=address
    let r2 = locale::fetch_locale_with_context("city_suffix", "en", Some("address"));
    println!(
        "fetch_locale_with_context('city_suffix', 'en', Some('address')): {} items",
        r2.as_ref().map(|v| v.len()).unwrap_or(0)
    );

    // city_prefix with context=address
    let r3 = locale::fetch_locale_with_context("city_prefix", "en", Some("address"));
    println!(
        "fetch_locale_with_context('city_prefix', 'en', Some('address')): {} items",
        r3.as_ref().map(|v| v.len()).unwrap_or(0)
    );
}

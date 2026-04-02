//! Regional Data Example
//! Demonstrates regional data generators (Chile, South Africa, UK, US, NHS, etc.)

use faker::{
    chile_rut, driving_licence, id_number, national_health_service,
    phone_number, south_africa, address,
};

fn main() {
    println!("=== Faker-Rust Regional Data Examples ===\n");

    // Chile
    println!("🇨🇱 CHILE:");
    println!("  RUT:                {}", chile_rut::rut());
    println!();

    // South Africa
    println!("🇿🇦 SOUTH AFRICA:");
    println!("  ID Number:          {}", south_africa::id_number());
    println!("  Phone Number:       {}", south_africa::phone_number());
    println!("  Province:           {}", south_africa::province());
    println!("  License Plate:      {}", south_africa::license_plate());
    println!();

    // ID Numbers (Various Countries)
    println!("🆔 ID NUMBERS:");
    println!("  Spanish DNI:        {}", id_number::spanish());
    println!("  Spanish (Valid):    {}", id_number::valid());
    println!("  US SSN:             {}", id_number::ssn());
    println!("  Invalid ID:         {}", id_number::invalid());
    println!();

    // Driving Licences
    println!("🚗 DRIVING LICENCES:");
    println!("  UK Licence:         {}", driving_licence::uk());
    println!("  US Licence:         {}", driving_licence::usa());
    println!();

    // UK NHS
    println!("🏥 UK NATIONAL HEALTH SERVICE:");
    println!("  NHS Number:         {}", national_health_service::nhs_number());
    println!("  Practitioner:       {}", national_health_service::practitioner());
    println!("  Hospital:           {}", national_health_service::hospital());
    println!();

    // Phone Numbers (International)
    println!("📞 INTERNATIONAL PHONES:");
    println!("  US Phone:           {}", phone_number::phone_number());
    println!("  US Cell:            {}", phone_number::cell_phone());
    println!();

    // Address
    println!("� ADDRESSES:");
    println!("  City:               {}", address::city());
    println!("  Street:             {}", address::street_address());
    println!("  ZIP:                {}", address::zip_code());
    println!("  Country:            {}", address::country());
    println!("  Country Code:       {}", address::country_code());
    println!();
}

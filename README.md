# 🦀 Faker-Rust 🚀

[![Crates.io](https://img.shields.io/crates/v/faker-rust.svg)](https://crates.io/crates/faker-rust)
[![Documentation](https://docs.rs/faker-rust/badge.svg)](https://docs.rs/faker-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Faker-Rust** is a high-performance, locale-aware, and reproducible fake data generator for Rust. Strongly inspired by the legendary [Ruby Faker gem](https://github.com/faker-ruby/faker), it brings massive-scale data generation to the Rust ecosystem with a focus on speed and reliability.

---

## ✨ Features

- **🚀 Performance-First**: Built in Rust for blazing-fast data generation.
- **🌍 Locale-Aware**: Multi-language support out of the box (English, with extensible architecture for more).
- **🎲 Reproducible**: Seedable RNG for deterministic data generation—perfect for testing.
- **📦 Massive Library**: Over 80+ modules already implemented, targeting 200+ for full parity with Ruby Faker.
- **🛠️ Extensible**: Easily add your own custom providers and locales.
- **🧪 Well Tested**: 230+ unit tests ensuring reliability.

---

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
faker-rust = "0.1.0"
```

### Usage Example

```rust
use faker_rust::Faker;

fn main() {
    // Basic Usage - Name generation
    println!("Name: {}", faker_rust::name::name());
    
    // Internet data
    println!("Email: {}", faker_rust::internet::email(None, None, None));
    println!("Domain: {}", faker_rust::internet::domain_name());
    
    // Address
    println!("City: {}", faker_rust::address::city());
    println!("Street: {}", faker_rust::address::street_address());
    
    // Seeded Randomness for Reproducibility
    faker_rust::Faker::set_seed(12345);
    println!("Seeded Name: {}", faker_rust::name::name());
    
    // Entertainment - Movies
    println!("Star Wars Character: {}", faker_rust::movies::star_wars::character());
    println!("Harry Potter House: {}", faker_rust::movies::harry_potter::house());
    
    // Gaming
    println!("Pokemon: {}", faker_rust::games::pokemon::name());
    println!("Zelda Character: {}", faker_rust::games::zelda::character());
}
```

---

## 📂 Available Modules

| Category | Modules |
| :--- | :--- |
| **👤 Identity** | Name, Address, Phone, Job, Blood, Gender, University |
| **💻 Technology** | Internet, Computer, App, Code, Crypto, Device, Hacker |
| **🏢 Business** | Company, Bank, Business, Coin, Commerce, Finance, Currency |
| **🍔 Lifestyle** | Food, Beer, Coffee, Dessert, Hobby, Home, House |
| **🎬 Media** | Movies (Star Wars, Harry Potter, Lord of the Rings, Avatar, etc.), Music, Books |
| **📺 TV Shows** | Game of Thrones, Breaking Bad, Simpsons, Rick and Morty, etc. |
| **🎮 Gaming** | Pokemon, Zelda, Mario, Minecraft, DnD, WoW, LoL, 20+ more |
| **🎵 Music** | RockBand, Opera, GratefulDead, Prince, PearlJam, BossaNova |
| **📚 Books** | HarryPotter, Dune, Lovecraft, CultureSeries |
| **⚽ Sports** | Football, Basketball, Baseball, Volleyball, WorldCup |
| **🌸 Anime** | Naruto, DragonBall, SwordArtOnline, StudioGhibli |
| **� Blockchain** | Bitcoin, Ethereum |
| **✈️ Travel** | Airport, TrainStation |
| **�🛠️ Misc** | Color, Construction, Science, Space, Time, Animal |

---

## 🗺️ Implementation Status

We are actively porting the entire Ruby Faker library. Current progress:

- [x] **Phase 1-4**: Core Defaults (46+ modules)
- [x] **Phase 5**: Creature Kingdom & General Concepts ✅
- [x] **Phase 6**: Movies & TV Shows ✅
- [x] **Phase 7**: The Gaming Universe ✅
- [x] **Phase 8**: Sports, Anime, Blockchain, Travel ✅
- [ ] **Phase 9**: Final Polish & crates.io Release (In Progress)

---

## 📊 Statistics

- **233+ Tests Passing**
- **80+ Generator Modules**
- **15+ Categories**
- **Complete API Documentation**

---

## 🤝 Contributing

Contributions are welcome! If you'd like to help us reach full parity with Ruby Faker, feel free to open a PR.

## 📄 License

This project is licensed under the MIT License.

# 🦀 Faker-Rust 🚀

[![Crates.io](https://img.shields.io/crates/v/faker-rust.svg)](https://crates.io/crates/faker-rust)
[![Documentation](https://docs.rs/faker-rust/badge.svg)](https://docs.rs/faker-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Faker-Rust** is a high-performance, locale-aware, and reproducible fake data generator for Rust. Strongly inspired by the legendary [Ruby Faker gem](https://github.com/faker-ruby/faker), it brings massive-scale data generation to the Rust ecosystem with a focus on speed and reliability.

---

## ✨ Features

- **🚀 Performance-First**: Built in Rust for blazing-fast data generation.
- **🌍 Locale-Aware**: Multi-language support out of the box (English, Portuguese, Spanish, and more).
- **🎲 Reproducible**: Seedable RNG for deterministic data generation—perfect for testing.
- **📦 Massive Library**: Over 50+ modules already implemented, targeting 200+ for full paraty.
- **🛠️ Extensible**: Easily add your own custom providers and locales.

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
    // Basic Usage
    println!("Name: {}", faker_rust::name::name());
    println!("Email: {}", faker_rust::internet::email(None, None, None));
    
    // Seeded Randomness for Reproducibility
    faker_rust::set_seed(12345);
    println!("Seeded Name: {}", faker_rust::name::name());
}
```

---

## 📂 Available Modules

| Category | Modules |
| :--- | :--- |
| **👤 Identity** | Name, Address, Phone, Job, Blood, Gender |
| **💻 Technology** | Internet, Computer, App, Code, Crypto, Device |
| **🏢 Business** | Company, Bank, Business, Coin, Commerce, Finance |
| **🍔 Lifestyle** | Food, Beer, Coffee, Dessert, Hobby, Home |
| **🎬 Media** | Movies (Star Wars, Harry Potter), Music, Books |
| **🎮 Gaming** | Pokemon, Chess |
| **🛠️ Misc** | Color, Construction, Science, Space, Time |

---

## 🗺️ Roadmap to 1.0 (Full Parity)

We are actively porting the entire Ruby Faker library. Current progress:

- [x] **Batch 1-4**: Core Defaults (46+ modules)
- [ ] **Batch 5**: Creature Kingdom (Animal, Bird, Cat, Dog) & Misc. (Culture, Religion)
- [ ] **Batch 6**: TV Shows (Friends, Seinfeld, Simpsons, GoT)
- [ ] **Batch 7**: Pro Gaming (Zelda, Mario, WoW, LoL)
- [ ] **Batch 8**: Global Sports & Anime (Naruto, One Piece)
- [ ] **Batch 9**: crates.io Release & Documentation Polish

---

## 🤝 Contributing

Contributions are welcome! If you'd like to help us reach full parity with Ruby Faker, feel free to open a PR.

## 📄 License

This project is licensed under the MIT License.

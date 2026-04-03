# Faker-Rust [![Crates.io][crates-badge]][crates] [![Documentation][docs-badge]][docs] [![License: MIT][license-badge]][license]

<div align="center">
  <img src="./assets/images/logo.png" width="400"/>
  <p>Generate realistic fake data for testing, demos, and populating your database during development.</p>
  <p><em>Inspired by the legendary <a href="https://github.com/faker-ruby/faker">Ruby Faker gem</a></em></p>
</div>

[crates-badge]: https://img.shields.io/crates/v/faker-rust.svg
[crates]: https://crates.io/crates/faker-rust
[docs-badge]: https://docs.rs/faker-rust/badge.svg
[docs]: https://docs.rs/faker-rust
[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license]: https://opensource.org/licenses/MIT

## Quick links

- 📖 **[Read the documentation][docs]**
- 📢 **[See what's changed in recent versions](./CHANGELOG.md)**

## Features

- 🚀 **High Performance**: Built in Rust for blazing-fast data generation
- 🌍 **Locale-Aware**: Multi-language support with fallback mechanism
- 🎲 **Reproducible**: Seedable RNG for deterministic data generation
- 📦 **Complete Parity**: 100% API compatibility with Ruby Faker gem
- 🧪 **Well Tested**: 549+ unit tests ensuring reliability
- 🏢 **100+ Modules**: Internet, Person, Location, Finance, and much more

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
faker-rust = "0.1.0"
```

Then run `cargo build`.

## Usage

```rust
use faker_rust::name;
use faker_rust::internet;
use faker_rust::address;
use faker_rust::lorem;
use faker_rust::programming_language;

fn main() {
    // Person
    println!("{}", name::name());                      //=> "Christophe Bartell"
    
    // Internet
    println!("{}", internet::password());            //=> "Vg5mSvY1UeRg7"
    println!("{}", internet::email(None, None, None)); //=> "eliza@mann.test"
    
    // Address
    println!("{}", address::full_address());         //=> "5479 William Way, East Sonnyhaven, LA 63637"
    
    // Lorem
    println!("{}", lorem::paragraph());                //=> "Recusandae minima consequatur. Expedita sequi blanditiis. Ut fuga et."
    
    // Programming Language
    println!("{}", programming_language::name());      //=> "Rust"
    
    // Seeded Randomness for Reproducibility
    faker_rust::Faker::set_seed(12345);
    println!("Seeded: {}", name::name());             // Always returns same value
}
```

## 🖥️ Command Line Interface (CLI)

Faker-Rust now includes a powerful CLI tool for generating fake data directly from the command line:

### Installation

```bash
cargo install faker-rust
```

Or run directly with:
```bash
# Run the CLI binary
cargo run --bin faker -- name

# Run from installed binary
faker name
```

### Quick CLI Examples

```bash
# Generate a random name
faker name

# Or using cargo run
cargo run --bin faker -- name

# Generate an email
faker email

# Generate multiple values
faker -c 5 name

# Generate deterministic output (repeatable)
faker --seed 12345 name

# List all available generators
faker list
```

### CLI Demo Example

Run the interactive CLI demo example:

```bash
cargo run --example cli_demo
```

### CLI Commands

| Command | Description | Example |
|---------|-------------|---------|
| `name` | Generate names | `faker name --first` |
| `email` | Generate emails | `faker email` |
| `address` | Generate addresses | `faker address --full` |
| `company` | Generate company names | `faker company` |
| `phone` | Generate phone numbers | `faker phone --cell` |
| `internet` | Generate internet data | `faker internet --username` |
| `games` | Generate game data | `faker games --pokemon` |
| `movies` | Generate movie data | `faker movies --star-wars` |
| `tv` | Generate TV show data | `faker tv --simpsons` |

For complete CLI documentation, see [CLI.md](./docs/CLI.md).

## Generators

To see the full list, check out the [GENERATORS](./GENERATORS.md) document.

### Default Generators

| Category | Generators |
|----------|------------|
| **Person** | `name`, `first_name`, `last_name`, `job`, `gender`, `blood` |
| **Internet** | `email`, `password`, `domain`, `url`, `ip`, `user_agent` |
| **Address** | `city`, `street_address`, `zip_code`, `state`, `country` |
| **Finance** | `bank`, `credit_card`, `crypto`, `currency`, `iban`, `bic` |
| **Company** | `company_name`, `bs`, `catch_phrase`, `industry` |
| **Date & Time** | `date`, `time`, `datetime`, `timezone` |
| **Number** | `digit`, `number`, `decimal`, `hexadecimal` |
| **String** | `alphanumeric`, `alpha`, `numeric`, `hex`, `uuid` |
| **Lorem** | `word`, `sentence`, `paragraph`, `question` |
| **Color** | `hex_color`, `rgb_color`, `safe_color` |
| **Code** | `isbn`, `ean`, `barcode`, `sku` |
| **Hacker** | `abbreviation`, `adjective`, `noun`, `verb`, `ingverb` |
| **App** | `name`, `version`, `author` |
| **Crypto** | `md5`, `sha1`, `sha256` |
| **Device** | `model`, `platform`, `serial` |

### Entertainment Generators

| Category | Generators |
|----------|------------|
| **Movies** | `star_wars`, `harry_potter`, `lord_of_the_rings`, `avatar`, `hackers`, `hitchhikers_guide_to_the_galaxy` |
| **TV Shows** | `game_of_thrones`, `breaking_bad`, `simpsons`, `rick_and_morty`, `aqua_teen_hunger_force`, `archer`, `how_i_met_your_mother`, `suits`, `supernatural`, `the_expanse`, `the_it_crowd`, `the_thick_of_it`, and 30+ more |
| **Gaming** | `pokemon`, `zelda`, `mario`, `minecraft`, `dnd`, `wow`, `lol`, `esport`, `dc_comics`, `superhero`, `cosmere` |
| **Books** | `book`, `culture_series`, `dune`, `lovecraft`, `kingkiller_chronicle` |
| **Music** | `rock_band`, `opera`, `grateful_dead`, `prince`, `pearl_jam`, `bossa_nova` |

### Specialty Generators

| Category | Generators |
|----------|------------|
| **Anime** | `naruto`, `dragon_ball`, `one_piece`, `sword_art_online`, `studio_ghibli` |
| **Sports** | `football`, `basketball`, `baseball`, `volleyball`, `world_cup` |
| **Blockchain** | `bitcoin`, `ethereum`, `tezos` |
| **Travel** | `airport`, `train_station` |
| **Fantasy** | `tolkien_race`, `creature`, `location`, `weapon`, `spell` |
| **Locations** | `community`, `neighborhood`, `building_type`, `place` |
| **Quotes** | `famous_last_words`, `motivational`, `philosophical`, `shakespeare`, `movie` |
| **Religion** | `name`, `figure`, `text`, `practice`, `holiday` |

### Regional & Tech Generators

| Category | Generators |
|----------|------------|
| **Regional** | `chile_rut`, `driving_licence`, `id_number`, `south_africa`, `national_health_service` |
| **Tech** | `drone`, `electrical_components`, `html`, `programming_language`, `internet_http` |
| **Security** | `vulnerability_identifier`, `omniauth` |
| **Misc** | `slack_emoji`, `lorem_flickr`, `placeholdit`, `x` (Twitter), `source` |

## Notes

* While Faker generates data at random, returned values are not guaranteed to be unique by default.
* Values can be deterministic if you use the seeded RNG feature.
* This is the `main` branch and may contain unreleased changes. See [CHANGELOG](./CHANGELOG.md) for all versions.

## Deterministic Random

Faker supports seeding of its pseudo-random number generator to provide deterministic output:

```rust
use faker_rust::Faker;
use faker_rust::name;

fn main() {
    Faker::set_seed(42);
    println!("{}", name::name());  //=> "Aurelia Wilkinson"
    
    Faker::set_seed(42);
    println!("{}", name::name());  //=> "Aurelia Wilkinson" (same value)
}
```

## Localization

Faker supports locale-aware data generation:

```rust
// Set locale for data generation
faker_rust::locale::set_locale("en");

// The library will use locale-specific data where available,
// falling back to English defaults for missing translations
```

## Table of Contents

- [Faker-Rust](#faker-rust)
  - [Quick links](#quick-links)
  - [Features](#features)
  - [Getting Started](#getting-started)
  - [Usage](#usage)
  - [Generators](#generators)
  - [Notes](#notes)
  - [Deterministic Random](#deterministic-random)
  - [Localization](#localization)
  - [Contributing](#contributing)
  - [Inspiration](#inspiration)
  - [License](#license)

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.

## Inspiration

Faker-Rust is inspired by the legendary [Ruby Faker gem](https://github.com/faker-ruby/faker) and aims to bring the same ease of use and comprehensive coverage to the Rust ecosystem.

## License

This code is free to use under the terms of the MIT license.

[MIT License](https://github.com/faker-rust/faker-rust/blob/main/LICENSE)

## Links
https://github.com/faker-rust/faker-rust

https://github.com/faker-rust/faker-rust/issues

https://github.com/faker-rust/faker-rust/discussions

https://github.com/faker-rust/faker-rust/pulls

https://github.com/faker-rust/faker-rust/wiki

https://github.com/faker-rust/faker-rust/blob/main/SECURITY.md

https://github.com/faker-rust/faker-rust/blob/main/CONTRIBUTING.md

https://github.com/faker-rust/faker-rust/blob/main/CODE_OF_CONDUCT.md

https://gilcierweb.com.br

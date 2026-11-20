# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.1] - 2026-11-20

### Added
- **Full Ruby Faker gem v3.6.1 source parity**: all 1197 public class methods from the gem
  source are now present in Rust, including 58 previously undocumented source-only methods
  and `Quote.fortune_cookie`.
- **Complete documented API**: all 1144 gem-documented methods are implemented and mapped,
  with 0 unmapped modules.
- **Method-name aliases for gem parity**: `tome`, `postcode`, `zip`, `middle_name`,
  `door_count`, `engine_size`, `kilometrage`, `south_african_id_number`, `color` and
  `adjective` re-exported so legacy gem names resolve in Rust.
- **New parity generators**: 235 locale-backed generators, 24 Drone specification
  generators, ISMN/ISSN/composite barcodes, structured and stateful gem generators,
  algorithmic generators across 16 modules, and company, identifier and OmniAuth
  generators.
- **Configured-locale resolution**: generators now read data through
  `FakerConfig::locale()`, so setting a locale changes generated output; unknown locales
  fall back to English.
- **Function-level test coverage**: all 1466 public functions (including re-export
  aliases) are referenced by tests; added regression tests for 18 previously untested
  modules and seeded determinism.
- **Expanded examples**: module-level coverage for all 260 public modules, a smoke
  example for 60 uncovered generators, and a localization example. All 29 examples
  run successfully.
- **locale YAML load test**: a test asserts that every YAML file loads with the Rust
  parser.

### Changed
- **Rust 2024 edition, MSRV 1.89**: the minimum supported Rust version was raised to
  1.89 because `rust-i18n` 4 and `serde-saphyr` require it.
- **Dependencies**: migrated `rand` 0.8 to 0.9, replaced unmaintained `chrono` with
  `time` 0.3, replaced deprecated `serde_yaml` with `serde-saphyr`, and updated
  `rust-i18n` to 4.2.4.
- **Locale engine refactor**: removed roughly 537 dead extract/resolve functions,
  unified placeholder depth limits, unified singular/plural key bridging, and paired
  Rust fetch keys with the actual gem YAML paths.
- **Generator cleanup**: deduplicated `crypto_coin` into `crypto` re-exports, merged
  `driving_licence` into `driving_license`, and wired orphaned locale data in fantasy,
  locations and TV shows modules.
- **Documentation**: adapted all generated docs from Ruby to Rust syntax, corrected
  module paths, reserved-keyword calls (`r#type`, `r#move`), and per-module arities;
  relabeled illustrative Ruby output as `text`/`ruby` and normalized URL artifacts.
  All 1116 distinct Rust doc calls are compile-checked.
- **CLI version bumped** to 0.1.1 and installation snippets updated accordingly.

### Fixed
- Locale placeholder resolution now terminates on cycles, stops rewriting underscores
  in key extraction, resolves scalar placeholder values, and falls through to alternate
  lookup cases when a wrapper lacks a match.
- Corrected generator bugs: `numerify` allows zero in non-leading positions,
  `letterify` samples a distinct letter for each `?`, EAN/UPC check-digit weighting,
  inclusive `rand_range`/`rand_range_i64`/`rand_in_range`, `number::between` with
  negative ranges, panic-safe `mac_address` prefix parsing, structurally valid GB IBAN
  check digits, UK DVLA driving-licence format, and fixed-width fields after the
  inclusive `rand_range` change.
- `airport::name` and `iata` now honor the required `size` and `region` arguments.
- Internet HTTP generators restore base64 padding to a multiple of four.

### Security
- Patched `crossbeam-epoch` to 0.9.20 to address RUSTSEC-2026-0204.

## [0.1.0] - 2026-04-02

### Added - CLI & Examples
- **Command Line Interface (CLI)**: New `faker` binary with interactive commands
  - Commands: `name`, `email`, `address`, `company`, `phone`, `internet`, `games`, `movies`, `tv`, `list`
  - Options: `--seed` for deterministic output, `-c` for multiple values
  - Built with `clap` for argument parsing
- **CLI Documentation**: Complete CLI guide at `docs/CLI.md`
- **8 New Example Files**:
  - `cli_demo.rs` - Interactive CLI demonstration
  - `business_finance.rs` - Banking, crypto, Stripe generators
  - `tech_programming.rs` - Internet, code, device generators
  - `fantasy_rpg.rs` - DnD, superheroes, cosmere, mythology
  - `regional_data.rs` - Chile RUT, SA IDs, driving licenses
  - `social_media.rs` - X (Twitter), Slack emoji, Lorem
  - `education_science.rs` - Universities, science, space
  - `sports_events.rs` - Football, basketball, World Cup, eSports

### Added - Complete Ruby Faker Gem Parity! 🎉
- **40 new Default modules**: adjective, alphanumeric, chile_rut, cosmere, crypto_coin, dc_comics, drone, driving_licence, electrical_components, esport, html, id_number, industry_segments, internet_http, invoice, lorem_flickr, marketing, measurement, mountain, national_health_service, nato_phonetic_alphabet, omniauth, placeholdit, programming_language, restaurant, slack_emoji, source, south_africa, string, stripe, subscription, superhero, team, tea, theater, types, verb, vulnerability_identifier, world_cup, x
- **15 new TV Show modules**: aqua_teen_hunger_force, archer, dumb_and_dumber, final_space, hey_arnold, how_i_met_your_mother, michael_scott, new_girl, ru_paul, suits, supernatural, the_expanse, the_it_crowd, the_thick_of_it
- **2 new Movie modules**: hackers, hitchhikers_guide_to_the_galaxy
- **4 new categories**: fantasy, locations, quotes, religion
- **549+ tests** (up from 233)
- **100% API parity** with Ruby Faker gem

### Changed
- Updated module count: 100+ total modules across 20 categories
- Complete coverage of all Ruby Faker Default modules

## [0.1.0] - 2026-04-02

### Added
- Initial release with 80+ generator modules
- Core generators: Name, Address, Internet, Phone, Email, Company
- Entertainment: Movies (Star Wars, Harry Potter, LOTR, Avatar, etc.), TV Shows (GoT, Simpsons, Rick and Morty, etc.)
- Gaming: Pokemon, Zelda, Mario, Minecraft, DnD, WoW, League of Legends, 20+ more
- Music: RockBand, Opera, GratefulDead, Prince, PearlJam, BossaNova
- Books: HarryPotter, Dune, Lovecraft, CultureSeries
- Sports: Football, Basketball, Baseball, Volleyball, WorldCup
- Anime: Naruto, DragonBall, SwordArtOnline, StudioGhibli
- Blockchain: Bitcoin, Ethereum address generators
- Travel: Airport, TrainStation generators
- Locale support with fallback mechanism
- Seeded RNG for reproducible data generation
- Comprehensive test suite with 233+ tests

### Features
- 🚀 High-performance Rust implementation
- 🌍 Locale-aware data generation (English with extensible architecture)
- 🎲 Seedable RNG for deterministic output
- 📦 80+ modules across 15 categories
- 🧪 100% test coverage on all generators

## Roadmap

### Future (1.0.0)
- Additional locales (Portuguese, Spanish, French)
- Advanced templating system
- Custom provider API
- WebAssembly support

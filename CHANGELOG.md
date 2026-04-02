# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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

# faker::README

// Faker-Rust Documentation

Complete documentation for all faker-rust generators.

## Categories

### Default Generators (100 modules)
Located in `docs/default/`:
- `address::md` - Address data (city, street, zip, country)
- `bank::md` - Banking data (IBAN, SWIFT, account numbers)
- `company::md` - Company names and catch phrases
- `internet::md` - Emails, domains, IPs, passwords
- `name::md` - Names (first, last, with middle)
- `phone_number::md` - Phone numbers
- `lorem::md` - Lorem ipsum text
- And 93 more...

### Movies (17 modules)
Located in `docs/movies/`:
- `star_wars::md` - Star Wars characters, planets, quotes
- `harry_potter::md` - Harry Potter universe
- `lord_of_the_rings::md` - LOTR characters and locations
- `avatar::md` - Avatar: The Last Airbender
- And 13 more...

### TV Shows (38 modules)
Located in `docs/tv_shows/`:
- `game_of_thrones::md` - GoT characters and houses
- `simpsons::md` - Simpsons characters and quotes
- `breaking_bad::md` - Breaking Bad universe
- `rick_and_morty::md` - Rick and Morty
- And 34 more...

### Games (24 modules)
Located in `docs/games/`:
- `pokemon::md` - Pokemon names
- `zelda::md` - Zelda characters and items
- `dnd::md` - Dungeons & Dragons
- `world_of_warcraft::md` - WoW races and classes
- And 20 more...

### Other Categories
- `blockchain/` - Bitcoin, Ethereum, Tezos
- `books/` - Dune, Lovecraft, Culture Series
- `creature/` - Animals, birds, cats, dogs
- `music/` - Rock bands, opera, grateful dead
- `sports/` - Football, basketball, volleyball
- `travel/` - Airports, train stations
- `japanese_media/` - Naruto, Dragon Ball, Studio Ghibli
- `fantasy/` - Tolkien, creatures, locations
- `quotes/` - Famous quotes
- `religion/` - Religious figures and texts

## Usage

All generators follow the same pattern:

```rust
use faker::{module};

module::function() //=> "generated data"
```

## CLI Documentation

See [CLI.md](./CLI.md) for command-line usage.

---

*Adapted from Ruby Faker gem documentation*

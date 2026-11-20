# Gem source parity

The documented gem API (`faker/doc`) is fully mirrored in Rust. This page
covers the additional public methods that exist in the gem **source** but have
no generated documentation page. They are implemented so that the crate also
matches the gem implementation, not only its docs.

A runnable program exercising every method below is available in
[`examples/source_parity.rs`](../examples/source_parity.rs).

## Locale-backed generators

These methods select one value from the same locale data as their documented
singular counterparts.

```rust
use faker_rust::{games, quotes, tv_shows};

games::control::hiss();
games::control::the_board();
games::control::quote();

games::tarkov::weapon();
games::tarkov::boss();
games::tarkov::quest();
games::tarkov::prapor_quest();
games::tarkov::therapist_quest();
games::tarkov::skier_quest();
games::tarkov::peacekeeper_quest();
games::tarkov::mechanic_quest();
games::tarkov::ragman_quest();
games::tarkov::jaeger_quest();
games::tarkov::fence_quest();

quotes::shakespeare::hamlet();
quotes::shakespeare::as_you_like_it();
quotes::shakespeare::king_richard_iii();
quotes::shakespeare::romeo_and_juliet();

tv_shows::futurama::hermes_catchphrase();
faker_rust::quote::fortune_cookie();
```

## Full-array generators

The gem's `fetch_all` returns every value instead of picking one. In Rust these
return `Vec<String>`.

```rust
use faker_rust::movies::{star_wars, tron};

star_wars::call_numbers();
star_wars::call_squadrons();
star_wars::characters();
star_wars::droids();
star_wars::planets();
star_wars::species();
star_wars::vehicles();
star_wars::wookiee_words();

tron::characters(); // programs and users combined
tron::games();
tron::locations();
tron::programs();
tron::taglines();
tron::users();
tron::vehicles();
```

## Structured and algorithmic helpers

Ruby arrays, hashes and regex objects are adapted to native Rust types.

```rust
use faker_rust::{
    address, bank, blockchain, color, driving_license, finance, hacker,
    id_number, internet, invoice, japanese_media, music, omniauth,
};

blockchain::aeternity::rand_strings(12);          // String
blockchain::bitcoin::address_for("main");         // String
blockchain::tezos::encode_tz("tz1", 20);          // String

address::full_address_as_hash(&["city", "zip_code"]); // HashMap<String, String>
bank::bsb_number();                                // six-digit String
color::single_rgb_color();                         // u8
finance::vat_number_keys();                        // Vec<String>
finance::condominium_fiscal_code("IT");            // String
hacker::phrases();                                 // Vec<String>
id_number::ssn_valid();                            // String

internet::private_nets_regex();                    // &[&str]
internet::private_net_checker("10.0.0.1");         // bool
internet::reserved_nets_regex();                   // &[&str]
internet::reserved_net_checker("8.8.8.8");         // bool
invoice::amount_between(10.0, 20.0);               // f64

omniauth::initialize(Some("Ada Lovelace"), None);  // Omniauth
music::music::keys();                              // &[&str]
music::music::key_variants();                      // &[&str]
music::music::key_types();                         // &[&str]
music::music::chord_types();                       // &[&str]

japanese_media::kamen_rider::collectible_device(&[]);
japanese_media::kamen_rider::transformation_device(&[]);

driving_license::InvalidStatePassed::initialize("Invalid state code passed");
```

Notes on the adaptations:

- `private_nets_regex` and `reserved_nets_regex` return the gem's patterns as
  string slices. The companion `*_checker` functions parse an IPv4 address into
  octets, avoiding a regex dependency.
- `full_address_as_hash` accepts the zero-argument address generators that are
  safe to call by name; unknown names are ignored.
- `InvalidStatePassed` is the gem's exception class. Rust models it as an error
  type whose `initialize` constructor carries the message.
- `collectible_device` excludes the Showa era, matching the gem's
  `delete(:showa)` behaviour because that era has no collectible devices.

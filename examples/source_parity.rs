//! Runnable example for gem source methods that were absent from `faker/doc`.
//!
//! These methods complete parity with the Ruby gem source. Rust adaptations
//! use native types where the gem used arrays, hashes and regex objects.

use faker_rust::blockchain::{aeternity, bitcoin, tezos};
use faker_rust::games::{control, tarkov};
use faker_rust::japanese_media::kamen_rider;
use faker_rust::movies::{star_wars, tron};
use faker_rust::{address, bank, color, driving_license, finance, hacker, id_number};
use faker_rust::{internet, invoice, music, omniauth, quote, quotes, tv_shows};

fn show(label: &str, value: &str) {
    println!("{label}: {value}");
}

fn show_all(label: &str, values: &[String]) {
    println!("{label}: {} values", values.len());
}

fn main() {
    // Locale-backed generators.
    show("control.hiss", &control::hiss());
    show("control.the_board", &control::the_board());
    show("control.quote", &control::quote());

    show("tarkov.weapon", &tarkov::weapon());
    show("tarkov.boss", &tarkov::boss());
    show("tarkov.quest", &tarkov::quest());
    show("tarkov.prapor_quest", &tarkov::prapor_quest());
    show("tarkov.therapist_quest", &tarkov::therapist_quest());
    show("tarkov.skier_quest", &tarkov::skier_quest());
    show("tarkov.peacekeeper_quest", &tarkov::peacekeeper_quest());
    show("tarkov.mechanic_quest", &tarkov::mechanic_quest());
    show("tarkov.ragman_quest", &tarkov::ragman_quest());
    show("tarkov.jaeger_quest", &tarkov::jaeger_quest());
    show("tarkov.fence_quest", &tarkov::fence_quest());

    show("shakespeare.hamlet", &quotes::shakespeare::hamlet());
    show(
        "shakespeare.as_you_like_it",
        &quotes::shakespeare::as_you_like_it(),
    );
    show(
        "shakespeare.king_richard_iii",
        &quotes::shakespeare::king_richard_iii(),
    );
    show(
        "shakespeare.romeo_and_juliet",
        &quotes::shakespeare::romeo_and_juliet(),
    );
    show(
        "futurama.hermes_catchphrase",
        &tv_shows::futurama::hermes_catchphrase(),
    );
    show("quote.fortune_cookie", &quote::fortune_cookie());

    // `fetch_all` equivalents return the full locale array.
    show_all("star_wars.call_numbers", &star_wars::call_numbers());
    show_all("star_wars.call_squadrons", &star_wars::call_squadrons());
    show_all("star_wars.characters", &star_wars::characters());
    show_all("star_wars.droids", &star_wars::droids());
    show_all("star_wars.planets", &star_wars::planets());
    show_all("star_wars.species", &star_wars::species());
    show_all("star_wars.vehicles", &star_wars::vehicles());
    show_all("star_wars.wookiee_words", &star_wars::wookiee_words());

    show_all("tron.characters", &tron::characters());
    show_all("tron.games", &tron::games());
    show_all("tron.locations", &tron::locations());
    show_all("tron.programs", &tron::programs());
    show_all("tron.taglines", &tron::taglines());
    show_all("tron.users", &tron::users());
    show_all("tron.vehicles", &tron::vehicles());

    // Blockchain helpers.
    show("aeternity.rand_strings", &aeternity::rand_strings(12));
    show("bitcoin.address_for", &bitcoin::address_for("main"));
    show("tezos.encode_tz", &tezos::encode_tz("tz1", 20));

    // Structured and algorithmic helpers.
    let parts = address::full_address_as_hash(&["city", "zip_code"]);
    println!("address.full_address_as_hash: {} keys", parts.len());
    show("bank.bsb_number", &bank::bsb_number());
    println!("color.single_rgb_color: {}", color::single_rgb_color());
    show_all("finance.vat_number_keys", &finance::vat_number_keys());
    show(
        "finance.condominium_fiscal_code",
        &finance::condominium_fiscal_code("IT"),
    );
    show_all("hacker.phrases", &hacker::phrases());
    show("id_number.ssn_valid", &id_number::ssn_valid());

    println!(
        "internet.private_nets_regex: {} patterns",
        internet::private_nets_regex().len()
    );
    println!(
        "internet.private_net_checker: {}",
        internet::private_net_checker("10.0.0.1")
    );
    println!(
        "internet.reserved_nets_regex: {} patterns",
        internet::reserved_nets_regex().len()
    );
    println!(
        "internet.reserved_net_checker: {}",
        internet::reserved_net_checker("8.8.8.8")
    );
    println!(
        "invoice.amount_between: {:.2}",
        invoice::amount_between(10.0, 20.0)
    );

    let identity = omniauth::initialize(Some("Ada Lovelace"), None);
    show("omniauth.initialize", &identity.name);

    println!("music.keys: {:?}", music::music::keys());
    println!("music.key_variants: {:?}", music::music::key_variants());
    println!("music.key_types: {:?}", music::music::key_types());
    println!("music.chord_types: {:?}", music::music::chord_types());

    show(
        "kamen_rider.collectible_device",
        &kamen_rider::collectible_device(&[]),
    );
    show(
        "kamen_rider.transformation_device",
        &kamen_rider::transformation_device(&[]),
    );

    let error = driving_license::InvalidStatePassed::initialize("Invalid state code passed");
    show("driving_license.initialize", error.message());
}

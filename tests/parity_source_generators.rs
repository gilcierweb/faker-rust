//! Regression coverage for gem source generators that were absent from
//! `faker/doc` and therefore missed by the documented-API parity audit.
//!
//! Every call below mirrors a public gem class method. String generators must
//! return a usable value; `fetch_all` equivalents must return the full array.

fn check_vec(label: &str, values: &[String]) {
    assert!(!values.is_empty(), "{label} returned no values");
    assert!(
        values.iter().all(|value| !value.is_empty()),
        "{label} returned an empty value"
    );
}

#[test]
fn control_generators_return_values() {
    check_vec("control::hiss", &[faker_rust::games::control::hiss()]);
    check_vec(
        "control::the_board",
        &[faker_rust::games::control::the_board()],
    );
    check_vec("control::quote", &[faker_rust::games::control::quote()]);
}

#[test]
fn tarkov_generators_return_values() {
    check_vec("tarkov::weapon", &[faker_rust::games::tarkov::weapon()]);
    check_vec("tarkov::boss", &[faker_rust::games::tarkov::boss()]);
    check_vec("tarkov::quest", &[faker_rust::games::tarkov::quest()]);
    check_vec(
        "tarkov::prapor_quest",
        &[faker_rust::games::tarkov::prapor_quest()],
    );
    check_vec(
        "tarkov::therapist_quest",
        &[faker_rust::games::tarkov::therapist_quest()],
    );
    check_vec(
        "tarkov::skier_quest",
        &[faker_rust::games::tarkov::skier_quest()],
    );
    check_vec(
        "tarkov::peacekeeper_quest",
        &[faker_rust::games::tarkov::peacekeeper_quest()],
    );
    check_vec(
        "tarkov::mechanic_quest",
        &[faker_rust::games::tarkov::mechanic_quest()],
    );
    check_vec(
        "tarkov::ragman_quest",
        &[faker_rust::games::tarkov::ragman_quest()],
    );
    check_vec(
        "tarkov::jaeger_quest",
        &[faker_rust::games::tarkov::jaeger_quest()],
    );
    check_vec(
        "tarkov::fence_quest",
        &[faker_rust::games::tarkov::fence_quest()],
    );
}

#[test]
fn shakespeare_generators_return_values() {
    check_vec(
        "shakespeare::hamlet",
        &[faker_rust::quotes::shakespeare::hamlet()],
    );
    check_vec(
        "shakespeare::as_you_like_it",
        &[faker_rust::quotes::shakespeare::as_you_like_it()],
    );
    check_vec(
        "shakespeare::king_richard_iii",
        &[faker_rust::quotes::shakespeare::king_richard_iii()],
    );
    check_vec(
        "shakespeare::romeo_and_juliet",
        &[faker_rust::quotes::shakespeare::romeo_and_juliet()],
    );
}

#[test]
fn futurama_hermes_catchphrase_returns_value() {
    check_vec(
        "futurama::hermes_catchphrase",
        &[faker_rust::tv_shows::futurama::hermes_catchphrase()],
    );
}

#[test]
fn star_wars_fetch_all_generators_return_full_arrays() {
    check_vec(
        "star_wars::call_numbers",
        &faker_rust::movies::star_wars::call_numbers(),
    );
    check_vec(
        "star_wars::call_squadrons",
        &faker_rust::movies::star_wars::call_squadrons(),
    );
    check_vec(
        "star_wars::characters",
        &faker_rust::movies::star_wars::characters(),
    );
    check_vec(
        "star_wars::droids",
        &faker_rust::movies::star_wars::droids(),
    );
    check_vec(
        "star_wars::planets",
        &faker_rust::movies::star_wars::planets(),
    );
    check_vec(
        "star_wars::species",
        &faker_rust::movies::star_wars::species(),
    );
    check_vec(
        "star_wars::vehicles",
        &faker_rust::movies::star_wars::vehicles(),
    );
    check_vec(
        "star_wars::wookiee_words",
        &faker_rust::movies::star_wars::wookiee_words(),
    );
}

#[test]
fn tron_fetch_all_generators_return_full_arrays() {
    check_vec("tron::characters", &faker_rust::movies::tron::characters());
    check_vec("tron::games", &faker_rust::movies::tron::games());
    check_vec("tron::locations", &faker_rust::movies::tron::locations());
    check_vec("tron::programs", &faker_rust::movies::tron::programs());
    check_vec("tron::taglines", &faker_rust::movies::tron::taglines());
    check_vec("tron::users", &faker_rust::movies::tron::users());
    check_vec("tron::vehicles", &faker_rust::movies::tron::vehicles());
}

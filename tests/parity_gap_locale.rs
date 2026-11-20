//! Parity tests for gem-documented generators missed by the initial audit
//! because they are backed by locale data through `method_missing` or were
//! classified as internal helpers.

#[test]
fn parity_gap_beer_brand() {
    assert!(!faker_rust::beer::brand().is_empty());
}

#[test]
fn parity_gap_food_sushi() {
    assert!(!faker_rust::food::sushi().is_empty());
}

#[test]
fn parity_gap_job_education_level() {
    assert!(!faker_rust::job::education_level().is_empty());
}

#[test]
fn parity_gap_job_employment_type() {
    assert!(!faker_rust::job::employment_type().is_empty());
}

#[test]
fn parity_gap_verb_base() {
    assert!(!faker_rust::verb::base().is_empty());
}

#[test]
fn parity_gap_verb_past_participle() {
    assert!(!faker_rust::verb::past_participle().is_empty());
}

#[test]
fn parity_gap_verb_simple_present() {
    assert!(!faker_rust::verb::simple_present().is_empty());
}

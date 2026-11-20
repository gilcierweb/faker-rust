//! Crate-wide parity tests for company, identifier and OmniAuth generators.

#[test]
fn parity_company_european_numbers() {
    assert_eq!(
        faker_rust::company::spanish_organisation_number(None).len(),
        9
    );
    assert_eq!(faker_rust::company::swedish_organisation_number().len(), 10);
    assert_eq!(faker_rust::company::czech_organisation_number().len(), 8);
    assert_eq!(faker_rust::company::french_siren_number().len(), 9);
    assert_eq!(faker_rust::company::french_siret_number().len(), 14);
    assert_eq!(
        faker_rust::company::norwegian_organisation_number().len(),
        9
    );
    assert_eq!(faker_rust::company::indian_gst_number(None).len(), 15);
}

#[test]
fn parity_company_other_numbers() {
    assert_eq!(faker_rust::company::australian_business_number().len(), 11);
    assert_eq!(
        faker_rust::company::polish_taxpayer_identification_number().len(),
        10
    );
    assert_eq!(
        faker_rust::company::polish_register_of_national_economy(9).len(),
        9
    );
    assert_eq!(
        faker_rust::company::polish_register_of_national_economy(14).len(),
        14
    );
    assert_eq!(
        faker_rust::company::russian_tax_number(None, true).len(),
        10
    );

    let cnpj = faker_rust::company::brazilian_company_number(false);
    assert_eq!(cnpj.len(), 14);
    assert!(cnpj.chars().all(|c| c.is_ascii_digit()));
    assert!(faker_rust::company::brazilian_company_number(true).contains('.'));
}

#[test]
fn parity_company_south_african_numbers() {
    let pty = faker_rust::company::south_african_pty_ltd_registration_number();
    assert_eq!(pty.matches('/').count(), 2);
    assert!(pty.ends_with("/07"));

    let cc = faker_rust::company::south_african_close_corporation_registration_number();
    assert_eq!(cc.matches('/').count(), 2);
    assert!(cc.contains("/23"));

    let listed = faker_rust::company::south_african_listed_company_registration_number();
    assert_eq!(listed.matches('/').count(), 2);
    assert!(listed.ends_with("/06"));

    let trust = faker_rust::company::south_african_trust_registration_number();
    assert!(trust.starts_with("IT") && trust.contains('/'));
}

#[test]
fn parity_id_number_spanish_and_sa() {
    let citizen = faker_rust::id_number::spanish_citizen_number();
    assert_eq!(citizen.len(), 10);
    assert!(citizen.contains('-'));

    let foreign = faker_rust::id_number::spanish_foreign_citizen_number();
    assert_eq!(foreign.matches('-').count(), 2);
    assert!(foreign.starts_with(['X', 'Y', 'Z']));

    let valid = faker_rust::id_number::valid_south_african_id_number();
    assert_eq!(valid.len(), 13);
    assert!(valid.chars().all(|c| c.is_ascii_digit()));

    let invalid = faker_rust::id_number::invalid_south_african_id_number();
    assert_eq!(invalid.len(), 13);
}

#[test]
fn parity_id_number_international() {
    let cpf = faker_rust::id_number::brazilian_citizen_number(false);
    assert_eq!(cpf.len(), 11);
    assert!(faker_rust::id_number::brazilian_citizen_number(true).contains('.'));

    let rg = faker_rust::id_number::brazilian_id(false);
    assert!(rg.len() >= 8);
    assert!(faker_rust::id_number::brazilian_id(true).contains('.'));

    let chilean = faker_rust::id_number::chilean_id();
    assert_eq!(chilean.matches('-').count(), 1);

    let croatian = faker_rust::id_number::croatian_id(false);
    assert_eq!(croatian.len(), 11);
    assert!(faker_rust::id_number::croatian_id(true).starts_with("HR"));

    let danish = faker_rust::id_number::danish_id_number(false);
    assert_eq!(danish.len(), 10);
    assert!(faker_rust::id_number::danish_id_number(true).contains('-'));

    let insee = faker_rust::id_number::french_insee_number();
    assert_eq!(insee.len(), 15);
    assert!(insee[..13].chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn parity_south_africa_generators() {
    assert!(!faker_rust::south_africa::cell_phone().is_empty());
    assert_eq!(faker_rust::south_africa::valid_id_number().len(), 13);
    assert_eq!(faker_rust::south_africa::invalid_id_number().len(), 13);
    assert!(faker_rust::south_africa::vat_number().starts_with("ZA"));
    assert!(faker_rust::south_africa::pty_ltd_registration_number().ends_with("/07"));
    assert!(faker_rust::south_africa::close_corporation_registration_number().contains("/23"));
    assert!(faker_rust::south_africa::listed_company_registration_number().ends_with("/06"));
    assert!(faker_rust::south_africa::trust_registration_number().starts_with("IT"));
}

#[test]
fn parity_omniauth_auth_hashes() {
    let providers = [
        faker_rust::omniauth::google(None, None, None),
        faker_rust::omniauth::facebook(None, None, None, None),
        faker_rust::omniauth::twitter(None, None, None),
        faker_rust::omniauth::linkedin(None, None, None),
        faker_rust::omniauth::github(None, None, None),
        faker_rust::omniauth::apple(None, None, None),
        faker_rust::omniauth::auth0(None, None, None),
    ];
    for auth in &providers {
        let object = auth.as_object().unwrap();
        assert!(object.contains_key("provider"));
        assert!(object.contains_key("uid"));
        assert!(object.contains_key("info"));
        assert!(object.contains_key("credentials"));
        assert!(object.contains_key("extra"));
        assert!(object["uid"].as_str().unwrap().len() >= 6);
    }
}

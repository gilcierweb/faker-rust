//! Parity tests for gem-documented Barcode generators omitted by the initial audit.

#[test]
fn parity_gap_barcode_ismn() {
    let value = faker_rust::barcode::ismn();
    assert!(value.starts_with("9790"), "unexpected ISMN prefix: {value}");
    assert_eq!(value.len(), 13);
    assert!(value.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn parity_gap_barcode_issn() {
    let value = faker_rust::barcode::issn();
    assert!(value.starts_with("977"), "unexpected ISSN prefix: {value}");
    assert_eq!(value.len(), 13);
    assert!(value.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn parity_gap_barcode_upc_a_with_composite_symbology() {
    let value = faker_rust::barcode::upc_a_with_composite_symbology();
    let mut parts = value.split('|');
    assert_eq!(parts.next().unwrap().len(), 12);
    assert_eq!(parts.next().unwrap().len(), 8);
    assert!(parts.next().is_none());
}

#[test]
fn parity_gap_barcode_upc_e_with_composite_symbology() {
    let value = faker_rust::barcode::upc_e_with_composite_symbology();
    let mut parts = value.split('|');
    let upc_e = parts.next().unwrap();
    assert_eq!(upc_e.len(), 7);
    assert!(upc_e.starts_with('0') || upc_e.starts_with('1'));
    assert_eq!(parts.next().unwrap().len(), 8);
    assert!(parts.next().is_none());
}

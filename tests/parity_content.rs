//! Crate-wide parity tests for text markup, structured data and Lovecraft generators.

#[test]
fn parity_lovecraft_text_generators() {
    let chant = faker_rust::books::lovecraft::fhtagn(3);
    assert_eq!(chant.matches("Ph'nglui").count(), 3);
    assert_eq!(chant.matches(". ").count(), 2);

    let sentence = faker_rust::books::lovecraft::sentence(3, 0, false);
    assert!(sentence.ends_with('.'));
    assert!(sentence.chars().next().unwrap().is_uppercase());

    let sentences = faker_rust::books::lovecraft::sentences(2);
    assert_eq!(sentences.len(), 2);
    assert!(sentences.iter().all(|s| s.ends_with('.')));

    let paragraph = faker_rust::books::lovecraft::paragraph(2, 0);
    assert!(paragraph.contains('.'));

    let paragraphs = faker_rust::books::lovecraft::paragraphs(2);
    assert_eq!(paragraphs.len(), 2);

    let long = faker_rust::books::lovecraft::paragraph_by_chars(120);
    assert!(long.ends_with('.'));
    assert!(long.len() >= 100);

    let words = faker_rust::books::lovecraft::words(3, false);
    assert_eq!(words.len(), 3);
    assert!(words.iter().all(|w| !w.is_empty() && !w.contains(' ')));
}

#[test]
fn parity_html_inline_generators() {
    let code = faker_rust::html::code();
    assert!(code.starts_with("<code>") && code.ends_with("</code>"));

    let element = faker_rust::html::element("div", "hello");
    assert!(element.starts_with("<div "));
    assert!(element.contains("hello"));
    assert!(element.ends_with("</div>"));

    let emphasis = faker_rust::html::emphasis();
    assert!(emphasis.starts_with("<em>") && emphasis.ends_with("</em>"));

    let heading = faker_rust::html::heading();
    assert!(heading.starts_with("<h") && heading.contains("</h"));
    assert!(heading.ends_with('>'));

    let link = faker_rust::html::link("stylesheet");
    assert!(link.contains("rel=\"stylesheet\""));
    assert!(link.ends_with(".css\">"));

    let script = faker_rust::html::script();
    assert!(script.starts_with("<script src=\""));
    assert!(script.ends_with(".js\"></script>"));
}

#[test]
fn parity_html_block_generators() {
    let ordered = faker_rust::html::ordered_list();
    assert!(ordered.starts_with("<ol>\n<li>"));
    assert!(ordered.ends_with("\n</ol>"));

    let unordered = faker_rust::html::unordered_list();
    assert!(unordered.starts_with("<ul>\n<li>"));
    assert!(unordered.ends_with("\n</ul>"));

    let paragraph = faker_rust::html::paragraph(3, false, 0, None);
    assert!(paragraph.starts_with("<p>") && paragraph.ends_with("</p>"));

    let table = faker_rust::html::table();
    assert!(table.starts_with("<table>\n<thead>"));
    assert!(table.ends_with("\n</table>"));

    let chosen = faker_rust::html::random(&[]);
    assert!(!chosen.is_empty());

    let sandwich = faker_rust::html::sandwich(3, 1);
    assert!(sandwich.contains("<h"));
    assert!(sandwich.contains("<p>"));
}

#[test]
fn parity_markdown_generators() {
    let ordered = faker_rust::markdown::ordered_list();
    assert!(ordered.contains("1. "));
    assert!(ordered.trim_end().ends_with('.'));

    let unordered = faker_rust::markdown::unordered_list();
    assert!(unordered.contains("* "));

    let table = faker_rust::markdown::table();
    assert!(table.contains(" | "));
    assert!(table.contains("---- | ---- | ----"));
    assert_eq!(table.lines().count(), 4);

    let chosen = faker_rust::markdown::random(&[]);
    assert!(!chosen.is_empty());

    let sandwich = faker_rust::markdown::sandwich(3, 1);
    assert!(sandwich.starts_with('#'));
    assert!(sandwich.contains('\n'));
}

#[test]
fn parity_types_scalar_generators() {
    let ch = faker_rust::types::character();
    assert_eq!(ch.len(), 1);
    assert!(ch.chars().next().unwrap().is_ascii_alphanumeric());

    let number = faker_rust::types::rb_integer(10, 20);
    assert!((10..=20).contains(&number));

    let text = faker_rust::types::rb_string(2);
    assert_eq!(text.split_whitespace().count(), 2);

    let simple = faker_rust::types::random_type();
    assert!(simple.is_string() || simple.is_i64() || simple.is_u64());

    let complex = faker_rust::types::random_complex_type();
    assert!(!complex.is_null());
}

#[test]
fn parity_types_collection_generators() {
    let array = faker_rust::types::rb_array(2);
    assert!(array.is_array());
    assert_eq!(array.as_array().unwrap().len(), 2);

    let hash = faker_rust::types::rb_hash(1);
    assert!(hash.is_object());
    assert_eq!(hash.as_object().unwrap().len(), 1);

    let complex = faker_rust::types::complex_rb_hash(1);
    assert!(complex.is_object());
    assert_eq!(complex.as_object().unwrap().len(), 1);
}

#[test]
fn parity_json_generators() {
    let shallow = faker_rust::json::shallow_json(3);
    let parsed: serde_json::Value = serde_json::from_str(&shallow).unwrap();
    assert_eq!(parsed.as_object().unwrap().len(), 3);
    assert!(parsed.as_object().unwrap().values().all(|v| v.is_string()));

    let deep = faker_rust::json::add_depth_to_json(&shallow, 2);
    let parsed: serde_json::Value = serde_json::from_str(&deep).unwrap();
    let root = parsed.as_object().unwrap();
    assert_eq!(root.len(), 3);
    assert!(root.values().all(|v| v.is_object()));
    assert!(root.values().all(|v| v.as_object().unwrap().len() == 2));
}

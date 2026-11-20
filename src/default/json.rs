//! JSON data generator

use crate::config::FakerConfig;

/// Generate a random JSON object as string
pub fn object() -> String {
    let config = FakerConfig::current();
    let mut keys = vec!["name", "email", "age", "city", "country", "company", "job"];
    config.shuffle(&mut keys);

    let num_fields = config.rand_range(2, 5) as usize;
    let mut fields = Vec::new();

    for key in keys.iter().take(num_fields) {
        let value = generate_value(&config);
        fields.push(format!("\"{}\": {}", key, value));
    }

    format!("{{{}}}", fields.join(", "))
}

fn generate_value(config: &FakerConfig) -> String {
    let type_choice = config.rand_range(0, 3);
    match type_choice {
        0 => {
            // String
            let strings = ["test", "example", "sample", "data"];
            format!("\"{}\"", strings[config.rand_usize(strings.len())])
        }
        1 => {
            // Number
            config.rand_range(1, 100).to_string()
        }
        _ => {
            // Boolean
            if config.rand_bool() {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
    }
}

/// Generate a random JSON array as string
pub fn array() -> String {
    let config = FakerConfig::current();
    let length = config.rand_range(1, 5) as usize;
    let mut items = Vec::new();

    for _ in 0..length {
        items.push(generate_value(&config));
    }

    format!("[{}]", items.join(", "))
}

// --- gem parity generators (algorithmic) ---

fn shallow_map(width: usize) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    for _ in 0..width {
        map.insert(
            crate::name::first_name(),
            serde_json::Value::String(crate::name::first_name()),
        );
    }
    map
}

fn replace_leaves(value: &mut serde_json::Value, width: usize) {
    match value {
        serde_json::Value::Object(map) => {
            for child in map.values_mut() {
                replace_leaves(child, width);
            }
        }
        _ => *value = serde_json::Value::Object(shallow_map(width)),
    }
}

/// Gem-compatible generator: JSON object with a fixed number of name pairs.
pub fn shallow_json(width: usize) -> String {
    serde_json::to_string(&serde_json::Value::Object(shallow_map(width))).unwrap_or_default()
}

/// Gem-compatible generator: replace every JSON leaf with a nested object.
pub fn add_depth_to_json(json: &str, width: usize) -> String {
    let mut value: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    replace_leaves(&mut value, width);
    serde_json::to_string(&value).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object() {
        let json = object();
        assert!(json.starts_with('{'));
        assert!(json.ends_with('}'));
    }

    #[test]
    fn test_object_randomizes_key_selection() {
        // The generated object must not always start with the same key.
        let mut first_keys = std::collections::HashSet::new();
        for _ in 0..100 {
            let json = object();
            let first = json
                .trim_start_matches('{')
                .split(',')
                .next()
                .unwrap()
                .trim()
                .trim_start_matches('"')
                .split('"')
                .next()
                .unwrap()
                .to_string();
            first_keys.insert(first);
        }
        assert!(
            first_keys.len() > 1,
            "object key selection is not randomized: {first_keys:?}"
        );
    }

    #[test]
    fn test_array() {
        let arr = array();
        assert!(arr.starts_with('['));
        assert!(arr.ends_with(']'));
    }
}

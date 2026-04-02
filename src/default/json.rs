//! JSON data generator

use crate::config::FakerConfig;

/// Generate a random JSON object as string
pub fn object() -> String {
    let config = FakerConfig::current();
    let keys = vec![
        "name", "email", "age", "city", "country", "company", "job",
    ];
    
    let num_fields = config.rand_range(2, 5) as usize;
    let mut fields = Vec::new();
    
    for i in 0..num_fields {
        if i < keys.len() {
            let key = keys[i];
            let value = generate_value(&config);
            fields.push(format!("\"{}\": {}", key, value));
        }
    }
    
    format!("{{{}}}", fields.join(", "))
}

fn generate_value(config: &FakerConfig) -> String {
    let type_choice = config.rand_range(0, 3);
    match type_choice {
        0 => {
            // String
            let strings = vec!["test", "example", "sample", "data"];
            format!("\"{}\"", strings[config.rand_range(0, strings.len() as u32) as usize])
        }
        1 => {
            // Number
            config.rand_range(1, 100).to_string()
        }
        _ => {
            // Boolean
            if config.rand_bool() { "true".to_string() } else { "false".to_string() }
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
    fn test_array() {
        let arr = array();
        assert!(arr.starts_with('['));
        assert!(arr.ends_with(']'));
    }
}

//! Types generator - programming types

use crate::base::sample;

/// Generate a random Ruby type
pub fn ruby() -> String {
    let types = [
        "String", "Integer", "Float", "Boolean", "Array", "Hash", "Symbol", "NilClass", "Range",
        "Regexp", "Time", "Date",
    ];
    sample(&types).to_string()
}

/// Generate a random JavaScript type
pub fn javascript() -> String {
    let types = [
        "string",
        "number",
        "boolean",
        "object",
        "array",
        "function",
        "undefined",
        "null",
        "symbol",
        "bigint",
        "Date",
        "RegExp",
    ];
    sample(&types).to_string()
}

/// Generate a random SQL type
pub fn sql() -> String {
    let types = [
        "VARCHAR",
        "INTEGER",
        "FLOAT",
        "DOUBLE",
        "BOOLEAN",
        "DATE",
        "TIMESTAMP",
        "TEXT",
        "BLOB",
        "DECIMAL",
        "CHAR",
        "BIGINT",
    ];
    sample(&types).to_string()
}

/// Generate a random Rust type
pub fn rust() -> String {
    let types = [
        "String",
        "i32",
        "i64",
        "f32",
        "f64",
        "bool",
        "Vec<T>",
        "Option<T>",
        "Result<T, E>",
        "HashMap<K, V>",
        "Arc<T>",
        "Rc<T>",
    ];
    sample(&types).to_string()
}

// --- gem parity generators (algorithmic) ---

fn lorem_pool() -> Vec<String> {
    crate::locale::fetch_locale("lorem.words", "en").unwrap_or_else(|| vec!["lorem".to_string()])
}

fn unique_words(count: usize) -> Vec<String> {
    let pool = lorem_pool();
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    let mut guard = 0;
    while result.len() < count && guard < count * 20 + 100 {
        let word = crate::base::sample(&pool);
        if seen.insert(word.clone()) {
            result.push(word);
        }
        guard += 1;
    }
    result
}

/// Gem-compatible generator: random alphanumeric character.
pub fn character() -> String {
    let alphabet: Vec<char> = ('0'..='9').chain('a'..='z').collect();
    crate::base::sample(&alphabet).to_string()
}

/// Gem-compatible generator: random Ruby integer in the inclusive range.
pub fn rb_integer(from: i64, to: i64) -> i64 {
    if from >= to {
        return from;
    }
    let config = crate::config::FakerConfig::current();
    let width = (to - from + 1).min(u32::MAX as i64) as u32;
    from + config.rand_u32(width) as i64
}

/// Gem-compatible generator: random Ruby string made of lorem words.
pub fn rb_string(words: usize) -> String {
    crate::lorem::words(words)
}

/// Gem-compatible generator: random Ruby value of a simple type.
pub fn random_type() -> serde_json::Value {
    if crate::config::FakerConfig::current().rand_bool() {
        serde_json::Value::String(rb_string(1))
    } else {
        serde_json::Value::Number(serde_json::Number::from(rb_integer(0, 100)))
    }
}

/// Gem-compatible generator: random Ruby value of a simple or collection type.
pub fn random_complex_type() -> serde_json::Value {
    match crate::config::FakerConfig::current().rand_range(0, 4) {
        0 => serde_json::Value::String(rb_string(1)),
        1 => serde_json::Value::Number(serde_json::Number::from(rb_integer(0, 100))),
        2 => rb_hash(1),
        _ => rb_array(1),
    }
}

/// Gem-compatible generator: random Ruby array of simple values.
pub fn rb_array(len: usize) -> serde_json::Value {
    serde_json::Value::Array((0..len).map(|_| random_type()).collect())
}

/// Gem-compatible generator: random Ruby hash of simple values.
pub fn rb_hash(number: usize) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for key in unique_words(number.saturating_mul(2))
        .into_iter()
        .take(number)
    {
        map.insert(key, random_type());
    }
    serde_json::Value::Object(map)
}

/// Gem-compatible generator: random Ruby hash of nested values.
pub fn complex_rb_hash(number: usize) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for key in unique_words(number.saturating_mul(2))
        .into_iter()
        .take(number)
    {
        map.insert(key, random_complex_type());
    }
    serde_json::Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruby() {
        assert!(!ruby().is_empty());
    }

    #[test]
    fn test_javascript() {
        assert!(!javascript().is_empty());
    }

    #[test]
    fn test_sql() {
        assert!(!sql().is_empty());
    }

    #[test]
    fn test_rust() {
        assert!(!rust().is_empty());
    }
}

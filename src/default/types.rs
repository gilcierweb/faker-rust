//! Types generator - programming types

use crate::base::sample;

/// Generate a random Ruby type
pub fn ruby() -> String {
    let types = [
        "String", "Integer", "Float", "Boolean", "Array", "Hash",
        "Symbol", "NilClass", "Range", "Regexp", "Time", "Date",
    ];
    sample(&types).to_string()
}

/// Generate a random JavaScript type
pub fn javascript() -> String {
    let types = [
        "string", "number", "boolean", "object", "array", "function",
        "undefined", "null", "symbol", "bigint", "Date", "RegExp",
    ];
    sample(&types).to_string()
}

/// Generate a random SQL type
pub fn sql() -> String {
    let types = [
        "VARCHAR", "INTEGER", "FLOAT", "DOUBLE", "BOOLEAN", "DATE",
        "TIMESTAMP", "TEXT", "BLOB", "DECIMAL", "CHAR", "BIGINT",
    ];
    sample(&types).to_string()
}

/// Generate a random Rust type
pub fn rust() -> String {
    let types = [
        "String", "i32", "i64", "f32", "f64", "bool", "Vec<T>",
        "Option<T>", "Result<T, E>", "HashMap<K, V>", "Arc<T>", "Rc<T>",
    ];
    sample(&types).to_string()
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

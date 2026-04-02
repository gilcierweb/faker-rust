//! Locale data loader - loads and provides access to locale YAML files

use once_cell::sync::Lazy;
use serde_yaml::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_locales_path() -> PathBuf {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let possible_paths = vec![
        exe_dir.join("locales"),
        exe_dir.join("..").join("locales"),
        PathBuf::from("locales"),
        PathBuf::from("..").join("locales"),
    ];

    for path in possible_paths {
        if path.exists() && path.is_dir() {
            return path;
        }
    }

    PathBuf::from("locales")
}

static LOCALE_DATA: Lazy<HashMap<String, Value>> = Lazy::new(load_locale_data);

fn load_locale_data() -> HashMap<String, Value> {
    let mut result: HashMap<String, Value> = HashMap::new();
    let locales_path = get_locales_path();

    if !locales_path.exists() {
        return result;
    }

    if let Ok(entries) = fs::read_dir(&locales_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                if let Some(filename) = path.file_stem() {
                    let locale_name = filename.to_string_lossy().to_string();
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(yaml) = serde_yaml::from_str::<Value>(&contents) {
                            result.insert(locale_name, yaml);
                        }
                    }
                }
            }

            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();

                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_file()
                            && sub_path.extension().map_or(false, |ext| ext == "yml")
                        {
                            let file_stem = sub_path
                                .file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default();

                            let locale_name = format!("{}_{}", dir_name, file_stem);
                            if let Ok(contents) = fs::read_to_string(&sub_path) {
                                if let Ok(yaml) = serde_yaml::from_str::<Value>(&contents) {
                                    result.insert(locale_name, yaml);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

pub fn fetch_locale(key: &str, locale: &str) -> Option<Vec<String>> {
    fetch_locale_with_context(key, locale, None)
}

pub fn fetch_locale_with_context(
    key: &str,
    locale: &str,
    context_category: Option<&str>,
) -> Option<Vec<String>> {
    let locales_to_try: Vec<String> = if locale.contains('_') || locale.contains('-') {
        let short = locale.split(|c| c == '-' || c == '_').next().unwrap();
        vec![locale.to_string(), short.to_string()]
    } else {
        vec![locale.to_string()]
    };

    for loc in locales_to_try.iter() {
        if let Some(locale_data) = LOCALE_DATA.get(loc) {
            if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                return Some(result);
            }
        }

        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() >= 2 {
            let category = parts[0];
            let subdir_key = format!("{}_{}", loc, category);
            if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
                if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                    return Some(result);
                }
            }
        }

        let subdir_key = format!("{}_{}", loc, key.split('.').nth(1).unwrap_or("name"));
        if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
            if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                return Some(result);
            }
        }
    }

    if locale != "en" {
        return fetch_locale_with_context(key, "en", context_category);
    }

    None
}

fn try_extract(
    locale_data: &Value,
    locale: &str,
    key: &str,
    context_category: Option<&str>,
) -> Option<Vec<String>> {
    if let Value::Mapping(top) = locale_data {
        if let Some(inner) = top.get(&Value::String(locale.to_string())) {
            if let Value::Mapping(inner_map) = inner {
                if let Some(faker) = inner_map.get(&Value::String("faker".to_string())) {
                    let clean_key = if key.starts_with("faker.") {
                        key[6..].to_string()
                    } else if key.starts_with("faker_") {
                        key[6..].replace('_', ".")
                    } else if key.contains("faker.") {
                        key.replace("faker.", "").replace('_', ".")
                    } else {
                        key.replace('_', ".")
                    };
                    return extract_nested_array(faker, &clean_key, context_category);
                }
            }
        }

        if top.contains_key(&Value::String("faker".to_string())) {
            if let Some(faker) = top.get(&Value::String("faker".to_string())) {
                let clean_key = if key.starts_with("faker.") {
                    key[6..].to_string()
                } else if key.starts_with("faker_") {
                    key[6..].replace('_', ".")
                } else if key.contains("faker.") {
                    key.replace("faker.", "").replace('_', ".")
                } else {
                    key.replace('_', ".")
                };
                return extract_nested_array(faker, &clean_key, context_category);
            }
        }
    }

    None
}

fn extract_nested_array(
    data: &Value,
    key: &str,
    context_category: Option<&str>,
) -> Option<Vec<String>> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current: &Value = data;

    for (i, part) in parts.iter().enumerate() {
        let part_str = part.to_string();
        match current {
            Value::Mapping(map) => {
                if let Some(next) = map.get(&Value::String(part_str)) {
                    current = next;
                } else {
                    return None;
                }
            }
            Value::Sequence(seq) => {
                if i == parts.len() - 1 {
                    return extract_strings_from_seq(seq, context_category);
                }
                return None;
            }
            _ => return None,
        }
    }

    match current {
        Value::Sequence(seq) => extract_strings_from_seq(seq, context_category),
        _ => None,
    }
}

fn extract_strings_from_seq(seq: &[Value], context_category: Option<&str>) -> Option<Vec<String>> {
    let arr: Vec<String> = seq
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => {
                let resolved = resolve_placeholder(s, context_category);
                if resolved.is_empty() {
                    None
                } else {
                    Some(resolved)
                }
            }
            _ => None,
        })
        .collect();
    if arr.is_empty() {
        None
    } else {
        Some(arr)
    }
}

fn resolve_placeholder(s: &str, context_category: Option<&str>) -> String {
    resolve_placeholder_recursive(s, context_category, 0, 10)
}

fn resolve_placeholder_recursive(
    s: &str,
    context_category: Option<&str>,
    depth: usize,
    max_depth: usize,
) -> String {
    if depth >= max_depth {
        return s.to_string();
    }

    if s.starts_with("#{") && s.ends_with('}') {
        let inner = &s[2..s.len() - 1];

        let resolved = if inner.contains('.') {
            let parts: Vec<&str> = inner.split('.').collect();
            if parts.len() >= 2 {
                let cat = parts[0].to_lowercase();
                let field = parts[1].to_lowercase();
                let lookup_key = format!("{}_{}", cat, field);

                if let Some(values) = fetch_locale_with_context(&lookup_key, "en", None) {
                    if let Some(v) = values.first() {
                        return resolve_placeholder_recursive(
                            v,
                            context_category,
                            depth + 1,
                            max_depth,
                        );
                    }
                }

                let en_cat_key = format!("en_{}", cat);
                if let Some(subdata) = LOCALE_DATA.get(&en_cat_key) {
                    return extract_nested_value(
                        subdata,
                        &format!("{}.{}", cat, field),
                        context_category,
                    )
                    .unwrap_or_else(|| {
                        if let Some(values) = fetch_locale_with_context(&lookup_key, "en", None) {
                            values.first().cloned().unwrap_or_else(|| s.to_string())
                        } else {
                            s.to_string()
                        }
                    });
                }
            }
            s.to_string()
        } else {
            let key = if let Some(cat) = context_category {
                format!("{}_{}", cat, inner)
            } else {
                inner.to_string()
            };

            if let Some(values) = fetch_locale_with_context(&key, "en", context_category) {
                values.first().cloned().unwrap_or_else(|| s.to_string())
            } else {
                s.to_string()
            }
        };

        if resolved.starts_with("#{") && resolved.ends_with('}') {
            resolve_placeholder_recursive(&resolved, context_category, depth + 1, max_depth)
        } else {
            resolved
        }
    } else {
        s.to_string()
    }
}

fn extract_nested_value(
    data: &Value,
    key: &str,
    _context_category: Option<&str>,
) -> Option<String> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current: &Value = data;

    for part in parts.iter() {
        let part_lower = part.to_lowercase();
        match current {
            Value::Mapping(map) => {
                if let Some(next) = map.get(&Value::String(part_lower.clone())) {
                    current = next;
                } else if let Some(next) = map.get(&Value::String(part.to_string())) {
                    current = next;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    match current {
        Value::String(s) => Some(s.clone()),
        Value::Sequence(seq) if !seq.is_empty() => {
            if let Some(first) = seq.first() {
                match first {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn extract_array_from_value(data: &Value, key: &str) -> Option<Vec<String>> {
    // key can be like: "internet.user_agent.chrome" or "user_agent.chrome" or "faker.internet.user_agent.chrome"
    let parts: Vec<&str> = key.split('.').collect();

    // Determine where to start - find "faker" or the first real category
    let mut start_idx = 0;
    for (i, part) in parts.iter().enumerate() {
        if *part == "faker" {
            start_idx = i + 1;
            break;
        }
        if i > 0 && i < parts.len() - 1 {
            // If we find a reasonable category name (not faker), start from there
            if *part != "internet" && *part != "name" && *part != "address" && *part != "company" {
                // Keep looking
            }
        }
    }

    // Try to get to the "faker" level first
    let faker_map = match data {
        Value::Mapping(m) => {
            // Look for "faker" key
            if let Some(faker) = m.get(&Value::String("faker".to_string())) {
                faker
            } else {
                // No "faker" key, use the data as-is
                return extract_array_simple(data, key);
            }
        }
        _ => return None,
    };

    // Now navigate from faker through the rest of the path
    let mut current = faker_map;

    // Skip "internet" if present, or start from the right part
    let mut found_internet = false;
    for part in parts.iter() {
        if *part == "internet" {
            found_internet = true;
            continue;
        }
        if found_internet || start_idx > 0 {
            let part_str = part.to_string();
            current = match current {
                Value::Mapping(map) => {
                    if let Some(v) = map.get(&Value::String(part_str.clone())) {
                        v
                    } else {
                        // Try without "internet" prefix
                        return extract_array_simple(data, &parts[parts.len() - 1..].join("."));
                    }
                }
                _ => return None,
            };
        }
    }

    match current {
        Value::Sequence(seq) => {
            let arr: Vec<String> = seq
                .iter()
                .filter_map(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();
            if arr.is_empty() {
                None
            } else {
                Some(arr)
            }
        }
        _ => None,
    }
}

fn extract_array_simple(data: &Value, key: &str) -> Option<Vec<String>> {
    // Simplified version that just looks for the last key in the data
    let parts: Vec<&str> = key.split('.').collect();
    let last_key = parts.last().unwrap_or(&key);

    let faker_map = match data {
        Value::Mapping(m) => m.get(&Value::String("faker".to_string()))?,
        _ => return None,
    };

    match faker_map {
        Value::Mapping(faker_m) => {
            // Try each category in order until we find the key
            for category in ["internet", "name", "address", "company"] {
                if let Some(cat) = faker_m.get(&Value::String(category.to_string())) {
                    if let Value::Mapping(cat_m) = cat {
                        if let Some(result) = cat_m.get(&Value::String(last_key.to_string())) {
                            if let Value::Sequence(seq) = result {
                                return Some(
                                    seq.iter()
                                        .filter_map(|v| match v {
                                            Value::String(s) => Some(s.clone()),
                                            _ => None,
                                        })
                                        .collect(),
                                );
                            }
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn extract_array(data: &Value, key: &str) -> Option<Vec<String>> {
    let parts: Vec<&str> = key.split('.').collect();

    // The key can be:
    // - "faker.name.first_name" (with faker prefix)
    // - "name.first_name" (without faker prefix)
    // - "internet.user_agent.chrome" (nested category with vendor)

    // Find where to start - either at "faker" or at the first category
    let start_idx = if parts.first() == Some(&"faker") {
        1
    } else {
        0
    };

    // First, look for "faker" key in the data mapping
    // If not found (subdirectory files), just start from parts[0]
    let faker_map = match data {
        Value::Mapping(m) => {
            // Try to get faker first, if not, try the first part of the path
            if let Some(faker) = m.get(&Value::String("faker".to_string())) {
                faker
            } else if let Some(first) = m.get(&Value::String(parts[start_idx].to_string())) {
                first
            } else {
                m.get(&Value::String(parts[0].to_string()))?
            }
        }
        _ => return None,
    };

    let mut current = faker_map;

    // Now traverse the rest of the path
    for part in parts.iter().skip(start_idx) {
        let part_str = part.to_string();
        current = match current {
            Value::Mapping(map) => map.get(&Value::String(part_str))?,
            _ => return None,
        };
    }

    match current {
        Value::Sequence(seq) => {
            let arr: Vec<String> = seq
                .iter()
                .filter_map(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();
            if arr.is_empty() {
                None
            } else {
                Some(arr)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_locale() {
        let result = fetch_locale("faker.name.male_first_name", "en");
        assert!(result.is_some());
        let names = result.unwrap();
        assert!(!names.is_empty());
    }
}

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

/// Sample from locale data and resolve any placeholders in the result
pub fn sample_with_resolve(v: &[String], context_category: Option<&str>) -> String {
    if v.is_empty() {
        return String::new();
    }
    use crate::config::FakerConfig;
    let config = FakerConfig::current();
    let idx = config.rand_usize(v.len());
    let selected = &v[idx];
    resolve_placeholder(selected, context_category)
}

pub fn fetch_locale(key: &str, locale: &str) -> Option<Vec<String>> {
    fetch_locale_with_context(key, locale, None)
}

fn fetch_locale_no_resolve(
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
        // If we have context_category, try the subdir file directly with simple key
        if let Some(cat) = context_category {
            let subdir_key = format!("{}_{}", loc, cat);
            if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
                // Try with just the key directly
                if let Some(result) = try_extract(locale_data, loc, key, None) {
                    return Some(result);
                }
                // Try with faker prefix
                let faker_key = format!("faker.{}.{}", cat, key);
                if let Some(result) = try_extract(locale_data, loc, &faker_key, None) {
                    return Some(result);
                }
            }
        }
    }

    // Fallback to original fetch_locale_with_context for other cases
    fetch_locale_with_context(key, locale, context_category)
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
        // 1. Try full locale file (en.yml)
        if let Some(locale_data) = LOCALE_DATA.get(loc) {
            if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                return Some(result);
            }
        }

        // 2. Try subdirectory files (en_name.yml, en_address.yml, etc.)
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() >= 2 {
            let category = parts[0];
            let subdir_key = format!("{}_{}", loc, category);
            if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
                if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                    return Some(result);
                }
                let faker_key = format!("faker.{}", key);
                if let Some(result) = try_extract(locale_data, loc, &faker_key, context_category) {
                    return Some(result);
                }
                if let Some(result) = try_extract(
                    locale_data,
                    loc,
                    parts.last().unwrap_or(&key),
                    context_category,
                ) {
                    return Some(result);
                }
            }
        }

        // 3. NEW: If we have context_category and key is simple (like "city_prefix")
        // Try using the context_category as subdir directly
        if let Some(cat) = context_category {
            let subdir_key = format!("{}_{}", loc, cat);
            if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
                // Key is just the simple field name
                if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                    return Some(result);
                }
                // Try with faker prefix
                let faker_key = format!("faker.{}.{}", cat, key);
                if let Some(result) = try_extract(locale_data, loc, &faker_key, context_category) {
                    return Some(result);
                }
            }
        }

        // 4. Try fallback subdir with second part of key
        let subdir_key = format!("{}_{}", loc, key.split('.').nth(1).unwrap_or("name"));
        if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
            // Try with original key
            if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                return Some(result);
            }
            // Also try with just the last part
            if let Some(result) = try_extract(
                locale_data,
                loc,
                parts.last().unwrap_or(&key),
                context_category,
            ) {
                return Some(result);
            }
            // Also try with category prefix
            if parts.len() >= 2 {
                let category = parts[0];
                let full_key = format!("{}.{}", category, parts.last().unwrap_or(&key));
                if let Some(result) = try_extract(locale_data, loc, &full_key, context_category) {
                    return Some(result);
                }
            }
        }

        // 4. For simple keys without category (e.g., "city_prefix" with context "address")
        // Try to find the subdir directly
        if let Some(cat) = context_category {
            let subdir_key = format!("{}_{}", loc, cat);
            if let Some(locale_data) = LOCALE_DATA.get(&subdir_key) {
                // Try simple field name
                if let Some(result) = try_extract(locale_data, loc, key, context_category) {
                    return Some(result);
                }
                // Try with faker prefix
                let faker_key = format!("faker.{}.{}", cat, key);
                if let Some(result) = try_extract(locale_data, loc, &faker_key, context_category) {
                    return Some(result);
                }
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
        // Case 1: en.faker.name.first_name (file starts with locale name)
        if let Some(inner) = top.get(&Value::String(locale.to_string())) {
            if let Value::Mapping(inner_map) = inner {
                if let Some(faker) = inner_map.get(&Value::String("faker".to_string())) {
                    let clean_key = clean_key_for_extraction(key);
                    return extract_nested_array_raw(faker, &clean_key);
                }
            }
        }

        // Case 2: { "faker": {...} } (subdirectory files like en_name.yml)
        if let Some(faker) = top.get(&Value::String("faker".to_string())) {
            let clean_key = clean_key_for_extraction(key);
            return extract_nested_array_raw(faker, &clean_key);
        }

        // Case 3: en.faker.name.first_name -> try without faker prefix
        if let Some(inner) = top.get(&Value::String(locale.to_string())) {
            if let Value::Mapping(inner_map) = inner {
                let parts: Vec<&str> = key.split('.').collect();
                if parts.len() >= 2 {
                    let category = parts[0].to_lowercase();
                    if let Some(cat_data) = inner_map.get(&Value::String(category.clone())) {
                        if let Value::Mapping(cat_map) = cat_data {
                            let field = if parts.len() > 1 { parts[1] } else { "" };
                            if let Some(field_data) = cat_map.get(&Value::String(field.to_string()))
                            {
                                if let Value::Sequence(seq) = field_data {
                                    return extract_strings_raw(seq);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn clean_key_for_extraction(key: &str) -> String {
    if key.starts_with("faker.") {
        key[6..].to_string()
    } else if key.starts_with("faker_") {
        key[6..].replace('_', ".")
    } else if key.contains("faker.") {
        key.replace("faker.", "").replace('_', ".")
    } else {
        key.replace('_', ".")
    }
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
        let part_lower = part_str.to_lowercase();
        match current {
            Value::Mapping(map) => {
                // Try exact match first, then case-insensitive
                if let Some(next) = map.get(&Value::String(part_str.clone())) {
                    current = next;
                } else if let Some(next) = map.get(&Value::String(part_lower)) {
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

fn extract_strings_from_seq(seq: &[Value], _context_category: Option<&str>) -> Option<Vec<String>> {
    // Return raw strings without resolving placeholders - placeholders will be resolved later
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

fn extract_nested_array_raw(data: &Value, key: &str) -> Option<Vec<String>> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current: &Value = data;

    for (i, part) in parts.iter().enumerate() {
        let part_str = part.to_string();
        let part_lower = part_str.to_lowercase();
        match current {
            Value::Mapping(map) => {
                if let Some(next) = map.get(&Value::String(part_str.clone())) {
                    current = next;
                } else if let Some(next) = map.get(&Value::String(part_lower)) {
                    current = next;
                } else {
                    return None;
                }
            }
            Value::Sequence(seq) => {
                if i == parts.len() - 1 {
                    return extract_strings_raw(seq);
                }
                return None;
            }
            _ => return None,
        }
    }

    match current {
        Value::Sequence(seq) => extract_strings_raw(seq),
        _ => None,
    }
}

fn extract_strings_raw(seq: &[Value]) -> Option<Vec<String>> {
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

fn resolve_placeholder(s: &str, context_category: Option<&str>) -> String {
    resolve_multiple(s, context_category, 0, 10)
}

fn resolve_multiple(
    s: &str,
    context_category: Option<&str>,
    depth: usize,
    max_depth: usize,
) -> String {
    if depth >= max_depth {
        return s.to_string();
    }

    if !s.contains("#{") {
        return s.to_string();
    }

    // Track resolution to detect loops
    let original = s.to_string();
    let mut result = original.clone();
    let mut changed = true;

    while changed && depth < max_depth {
        changed = false;

        while let Some(start_byte) = result.find("#{") {
            let rest = &result[start_byte + 2..];
            let mut brace_depth = 1;
            let mut end_byte = start_byte + 2;

            for (char_idx, c) in rest.char_indices() {
                if c == '{' {
                    brace_depth += 1;
                } else if c == '}' {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        end_byte = start_byte + 2 + char_idx + 1;
                        break;
                    }
                }
            }

            if brace_depth == 0 {
                let placeholder = &result[start_byte..end_byte];
                if let Some(inner) = placeholder.get(2..placeholder.len() - 1) {
                    let replacement = resolve_inner(inner, context_category, depth + 1);
                    // Check if we got back the same placeholder (infinite loop)
                    if replacement == placeholder {
                        result = result.replace(placeholder, &format!("[UNRESOLVED:{}]", inner));
                    } else {
                        result = result.replace(placeholder, &replacement);
                    }
                    changed = true;
                    break;
                }
            } else {
                break;
            }
        }
    }

    result
}

fn resolve_inner(inner: &str, context_category: Option<&str>, depth: usize) -> String {
    if depth > 3 {
        return format!("[{}]", inner);
    }

    // Handle "Name.first_name" or "name.first_name" notation
    if inner.contains('.') {
        let parts: Vec<&str> = inner.split('.').collect();
        if parts.len() >= 2 {
            let cat = parts[0].to_lowercase();
            let field = parts[1].to_lowercase();

            // Use fetch_locale_with_context to get proper context lookup
            if let Some(result) = fetch_locale_with_context(&field, "en", Some(&cat)) {
                if !result.is_empty() {
                    use crate::config::FakerConfig;
                    let config = FakerConfig::current();
                    let idx = config.rand_usize(result.len());
                    let v = result[idx].clone();
                    return resolve_multiple(&v, context_category, depth + 1, 10);
                }
            }
        }
    } else {
        // Simple key (no dot notation) - try multiple common contexts
        // to find where this key belongs
        let contexts_to_try: Vec<&str> = if let Some(cat) = context_category {
            vec![cat, "name", "address", "company", "internet"]
        } else {
            vec!["name", "address", "company", "internet"]
        };

        for cat in contexts_to_try {
            if let Some(result) = fetch_locale_with_context(inner, "en", Some(cat)) {
                if !result.is_empty() {
                    use crate::config::FakerConfig;
                    let config = FakerConfig::current();
                    let idx = config.rand_usize(result.len());
                    let v = result[idx].clone();
                    return resolve_multiple(&v, context_category, depth + 1, 10);
                }
            }
        }
    }

    inner.to_string()
}

fn resolve_simple(s: &str, context_category: Option<&str>) -> String {
    // If no placeholder, return as-is
    if !(s.starts_with("#{") && s.ends_with('}')) {
        return s.to_string();
    }

    // Extract inner part
    let inner = &s[2..s.len() - 1];

    // Handle dot notation like "name.first_name" or "Name.first_name"
    if inner.contains('.') {
        let parts: Vec<&str> = inner.split('.').collect();
        if parts.len() >= 2 {
            let cat = parts[0].to_lowercase();
            let field = parts[1].to_lowercase();
            let lookup_key = format!("{}_{}", cat, field);

            if let Some(values) = fetch_locale_with_context(&lookup_key, "en", None) {
                return values.first().cloned().unwrap_or_else(|| inner.to_string());
            }

            // Try en_name.yml, en_address.yml directly
            let en_cat_key = format!("en_{}", cat);
            if let Some(subdata) = LOCALE_DATA.get(&en_cat_key) {
                if let Some(v) = extract_direct_value(subdata, &field) {
                    return v;
                }
            }
        }
    }

    // Simple case - try with context
    if let Some(cat) = context_category {
        // First, try just the inner key directly (not prefixed) - but without recursive resolution
        if let Some(values) = fetch_locale_no_resolve(inner, "en", Some(cat)) {
            return values.first().cloned().unwrap_or_else(|| inner.to_string());
        }
    }

    // Try without context
    if let Some(values) = fetch_locale_with_context(inner, "en", None) {
        return values.first().cloned().unwrap_or_else(|| inner.to_string());
    }

    inner.to_string()
}

fn extract_direct_value(data: &Value, field: &str) -> Option<String> {
    if let Value::Mapping(top) = data {
        // Try en.faker.category.field
        if let Some(en) = top.get(&Value::String("en".to_string())) {
            if let Value::Mapping(en_map) = en {
                if let Some(faker) = en_map.get(&Value::String("faker".to_string())) {
                    if let Value::Mapping(faker_m) = faker {
                        for (_, cat_val) in faker_m.iter() {
                            if let Value::Mapping(cat_m) = cat_val {
                                if let Some(field_val) =
                                    cat_m.get(&Value::String(field.to_string()))
                                {
                                    if let Value::Sequence(seq) = field_val {
                                        if let Some(first) = seq.first() {
                                            if let Value::String(s) = first {
                                                return Some(s.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Try direct faker.category.field
        if let Some(faker) = top.get(&Value::String("faker".to_string())) {
            if let Value::Mapping(faker_m) = faker {
                for (_, cat_val) in faker_m.iter() {
                    if let Value::Mapping(cat_m) = cat_val {
                        if let Some(field_val) = cat_m.get(&Value::String(field.to_string())) {
                            if let Value::Sequence(seq) = field_val {
                                if let Some(first) = seq.first() {
                                    if let Value::String(s) = first {
                                        return Some(s.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
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

    // Handle multiple placeholders like "#{foo}#{bar}"
    if s.contains("#{") {
        if depth >= max_depth {
            return s.to_string();
        }

        let mut result = s.to_string();
        // Keep replacing until no more placeholders or max depth
        loop {
            let placeholder_start = result.find("#{");
            if placeholder_start.is_none() {
                break;
            }
            let after_start = &result[placeholder_start.unwrap()..];
            let placeholder_end = after_start.find('}');
            if placeholder_end.is_none() {
                break;
            }

            let full_placeholder = &after_start[..placeholder_end.unwrap() + 1];
            let inner = &full_placeholder[2..full_placeholder.len() - 1];

            let replacement = resolve_single_placeholder(inner, context_category, depth, max_depth);

            // Check if replacement still contains placeholders and we can recurse
            if replacement.contains("#{") && depth < max_depth - 1 {
                let resolved = resolve_placeholder_recursive(
                    &replacement,
                    context_category,
                    depth + 1,
                    max_depth,
                );
                result = result.replace(full_placeholder, &resolved);
            } else {
                result = result.replace(full_placeholder, &replacement);
            }

            // Safety check to prevent infinite loops
            if depth > 5 {
                break;
            }
        }
        return result;
    }

    // Handle single placeholder
    if s.starts_with("#{") && s.ends_with('}') {
        let inner = &s[2..s.len() - 1];
        resolve_single_placeholder(inner, context_category, depth, max_depth)
    } else {
        s.to_string()
    }
}

fn resolve_single_placeholder(
    inner: &str,
    context_category: Option<&str>,
    depth: usize,
    max_depth: usize,
) -> String {
    // Handle cases like "Name.first_name" or "name.first_name"
    let parts: Vec<&str> = inner.split('.').collect();
    if parts.len() >= 2 {
        let cat = parts[0].to_lowercase();
        let field = parts[1].to_lowercase();
        let lookup_key = format!("{}_{}", cat, field);

        if let Some(values) = fetch_locale_with_context(&lookup_key, "en", None) {
            if let Some(v) = values.first() {
                if depth < max_depth - 1 && v.contains("#{") {
                    return resolve_placeholder_recursive(
                        &v,
                        context_category,
                        depth + 1,
                        max_depth,
                    );
                }
                return v.to_string();
            }
        }

        let en_cat_key = format!("en_{}", cat);
        if let Some(subdata) = LOCALE_DATA.get(&en_cat_key) {
            if let Some(v) = extract_nested_value_simple(subdata, &field) {
                if depth < max_depth - 1 && v.contains("#{") {
                    return resolve_placeholder_recursive(
                        &v,
                        context_category,
                        depth + 1,
                        max_depth,
                    );
                }
                return v;
            }
        }

        let faker_lookup = format!("faker.{}.{}", cat, field);
        if let Some(values) = fetch_locale(&faker_lookup, "en") {
            if let Some(v) = values.first() {
                if depth < max_depth - 1 && v.contains("#{") {
                    return resolve_placeholder_recursive(
                        v,
                        context_category,
                        depth + 1,
                        max_depth,
                    );
                }
                return v.to_string();
            }
        }
    }

    // Simple case like "first_name", "city_prefix"
    if let Some(cat) = context_category {
        let key = format!("{}_{}", cat, inner);
        if let Some(values) = fetch_locale_with_context(&key, "en", Some(cat)) {
            if let Some(v) = values.first() {
                if depth < max_depth - 1 && v.contains("#{") {
                    return resolve_placeholder_recursive(
                        v,
                        context_category,
                        depth + 1,
                        max_depth,
                    );
                }
                return v.to_string();
            }
        }
    }

    if let Some(values) = fetch_locale_with_context(inner, "en", None) {
        if let Some(v) = values.first() {
            if depth < max_depth - 1 && v.contains("#{") {
                return resolve_placeholder_recursive(v, context_category, depth + 1, max_depth);
            }
            return v.to_string();
        }
    }

    inner.to_string()
}

fn extract_nested_value_simple(data: &Value, field: &str) -> Option<String> {
    if let Value::Mapping(top) = data {
        // Check for locale prefix (en)
        if let Some(en) = top.get(&Value::String("en".to_string())) {
            if let Value::Mapping(en_map) = en {
                // Check for faker
                if let Some(faker) = en_map.get(&Value::String("faker".to_string())) {
                    if let Value::Mapping(faker_m) = faker {
                        // Try each category
                        for (_, cat_val) in faker_m.iter() {
                            if let Value::Mapping(cat_m) = cat_val {
                                if let Some(field_val) =
                                    cat_m.get(&Value::String(field.to_string()))
                                {
                                    if let Value::Sequence(seq) = field_val {
                                        if let Some(first) = seq.first() {
                                            if let Value::String(s) = first {
                                                return Some(s.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Try direct faker key
        if let Some(faker) = top.get(&Value::String("faker".to_string())) {
            if let Value::Mapping(faker_m) = faker {
                for (_, cat_val) in faker_m.iter() {
                    if let Value::Mapping(cat_m) = cat_val {
                        if let Some(field_val) = cat_m.get(&Value::String(field.to_string())) {
                            if let Value::Sequence(seq) = field_val {
                                if let Some(first) = seq.first() {
                                    if let Value::String(s) = first {
                                        return Some(s.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
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

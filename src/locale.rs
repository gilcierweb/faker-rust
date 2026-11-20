//! Locale data loader - loads and provides access to locale YAML files
#![allow(clippy::collapsible_match)]
#![allow(clippy::manual_strip)]
use once_cell::sync::Lazy;
use serde_json::Value;
use serde_saphyr::{DuplicateKeyPolicy, from_str_with_options, options};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Parse a locale YAML document into a generic value tree.
///
/// Uses serde-saphyr (pure safe Rust) instead of the deprecated `serde_yaml`.
/// Lenient duplicate-key handling mirrors the previous `serde_yaml` behavior
/// for user-provided locale files.
fn parse_locale_yaml(contents: &str) -> Option<Value> {
    let parse_options = options! {
        duplicate_keys: DuplicateKeyPolicy::LastWins,
    };
    from_str_with_options(contents, parse_options).ok()
}

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

            if path.is_file()
                && path.extension().is_some_and(|ext| ext == "yml")
                && let Some(stem) = path.file_stem()
            {
                let locale_name = stem.to_string_lossy().to_string();
                load_locale_file(&mut result, &path, &locale_name);
            }

            if path.is_dir() {
                load_locale_subdir(&mut result, &path);
            }
        }
    }

    result
}

/// Read and parse one locale YAML file into the result map under `locale_name`.
///
/// Unreadable or unparseable files are skipped silently, preserving the
/// behavior of partial locale directories.
fn load_locale_file(result: &mut HashMap<String, Value>, path: &Path, locale_name: &str) {
    if let Ok(contents) = fs::read_to_string(path)
        && let Some(yaml) = parse_locale_yaml(&contents)
    {
        result.insert(locale_name.to_string(), yaml);
    }
}

/// Load every `.yml` file of a locale subdirectory into the result map,
/// keyed by `"<dir>_<stem>"` (e.g. `locales/en/name.yml` becomes `en_name`).
fn load_locale_subdir(result: &mut HashMap<String, Value>, dir: &Path) {
    let dir_name = dir
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default();

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && path.extension().is_some_and(|ext| ext == "yml")
            && let Some(stem) = path.file_stem()
        {
            let locale_name = format!("{}_{}", dir_name, stem.to_string_lossy());
            load_locale_file(result, &path, &locale_name);
        }
    }
}

/// Sample from locale data and resolve any placeholders in the result
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

pub fn fetch_locale_with_context(
    key: &str,
    locale: &str,
    context_category: Option<&str>,
) -> Option<Vec<String>> {
    let locales_to_try: Vec<String> = if locale.contains('_') || locale.contains('-') {
        let short = locale.split(['-', '_']).next().unwrap();
        vec![locale.to_string(), short.to_string()]
    } else {
        vec![locale.to_string()]
    };

    for loc in locales_to_try.iter() {
        // 1. Try full locale file (en.yml)
        if let Some(locale_data) = LOCALE_DATA.get(loc)
            && let Some(result) = try_extract(locale_data, loc, key, context_category)
        {
            return Some(result);
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

        // 3. If we have a context_category, try using it as the subdir directly.
        if let Some(cat) = context_category {
            if let Some(result) = try_subdir(loc, cat, key, context_category) {
                return Some(result);
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

    }

    if locale != "en" {
        return fetch_locale_with_context(key, "en", context_category);
    }

    None
}

/// Try resolving `key` (and its `faker.`-prefixed form) from the subdir file
/// named `<loc>_<cat>.yml`.
fn try_subdir(loc: &str, cat: &str, key: &str, context_category: Option<&str>) -> Option<Vec<String>> {
    let subdir_key = format!("{}_{}", loc, cat);
    let locale_data = LOCALE_DATA.get(&subdir_key)?;
    if let Some(result) = try_extract(locale_data, loc, key, context_category) {
        return Some(result);
    }
    let faker_key = format!("faker.{}.{}", cat, key);
    try_extract(locale_data, loc, &faker_key, context_category)
}

fn try_extract(
    locale_data: &Value,
    locale: &str,
    key: &str,
    _context_category: Option<&str>,
) -> Option<Vec<String>> {
    if let Value::Object(top) = locale_data {
        // Case 1: en.faker.name.first_name (file starts with locale name)
        if let Some(inner) = top.get(locale)
            && let Value::Object(inner_map) = inner
            && let Some(faker) = inner_map.get("faker")
        {
            let clean_key = clean_key_for_extraction(key);
            return extract_nested_array_raw(faker, &clean_key);
        }

        // Case 2: { "faker": {...} } (subdirectory files like en_name.yml)
        if let Some(faker) = top.get("faker") {
            let clean_key = clean_key_for_extraction(key);
            return extract_nested_array_raw(faker, &clean_key);
        }

        // Case 3: en.faker.name.first_name -> try without faker prefix
        if let Some(inner) = top.get(locale)
            && let Value::Object(inner_map) = inner
        {
            let parts: Vec<&str> = key.split('.').collect();
            if parts.len() >= 2 {
                let category = parts[0].to_lowercase();
                if let Some(cat_data) = inner_map.get(category.as_str())
                    && let Value::Object(cat_map) = cat_data
                {
                    let field = if parts.len() > 1 { parts[1] } else { "" };
                    if let Some(field_data) = cat_map.get(field) {
                        if let Value::Array(seq) = field_data {
                            return extract_strings_raw(seq);
                        }
                        if let Value::String(s) = field_data {
                            return Some(vec![s.clone()]);
                        }
                    }
                }
            }
        }
    }

    None
}

fn clean_key_for_extraction(key: &str) -> String {
    // Strip a leading "faker." prefix only. Underscores inside segment names
    // (e.g. male_first_name, elder_scrolls) are significant and must not be
    // rewritten into path separators.
    if let Some(rest) = key.strip_prefix("faker.") {
        rest.to_string()
    } else {
        key.replace("faker.", "")
    }
}

fn extract_nested_array_raw(data: &Value, key: &str) -> Option<Vec<String>> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current: &Value = data;

    for (i, part) in parts.iter().enumerate() {
        let part_str = part.to_string();
        let part_lower = part_str.to_lowercase();
        match current {
            Value::Object(map) => {
                if let Some(next) = map.get(part_str.as_str()) {
                    current = next;
                } else {
                    let next = map.get(part_lower.as_str())?;
                    current = next;
                }
            }
            Value::Array(seq) => {
                if i == parts.len() - 1 {
                    return extract_strings_raw(seq);
                }
                return None;
            }
            Value::String(s) => {
                if i == parts.len() - 1 {
                    return Some(vec![s.clone()]);
                }
                return None;
            }
            _ => return None,
        }
    }

    match current {
        Value::Array(seq) => extract_strings_raw(seq),
        Value::String(s) => Some(vec![s.clone()]),
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
    if arr.is_empty() { None } else { Some(arr) }
}

fn resolve_placeholder(s: &str, context_category: Option<&str>) -> String {
    let lookup = |key: &str, ctx: Option<&str>| {
        if let Some(v) = fetch_locale_with_context(key, "en", ctx) {
            return Some(v);
        }
        if let Some(cat) = ctx {
            let en_cat_key = format!("en_{}", cat);
            if let Some(subdata) = LOCALE_DATA.get(&en_cat_key)
                && let Some(v) = extract_direct_value(subdata, key)
            {
                return Some(vec![v]);
            }
            let faker_lookup = format!("faker.{}.{}", cat, key);
            if let Some(v) = fetch_locale(&faker_lookup, "en") {
                return Some(v);
            }
        }
        None
    };
    let mut seen = HashSet::new();
    resolve_multiple(&lookup, s, context_category, 0, 10, &mut seen)
}

/// Pure, testable entry point for placeholder resolution.
///
/// `lookup` resolves a key (and optional category context) to a list of
/// candidate strings. Cycle detection is built in: a placeholder whose inner
/// key is already on the resolution stack resolves to `[UNRESOLVED:<inner>]`
/// instead of recursing forever.
pub fn resolve_placeholder_with<F>(lookup: F, s: &str, context_category: Option<&str>) -> String
where
    F: Fn(&str, Option<&str>) -> Option<Vec<String>>,
{
    let mut seen = HashSet::new();
    resolve_multiple(&lookup, s, context_category, 0, 10, &mut seen)
}

fn resolve_multiple<F>(
    lookup: &F,
    s: &str,
    context_category: Option<&str>,
    depth: usize,
    max_depth: usize,
    seen: &mut HashSet<String>,
) -> String
where
    F: Fn(&str, Option<&str>) -> Option<Vec<String>>,
{
    if depth >= max_depth {
        return s.to_string();
    }

    if !s.contains("#{") {
        return s.to_string();
    }

    let mut result = s.to_string();
    let mut changed = true;
    let mut iterations = 0;

    while changed && iterations < max_depth {
        changed = false;
        iterations += 1;

        let Some(start_byte) = result.find("#{") else {
            break;
        };
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
                if seen.contains(inner) {
                    result = result.replace(placeholder, &format!("[UNRESOLVED:{}]", inner));
                    changed = true;
                } else {
                    seen.insert(inner.to_string());
                    let replacement =
                        resolve_inner(lookup, inner, context_category, depth + 1, max_depth, seen);
                    seen.remove(inner);
                    if replacement == placeholder {
                        result = result.replace(placeholder, &format!("[UNRESOLVED:{}]", inner));
                    } else {
                        result = result.replace(placeholder, &replacement);
                    }
                    changed = true;
                }
            }
        } else {
            break;
        }
    }

    result
}

fn resolve_inner<F>(
    lookup: &F,
    inner: &str,
    context_category: Option<&str>,
    depth: usize,
    max_depth: usize,
    seen: &mut HashSet<String>,
) -> String
where
    F: Fn(&str, Option<&str>) -> Option<Vec<String>>,
{
    if depth > 3 {
        return format!("[{}]", inner);
    }

    // Handle "Name.first_name" or "name.first_name" notation
    if inner.contains('.') {
        let parts: Vec<&str> = inner.split('.').collect();
        if parts.len() >= 2 {
            let cat = parts[0].to_lowercase();
            let field = parts[1].to_lowercase();

            if let Some(result) = lookup(&field, Some(&cat))
                && !result.is_empty()
            {
                use crate::config::FakerConfig;
                let config = FakerConfig::current();
                let idx = config.rand_usize(result.len());
                let v = result[idx].clone();
                if v.contains("#{") {
                    return resolve_multiple(
                        lookup,
                        &v,
                        context_category,
                        depth + 1,
                        max_depth,
                        seen,
                    );
                }
                return v;
            }
        }
    } else {
        // Simple key (no dot notation) - try multiple common contexts
        let contexts_to_try: Vec<&str> = if let Some(cat) = context_category {
            vec![cat, "name", "address", "company", "internet"]
        } else {
            vec!["name", "address", "company", "internet"]
        };

        for cat in contexts_to_try {
            if let Some(result) = lookup(inner, Some(cat))
                && !result.is_empty()
            {
                use crate::config::FakerConfig;
                let config = FakerConfig::current();
                let idx = config.rand_usize(result.len());
                let v = result[idx].clone();
                if v.contains("#{") {
                    return resolve_multiple(
                        lookup,
                        &v,
                        context_category,
                        depth + 1,
                        max_depth,
                        seen,
                    );
                }
                return v;
            }
        }
    }

    inner.to_string()
}

fn extract_direct_value(data: &Value, field: &str) -> Option<String> {
    if let Value::Object(top) = data {
        // Try en.faker.category.field
        if let Some(en) = top.get("en")
            && let Value::Object(en_map) = en
            && let Some(faker) = en_map.get("faker")
            && let Value::Object(faker_m) = faker
        {
            for (_, cat_val) in faker_m.iter() {
                if let Value::Object(cat_m) = cat_val
                    && let Some(field_val) = cat_m.get(field)
                    && let Value::Array(seq) = field_val
                    && let Some(first) = seq.first()
                    && let Value::String(s) = first
                {
                    return Some(s.clone());
                }
            }
        }

        // Try direct faker.category.field
        if let Some(faker) = top.get("faker")
            && let Value::Object(faker_m) = faker
        {
            for (_, cat_val) in faker_m.iter() {
                if let Value::Object(cat_m) = cat_val
                    && let Some(field_val) = cat_m.get(field)
                    && let Value::Array(seq) = field_val
                    && let Some(first) = seq.first()
                    && let Value::String(s) = first
                {
                    return Some(s.clone());
                }
            }
        }
    }
    None
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

    // Characterization tests: pin the behavior of the YAML locale loader
    // so the serde_yaml -> serde-saphyr swap is provably behavior-preserving.

    #[test]
    fn test_fetch_locale_subdir_file() {
        // "name.first_name" resolves through the en_name.yml subdir file.
        let result = fetch_locale("name.first_name", "en");
        assert!(result.is_some());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_fetch_locale_with_context_category() {
        let result = fetch_locale_with_context("first_name", "en", Some("name"));
        assert!(result.is_some());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_fetch_locale_subdir_key_is_case_sensitive() {
        // Pins current behavior: the subdir file key (e.g. "en_name") is built
        // from the raw key segment, so a mixed-case segment misses.
        assert!(fetch_locale("faker.Name.male_first_name", "en").is_none());
    }

    #[test]
    fn test_fetch_locale_missing_returns_none() {
        assert!(fetch_locale("definitely.not.a.real.key", "en").is_none());
        assert!(fetch_locale_with_context("nope", "en", Some("nothing")).is_none());
    }

    #[test]
    fn test_resolve_placeholder_expands() {
        let resolved = resolve_placeholder("#{Name.first_name}", Some("name"));
        assert!(
            !resolved.contains("#{"),
            "placeholder should be expanded, got: {resolved}"
        );
        assert!(
            !resolved.starts_with("[UNRESOLVED"),
            "placeholder should resolve, got: {resolved}"
        );
        assert!(!resolved.is_empty());
    }

    #[test]
    fn test_sample_with_resolve() {
        assert!(sample_with_resolve(&[], None).is_empty());
        let picked = sample_with_resolve(&["alpha".to_string(), "beta".to_string()], None);
        assert!(picked == "alpha" || picked == "beta");
    }

    #[test]
    fn test_clean_key_preserves_underscores() {
        assert_eq!(clean_key_for_extraction("faker.elder_scrolls.race"), "elder_scrolls.race");
        assert_eq!(clean_key_for_extraction("elder_scrolls.race"), "elder_scrolls.race");
        assert_eq!(clean_key_for_extraction("faker.name.male_first_name"), "name.male_first_name");
    }

    #[test]
    fn test_fetch_locale_scalar_value() {
        // book.author is a scalar placeholder in the YAML, not a list.
        assert!(fetch_locale_with_context("book.author", "en", Some("book")).is_some());
    }

    #[test]
    fn test_resolve_placeholder_terminates_on_cycle() {
        use std::collections::HashMap;
        crate::config::FakerConfig::set_seed(0);
        let data = [
            ("a".to_string(), vec!["#{b}".to_string()]),
            ("b".to_string(), vec!["#{a}".to_string()]),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        let lookup = |k: &str, _: Option<&str>| data.get(k).cloned();
        let out = resolve_placeholder_with(lookup, "#{a}", Some("name"));
        assert!(
            !out.contains("#{"),
            "cycle must be marked unresolved, got {out}"
        );
        assert!(out.contains("[UNRESOLVED:"), "got {out}");
    }

    #[test]
    fn test_resolve_placeholder_terminates_on_self_reference() {
        use std::collections::HashMap;
        crate::config::FakerConfig::set_seed(0);
        let data = [("a".to_string(), vec!["prefix#{a}suffix".to_string()])]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let lookup = |k: &str, _: Option<&str>| data.get(k).cloned();
        let out = resolve_placeholder_with(lookup, "#{a}", Some("name"));
        assert!(
            !out.contains("#{"),
            "self-reference must be marked, got {out}"
        );
    }

    #[test]
    fn test_load_locale_data_keys_and_shape() {
        // Pins the loader contract before the DRY refactor: top-level files are
        // keyed by stem, subdirectory files by "<dir>_<stem>", values are maps.
        let data = &*LOCALE_DATA;
        assert!(data.contains_key("en"), "top-level locale file key missing");
        assert!(
            data.contains_key("en_name"),
            "subdir locale file key missing"
        );
        assert!(
            matches!(data.get("en_name"), Some(Value::Object(_))),
            "en_name must parse as a mapping"
        );
    }
}

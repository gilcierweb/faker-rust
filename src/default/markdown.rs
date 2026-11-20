//! Markdown generator - generates markdown-formatted text

use crate::config::FakerConfig;

/// Generate a random markdown header
pub fn headers() -> String {
    let config = FakerConfig::current();
    let level = config.rand_range(1, 7);
    let prefix = "#".repeat(level as usize);
    format!("{} Header", prefix)
}

/// Generate a random markdown emphasis
pub fn emphasis() -> String {
    let emphasis_types = vec![
        "*italic*",
        "**bold**",
        "***bold italic***",
        "~~strikethrough~~",
        "`code`",
    ];
    crate::base::sample(&emphasis_types).to_string()
}

/// Generate a random markdown table row
pub fn table_row() -> String {
    "| Cell 1 | Cell 2 | Cell 3 |".to_string()
}

/// Generate a random inline code
pub fn inline_code() -> String {
    let code_snippets = vec![
        "`foo()`",
        "`bar = 1`",
        "`println!`",
        "`cargo build`",
        "`git commit`",
    ];
    crate::base::sample(&code_snippets).to_string()
}

/// Generate a random block code
pub fn block_code() -> String {
    "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```".to_string()
}

// --- gem parity generators (algorithmic) ---

fn list(marker: &str) -> String {
    let count = crate::config::FakerConfig::current().rand_range(1, 11) as usize;
    let mut lines = String::new();
    for index in 0..count {
        let prefix = if marker == "*" {
            marker.to_string()
        } else {
            format!("{index}.")
        };
        lines.push_str(&format!("{prefix} {} \n", crate::lorem::sentence(Some(1))));
    }
    lines
}

/// Gem-compatible generator: Markdown ordered list.
pub fn ordered_list() -> String {
    list("1.")
}

/// Gem-compatible generator: Markdown unordered list.
pub fn unordered_list() -> String {
    list("*")
}

/// Gem-compatible generator: Markdown table with a row of headings and separator.
pub fn table() -> String {
    let mut rows: Vec<String> = (0..3)
        .map(|_| {
            format!(
                "{} | {} | {}",
                crate::lorem::word(),
                crate::lorem::word(),
                crate::lorem::word()
            )
        })
        .collect();
    rows.insert(1, "---- | ---- | ----".to_string());
    rows.join("\n")
}

/// Gem-compatible generator: one random Markdown generator, excluding named ones.
pub fn random(exclude: &[&str]) -> String {
    let names = [
        "block_code",
        "emphasis",
        "headers",
        "inline_code",
        "ordered_list",
        "table",
        "unordered_list",
    ];
    let choices: Vec<usize> = (0..names.len())
        .filter(|index| !exclude.contains(&names[*index]))
        .collect();
    if choices.is_empty() {
        return headers();
    }
    match crate::base::sample(&choices) {
        0 => block_code(),
        1 => emphasis(),
        2 => headers(),
        3 => inline_code(),
        4 => ordered_list(),
        5 => table(),
        _ => unordered_list(),
    }
}

/// Gem-compatible generator: header followed by repeated paragraph and element blocks.
pub fn sandwich(sentences: usize, repeat: usize) -> String {
    let mut blocks = vec![headers()];
    for _ in 0..repeat {
        blocks.push(crate::lorem::paragraph(Some(sentences)));
        blocks.push(random(&[]));
    }
    blocks.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        assert!(!headers().is_empty());
    }

    #[test]
    fn test_emphasis() {
        assert!(!emphasis().is_empty());
    }

    #[test]
    fn test_table_row() {
        assert!(!table_row().is_empty());
    }

    #[test]
    fn test_inline_code() {
        assert!(!inline_code().is_empty());
    }

    #[test]
    fn test_block_code() {
        assert!(!block_code().is_empty());
    }
}

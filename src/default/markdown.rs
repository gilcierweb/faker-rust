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
        "`foo()`", "`bar = 1`", "`println!`", "`cargo build`", "`git commit`",
    ];
    crate::base::sample(&code_snippets).to_string()
}

/// Generate a random block code
pub fn block_code() -> String {
    "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```".to_string()
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

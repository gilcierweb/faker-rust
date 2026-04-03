//! Source generator - for source code related fake data

use crate::base::sample;
use crate::config::FakerConfig;

/// Generate a random git branch name
pub fn git_branch() -> String {
    let prefixes = ["feature", "bugfix", "hotfix", "release", "develop"];
    let prefix = sample(&prefixes);
    let config = FakerConfig::current();
    let number = config.rand_range(1, 1000);
    let words = ["login", "auth", "api", "ui", "fix", "update", "refactor"];
    let word = sample(&words);
    
    format!("{}/{}-{}", prefix, word, number)
}

/// Generate a random git commit message
pub fn git_commit_message() -> String {
    let messages = [
        "Fix bug in authentication",
        "Add new feature",
        "Update documentation",
        "Refactor code",
        "Fix typo",
        "Initial commit",
        "Merge pull request",
        "Update dependencies",
    ];
    sample(&messages).to_string()
}

/// Generate a random version number
pub fn version() -> String {
    let config = FakerConfig::current();
    let major = config.rand_range(0, 10);
    let minor = config.rand_range(0, 20);
    let patch = config.rand_range(0, 100);
    format!("{}.{}.{}", major, minor, patch)
}

/// Generate a random semantic version
pub fn semver() -> String {
    version()
}

/// Generate a random filename with extension
pub fn filename() -> String {
    let names = [
        "main", "index", "app", "config", "utils", "helpers",
        "test", "spec", "README", "LICENSE", "package", "Cargo",
    ];
    let extensions = [
        "rs", "js", "ts", "py", "rb", "java", "go", "html", "css",
        "json", "yaml", "toml", "md", "txt",
    ];
    format!("{}.{}", sample(&names), sample(&extensions))
}

/// Generate a random directory name
pub fn directory() -> String {
    let dirs = [
        "src", "lib", "bin", "test", "docs", "config", "assets",
        "public", "private", "vendor", "node_modules", "target",
    ];
    sample(&dirs).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_branch() {
        let branch = git_branch();
        assert!(!branch.is_empty());
        assert!(branch.contains('/') || branch == "master" || branch == "main");
    }

    #[test]
    fn test_git_commit_message() {
        assert!(!git_commit_message().is_empty());
    }

    #[test]
    fn test_version() {
        let v = version();
        assert!(v.contains('.'));
    }

    #[test]
    fn test_filename() {
        let f = filename();
        assert!(f.contains('.'));
    }

    #[test]
    fn test_directory() {
        assert!(!directory().is_empty());
    }
}

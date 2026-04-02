//! Programming Language generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random programming language name
pub fn name() -> String {
    fetch_locale("programming_language.names", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_NAMES).to_string())
}

/// Generate a random programming language creator
pub fn creator() -> String {
    fetch_locale("programming_language.creators", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CREATORS).to_string())
}

// Fallback data
const FALLBACK_NAMES: &[&str] = &[
    "Rust", "Python", "JavaScript", "TypeScript", "Java", "C", "C++", "C#",
    "Go", "Ruby", "PHP", "Swift", "Kotlin", "Scala", "R", "MATLAB",
    "Perl", "Lua", "Haskell", "Clojure", "Elixir", "Erlang", "F#",
    "Dart", "Julia", "Groovy", "Objective-C", "Visual Basic", "Assembly",
    "Fortran", "COBOL", "Lisp", "Scheme", "Prolog", "Smalltalk", "Ada",
];

const FALLBACK_CREATORS: &[&str] = &[
    "Graydon Hoare", "Guido van Rossum", "Brendan Eich", "Anders Hejlsberg",
    "James Gosling", "Dennis Ritchie", "Bjarne Stroustrup", "Rob Pike",
    "Yukihiro Matsumoto", "Rasmus Lerdorf", "Chris Lattner", "Martin Odersky",
    "Rich Hickey", "José Valim", "Joe Armstrong", "Lars Bak",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }

    #[test]
    fn test_creator() {
        assert!(!creator().is_empty());
    }
}

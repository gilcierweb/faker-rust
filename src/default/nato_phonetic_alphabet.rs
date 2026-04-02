//! NATO Phonetic Alphabet generator

use crate::base::sample;

/// Generate a random NATO phonetic alphabet word
pub fn code_word() -> String {
    let words = [
        "Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot",
        "Golf", "Hotel", "India", "Juliett", "Kilo", "Lima",
        "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo",
        "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "X-ray",
        "Yankee", "Zulu",
    ];
    sample(&words).to_string()
}

/// Generate a NATO phonetic spelling for a given word
pub fn spelling(word: &str) -> String {
    let nato = std::collections::HashMap::from([
        ('A', "Alpha"), ('B', "Bravo"), ('C', "Charlie"), ('D', "Delta"),
        ('E', "Echo"), ('F', "Foxtrot"), ('G', "Golf"), ('H', "Hotel"),
        ('I', "India"), ('J', "Juliett"), ('K', "Kilo"), ('L', "Lima"),
        ('M', "Mike"), ('N', "November"), ('O', "Oscar"), ('P', "Papa"),
        ('Q', "Quebec"), ('R', "Romeo"), ('S', "Sierra"), ('T', "Tango"),
        ('U', "Uniform"), ('V', "Victor"), ('W', "Whiskey"), ('X', "X-ray"),
        ('Y', "Yankee"), ('Z', "Zulu"),
    ]);
    
    word.to_uppercase()
        .chars()
        .filter_map(|c| nato.get(&c).copied())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_word() {
        assert!(!code_word().is_empty());
    }

    #[test]
    fn test_spelling() {
        let spelled = spelling("ABC");
        assert!(spelled.contains("Alpha"));
        assert!(spelled.contains("Bravo"));
        assert!(spelled.contains("Charlie"));
    }
}

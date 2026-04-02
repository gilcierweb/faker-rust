//! League of Legends generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random League of Legends champion
pub fn champion() -> String {
    fetch_locale("league_of_legends.champion", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Lee Sin".to_string())
}

/// Generate a random League of Legends location
pub fn location() -> String {
    fetch_locale("league_of_legends.location", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Summoner's Rift".to_string())
}

/// Generate a random League of Legends quote
pub fn quote() -> String {
    fetch_locale("league_of_legends.quote", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Welcome to Summoner's Rift.".to_string())
}

/// Generate a random League of Legends summoner spell
pub fn summoner_spell() -> String {
    fetch_locale("league_of_legends.summoner_spell", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Flash".to_string())
}

/// Generate a random League of Legends mastery
pub fn mastery() -> String {
    fetch_locale("league_of_legends.masteries", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Thunderlord's Decree".to_string())
}

/// Generate a random League of Legends rank
pub fn rank() -> String {
    fetch_locale("league_of_legends.rank", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| "Challenger".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_league_of_legends_generation() {
        assert!(!champion().is_empty());
        assert!(!location().is_empty());
        assert!(!quote().is_empty());
        assert!(!summoner_spell().is_empty());
        assert!(!mastery().is_empty());
        assert!(!rank().is_empty());
    }
}

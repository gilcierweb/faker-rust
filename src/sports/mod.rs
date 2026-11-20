//! Sports generators - provides data for various sports

pub mod baseball;
pub mod basketball;
pub mod football;
pub mod mountaineering;
pub mod sport;
pub mod volleyball;
pub mod world_cup;

#[cfg(test)]
mod tests {
    #[test]
    fn test_mountaineering_mountaineer_resolves_from_locale() {
        assert!(!crate::sports::mountaineering::mountaineer().is_empty());
    }
}

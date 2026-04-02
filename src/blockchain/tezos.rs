//! Tezos blockchain generator

use crate::config::FakerConfig;

/// Generate a random Tezos account address (tz1...)
pub fn account() -> String {
    encode_tz("tz1", 20)
}

/// Generate a random Tezos contract address (KT1...)
pub fn contract() -> String {
    encode_tz("KT1", 20)
}

/// Generate a random Tezos operation hash
pub fn operation() -> String {
    encode_tz("o", 32)
}

/// Generate a random Tezos block hash
pub fn block() -> String {
    encode_tz("B", 32)
}

/// Generate a random Tezos signature
pub fn signature() -> String {
    encode_tz("edsig", 64)
}

/// Generate a random Tezos public key
pub fn public_key() -> String {
    encode_tz("edpk", 32)
}

/// Generate a random Tezos secret key
pub fn secret_key() -> String {
    encode_tz("edsk", 32)
}

fn encode_tz(prefix: &str, payload_size: usize) -> String {
    const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let config = FakerConfig::current();
    
    // Generate random payload
    let mut payload = String::new();
    for _ in 0..payload_size {
        let idx = config.rand_range(0, 256) as usize % BASE58_ALPHABET.len();
        payload.push(BASE58_ALPHABET[idx] as char);
    }
    
    format!("{}{}", prefix, payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account() {
        let addr = account();
        assert!(addr.starts_with("tz1"));
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_contract() {
        let addr = contract();
        assert!(addr.starts_with("KT1"));
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_operation() {
        let op = operation();
        assert!(op.starts_with("o"));
        assert!(!op.is_empty());
    }

    #[test]
    fn test_block() {
        let blk = block();
        assert!(blk.starts_with("B"));
        assert!(!blk.is_empty());
    }

    #[test]
    fn test_signature() {
        let sig = signature();
        assert!(sig.starts_with("edsig"));
        assert!(!sig.is_empty());
    }

    #[test]
    fn test_public_key() {
        let key = public_key();
        assert!(key.starts_with("edpk"));
        assert!(!key.is_empty());
    }

    #[test]
    fn test_secret_key() {
        let key = secret_key();
        assert!(key.starts_with("edsk"));
        assert!(!key.is_empty());
    }
}

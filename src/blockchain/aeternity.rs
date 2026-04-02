//! Aeternity blockchain generator

use crate::config::FakerConfig;

/// Generate a random Aeternity wallet address
pub fn address() -> String {
    format!("ak_{}", rand_strings(50))
}

/// Generate a random Aeternity transaction hash
pub fn transaction() -> String {
    format!("th_{}", rand_strings(51))
}

/// Generate a random Aeternity contract address
pub fn contract() -> String {
    format!("ct_{}", rand_strings(50))
}

/// Generate a random Aeternity oracle address
pub fn oracle() -> String {
    format!("ok_{}", rand_strings(51))
}

fn rand_strings(length: u32) -> String {
    let config = FakerConfig::current();
    const HEX_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    let mut result = String::new();
    for _ in 0..length {
        let idx = config.rand_range(0, HEX_ALPHABET.len() as u32) as usize;
        result.push(HEX_ALPHABET[idx] as char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let addr = address();
        assert!(addr.starts_with("ak_"));
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_transaction() {
        let tx = transaction();
        assert!(tx.starts_with("th_"));
        assert!(!tx.is_empty());
    }

    #[test]
    fn test_contract() {
        let ct = contract();
        assert!(ct.starts_with("ct_"));
        assert!(!ct.is_empty());
    }

    #[test]
    fn test_oracle() {
        let oc = oracle();
        assert!(oc.starts_with("ok_"));
        assert!(!oc.is_empty());
    }
}

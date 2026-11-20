//! Ethereum blockchain generator

/// Generate a random Ethereum address
pub fn address() -> String {
    generate_ethereum_address()
}

fn generate_ethereum_address() -> String {
    // Generate a mock Ethereum address (0x + 40 hex characters)
    let mut address = String::from("0x");
    const HEX: &[u8] = b"0123456789abcdef";
    let config = crate::config::FakerConfig::current();

    for _ in 0..40 {
        let idx = config.rand_usize(HEX.len());
        address.push(HEX[idx] as char);
    }

    address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let addr = address();
        assert!(!addr.is_empty());
        // Ethereum addresses start with 0x and are 42 characters long
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
    }
}

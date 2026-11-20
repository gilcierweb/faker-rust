//! Bitcoin blockchain generator

use crate::base::sample;

/// Generate a random Bitcoin address
pub fn address() -> String {
    generate_bitcoin_address()
}

fn generate_bitcoin_address() -> String {
    // Generate a mock Bitcoin address
    let prefix = sample(&["1", "3", "bc1"]);
    let mut address = prefix.to_string();

    // Add random alphanumeric characters
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let config = crate::config::FakerConfig::current();

    let length = if address.starts_with("bc1") {
        39 // bc1 addresses are longer
    } else {
        33
    };

    for _ in 0..length {
        let idx = config.rand_usize(CHARS.len());
        address.push(CHARS[idx] as char);
    }

    address
}
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: Bitcoin testnet address (m/n/2 prefix).
pub fn testnet_address() -> String {
    let prefix = crate::base::sample(&["m", "n", "2"]);
    let mut out = prefix.to_string();
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let config = crate::config::FakerConfig::current();
    for _ in 0..33 {
        out.push(CHARS[config.rand_usize(CHARS.len())] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let addr = address();
        assert!(!addr.is_empty());
        // Bitcoin addresses start with 1, 3, or bc1
        assert!(addr.starts_with('1') || addr.starts_with('3') || addr.starts_with("bc1"));
    }
}

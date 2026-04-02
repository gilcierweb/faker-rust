//! Bitcoin blockchain generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random Bitcoin address
pub fn address() -> String {
    fetch_locale("bitcoin.addresses", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| generate_bitcoin_address())
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
        let idx = config.rand_range(0, (CHARS.len() - 1) as u32) as usize;
        address.push(CHARS[idx] as char);
    }
    
    address
}

// Fallback data (sample real Bitcoin addresses)
const FALLBACK_ADDRESSES: &[&str] = &[
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
    "3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy",
    "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
];

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

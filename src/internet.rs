//! Internet generator - generates internet-related fake data

use crate::base::sample;
use crate::base::DIGITS;
use crate::base::L_LETTERS;
use crate::base::U_LETTERS;
use crate::company;
use crate::config::FakerConfig;
use crate::name;

/// Generate a random email address
///
/// # Arguments
///
/// * `name` - Optional name to use in the email
/// * `domain` - Optional domain to use (e.g., "gmail.com")
///
/// # Example
///
/// ```rust
/// let email = faker::Internet::email();
/// let email_with_name = faker::Internet::email_with_name("john");
/// let email_with_domain = faker::Internet::email(None, Some("gmail.com"));
/// ```
pub fn email(name: Option<&str>, domain: Option<&str>) -> String {
    let local = match name {
        Some(n) => sanitize_email_local(username(Some(n))),
        None => username(None),
    };

    let domain = match domain {
        Some(d) => d.to_string(),
        None => domain_suffix(),
    };

    format!("{}@{}", local, domain)
}

/// Generate a random username
///
/// # Arguments
///
/// * `specifier` - Optional specifier (name to base username on)
pub fn username(specifier: Option<&str>) -> String {
    let config = FakerConfig::current();

    match specifier {
        Some(name) => {
            let name = sanitize_username(name);
            let separator = sample(&['.', '_']);
            name.to_lowercase()
                .split_whitespace()
                .take(2)
                .collect::<Vec<_>>()
                .join(&separator.to_string())
        }
        None => {
            let first = name::first_name().to_lowercase();
            let last = name::last_name().to_lowercase();
            let separator = sample(&['.', '_']);
            format!("{}{}{}", first, separator, last)
        }
    }
}

/// Generate a random password
///
/// # Arguments
///
/// * `min_length` - Minimum length of password (default: 8)
/// * `max_length` - Maximum length of password (default: 16)
/// * `mix_case` - Include uppercase letters (default: true)
/// * `special` - Include special characters (default: false)
pub fn password(min_length: usize, max_length: usize, mix_case: bool, special: bool) -> String {
    let config = FakerConfig::current();

    let mut required_min = 2; // lowercase + digit
    if mix_case {
        required_min += 1;
    }
    if special {
        required_min += 1;
    }

    if min_length < required_min {
        panic!(
            "min_length must be at least {} for the given options",
            required_min
        );
    }

    let length = config.rand_range(min_length as u32, max_length as u32) as usize;
    let mut password = Vec::with_capacity(length);

    // Ensure at least one lowercase letter
    password.push(config.rand_char(&L_LETTERS));

    // Ensure at least one digit
    password.push(config.rand_char(&DIGITS));

    // Add uppercase if mix_case
    if mix_case {
        password.push(config.rand_char(&U_LETTERS));
    }

    // Add special character if special
    if special {
        password.push(sample(&['!', '@', '#', '$', '%', '^', '&', '*']));
    }

    // Fill remaining with random characters
    let mut charset: Vec<char> = L_LETTERS.to_vec();
    charset.extend_from_slice(&DIGITS);
    if mix_case {
        charset.extend_from_slice(&U_LETTERS);
    }
    if special {
        charset.extend_from_slice(&['!', '@', '#', '$', '%', '^', '&', '*']);
    }

    while password.len() < length {
        password.push(config.sample(&charset));
    }

    // Shuffle
    config.shuffle(&mut password);

    password.into_iter().collect()
}

/// Generate a random domain name
///
/// # Arguments
///
/// * `subdomain` - Include a subdomain (default: false)
/// * `domain` - Specific domain to use
pub fn domain_name(subdomain: bool, domain: Option<&str>) -> String {
    if let Some(d) = domain {
        let parts: Vec<&str> = d.split('.').collect();
        if parts.len() < 2 {
            return format!("{}.{}", domain_word(), domain_suffix());
        }
        return d.to_string();
    }

    let base = domain_word();

    if subdomain {
        format!("{}.{}.{}", domain_word(), base, domain_suffix())
    } else {
        format!("{}.{}", base, domain_suffix())
    }
}

/// Generate a random domain suffix (com, org, net, etc.)
pub fn domain_suffix() -> String {
    sample(&DOMAIN_SUFFIXES).to_string()
}

/// Generate a random domain word (from company name)
fn domain_word() -> String {
    let company_name = company::name();
    company_name
        .split_whitespace()
        .next()
        .unwrap_or("example")
        .to_lowercase()
}

/// Generate a random IPv4 address
pub fn ip_v4() -> String {
    let config = FakerConfig::current();
    format!(
        "{}.{}.{}.{}",
        config.rand_range(1, 255),
        config.rand_range(0, 255),
        config.rand_range(0, 255),
        config.rand_range(1, 255)
    )
}

/// Generate a random IPv6 address
pub fn ip_v6() -> String {
    let config = FakerConfig::current();
    (0..8)
        .map(|_| format!("{:x}", config.rand_range(0, 65535)))
        .collect::<Vec<_>>()
        .join(":")
}

/// Generate a random MAC address
///
/// # Arguments
///
/// * `prefix` - Optional prefix for the MAC address
pub fn mac_address(prefix: Option<&str>) -> String {
    let config = FakerConfig::current();

    let (prefix_bytes, count) = if let Some(p) = prefix {
        let parts: Vec<&str> = p.split(':').collect();
        let bytes: Vec<u8> = parts
            .iter()
            .filter_map(|s| u8::from_str_radix(s, 16).ok())
            .collect();
        let len = bytes.len();
        (bytes, 6 - len)
    } else {
        (vec![], 6)
    };

    let mut result = prefix_bytes;
    for _ in 0..count {
        result.push(config.rand_range(0, 256) as u8);
    }

    result
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(":")
}

/// Generate a random URL
///
/// # Arguments
///
/// * `host` - Host domain
/// * `path` - URL path
/// * `scheme` - URL scheme (http, https)
pub fn url(host: Option<&str>, path: Option<&str>, scheme: Option<&str>) -> String {
    let host = host.unwrap_or_else(|| &domain_name(false, None));
    let generated_path = format!("/{}", username(None));
    let path = path.unwrap_or(&generated_path);
    let scheme = scheme.unwrap_or("http");
    format!("{}://{}{}", scheme, host, path)
}

/// Generate a random UUID
pub fn uuid() -> String {
    let config = FakerConfig::current();
    let bytes: [u8; 16] = config.sample(&[
        // Generate random bytes - simplified version
        [0; 16], // placeholder - will be replaced
    ]);

    // Use rand to generate proper UUID
    use rand::Rng;
    let mut rng = config.rng.borrow_mut();
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    // Set version and variant
    bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 10

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

/// Generate a random user agent string
pub fn user_agent() -> String {
    let config = FakerConfig::current();
    let vendor = config.rand_range(0, 4);

    match vendor {
        0 => format!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.{}.{} Safari/537.36",
            config.rand_range(80, 120),
            config.rand_range(1000, 9999),
            config.rand_range(0, 100)
        ),
        1 => format!(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{}.{} Safari/605.1.15",
            config.rand_range(14, 17),
            config.rand_range(0, 10)
        ),
        2 => format!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:{}) Firefox/{}.0",
            config.rand_range(90, 120),
            config.rand_range(0, 100)
        ),
        _ => format!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.129 Safari/537.36 Edg/120.0.2150.72"
        ),
    }
}

// Helper functions

fn sanitize_email_local(local: String) -> String {
    local
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-' || *c == '+')
        .collect()
}

fn sanitize_username(name: &str) -> String {
    name.replace("'", "").replace('.', " ")
}

// Data

const DOMAIN_SUFFIXES: &[&str] = &[
    "com", "net", "org", "io", "co", "biz", "info", "me", "us", "uk", "ca", "de", "fr", "jp", "ru",
    "cn", "au", "in", "br", "es",
];

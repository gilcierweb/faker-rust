//! Internet generator - generates internet-related fake data

use crate::base::sample;
use crate::base::ALPHANUMERIC;
use crate::base::DIGITS;
use crate::base::L_LETTERS;
use crate::base::U_LETTERS;
use crate::company;
use crate::config::FakerConfig;
use crate::locale::fetch_locale;
use crate::name;

/// Generate a random email address
pub fn email(name: Option<&str>, domain: Option<&str>, separators: Option<Vec<&str>>) -> String {
    let sep = separators.unwrap_or_else(|| vec!["."]);
    let local = match name {
        Some(n) => sanitize_email_local(username_with_sep(Some(n), &sep)),
        None => username_with_sep(None, &sep),
    };

    let domain = match domain {
        Some(d) => d.to_string(),
        None => domain_suffix(false),
    };

    format!("{}@{}", local, domain)
}

/// Generate a random username with custom separators
fn username_with_sep(specifier: Option<&str>, separators: &[&str]) -> String {
    match specifier {
        Some(name) => {
            let name = sanitize_username(name);
            let sep = sample(separators);
            name.to_lowercase()
                .split_whitespace()
                .take(2)
                .collect::<Vec<_>>()
                .join(sep)
        }
        None => {
            let first = name::first_name().to_lowercase();
            let last = name::last_name().to_lowercase();
            let sep = sample(separators);
            format!("{}{}{}", first, sep, last)
        }
    }
}

/// Generate a random username
pub fn username(specifier: Option<&str>) -> String {
    username_with_sep(specifier, &["."])
}

/// Generate a random password
pub fn password(min_length: usize, max_length: usize, mix_case: bool, special: bool) -> String {
    let config = FakerConfig::current();

    let mut required_min = 2;
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

    password.push(config.rand_char(&L_LETTERS));
    password.push(config.rand_char(&DIGITS));

    if mix_case {
        password.push(config.rand_char(&U_LETTERS));
    }

    if special {
        password.push(sample(&['!', '@', '#', '$', '%', '^', '&', '*']));
    }

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

    config.shuffle(&mut password);

    password.into_iter().collect()
}

/// Generate a random domain name
pub fn domain_name(subdomain: bool, domain: Option<&str>) -> String {
    if let Some(d) = domain {
        let parts: Vec<&str> = d.split('.').collect();
        if parts.len() < 2 {
            return format!("{}.{}", domain_word(), domain_suffix(false));
        }
        return d.to_string();
    }

    let base = domain_word();

    if subdomain {
        format!("{}.{}.{}", domain_word(), base, domain_suffix(false))
    } else {
        format!("{}.{}", base, domain_suffix(false))
    }
}

/// Generate a random domain suffix
pub fn domain_suffix(safe: bool) -> String {
    if safe {
        fetch_locale("internet.safe_domain_suffix", "en")
            .map(|v| sample(&v))
            .unwrap_or_else(|| sample(&FALLBACK_SAFE_DOMAIN_SUFFIXES).to_string())
    } else {
        fetch_locale("internet.domain_suffix", "en")
            .map(|v| sample(&v))
            .unwrap_or_else(|| sample(&FALLBACK_DOMAIN_SUFFIXES).to_string())
    }
}

/// Fixes ä, ö, ü, ß characters in string
pub fn fix_umlauts(string: &str) -> string::String {
    string
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
}

/// Generate a random domain word
fn domain_word() -> string::String {
    let company_name = company::name();
    company_name
        .split_whitespace()
        .next()
        .unwrap_or("example")
        .to_lowercase()
}

/// Generate a random IPv4 address
pub fn ip_v4() -> string::String {
    let config = FakerConfig::current();
    format!(
        "{}.{}.{}.{}",
        config.rand_range(1, 255),
        config.rand_range(0, 255),
        config.rand_range(0, 255),
        config.rand_range(1, 255)
    )
}

/// Generate a random private IPv4 address
pub fn private_ip_v4() -> string::String {
    let config = FakerConfig::current();
    let choice = config.rand_range(0, 5);
    match choice {
        0 => format!(
            "10.{}.{}.{}",
            config.rand_range(0, 255),
            config.rand_range(0, 255),
            config.rand_range(1, 255)
        ),
        1 => format!(
            "172.{}.{}.{}",
            config.rand_range(16, 31),
            config.rand_range(0, 255),
            config.rand_range(1, 255)
        ),
        _ => format!(
            "192.168.{}.{}",
            config.rand_range(0, 255),
            config.rand_range(1, 255)
        ),
    }
}

/// Generate a random IPv6 address
pub fn ip_v6() -> string::String {
    let config = FakerConfig::current();
    (0..8)
        .map(|_| format!("{:x}", config.rand_range(0, 65535)))
        .collect::<Vec<_>>()
        .join(":")
}

/// Generate IPv4 with CIDR notation
pub fn ip_v4_cidr() -> string::String {
    let config = FakerConfig::current();
    format!("{}/{}", ip_v4(), config.rand_range(1, 31))
}

/// Generate IPv6 with CIDR notation
pub fn ip_v6_cidr() -> string::String {
    let config = FakerConfig::current();
    format!("{}/{}", ip_v6(), config.rand_range(1, 127))
}

/// Generate a random MAC address
pub fn mac_address(prefix: Option<&str>) -> string::String {
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
pub fn url(host: Option<&str>, path: Option<&str>, scheme: Option<&str>) -> string::String {
    let generated_host = domain_name(false, None);
    let host = host.unwrap_or(&generated_host);
    let generated_path = format!("/{}", username(None));
    let path = path.unwrap_or(&generated_path);
    let scheme = scheme.unwrap_or("http");
    format!("{}://{}{}", scheme, host, path)
}

/// Generate a random slug (URL-friendly string)
pub fn slug(words: Option<&str>, glue: Option<&str>) -> string::String {
    let config = FakerConfig::current();

    if let Some(w) = words {
        let g = glue.unwrap_or("-");
        w.replace(',', "")
            .replace('.', "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(g)
            .to_lowercase()
    } else {
        let g = glue.unwrap_or_else(|| if config.rand_bool() { "-" } else { "_" });
        let word1 = name::first_name().to_lowercase();
        let word2 = name::last_name().to_lowercase();
        format!("{}{}{}", word1, g, word2)
    }
}

/// Generate a random device token
pub fn device_token() -> string::String {
    let config = FakerConfig::current();
    let mut chars: Vec<char> = (0..64)
        .map(|_| {
            let idx = config.rand_range(0, 16) as u8;
            format!("{:x}", idx).chars().next().unwrap()
        })
        .collect();
    config.shuffle(&mut chars);
    chars.into_iter().collect()
}

/// Generate a random UUID
pub fn uuid() -> string::String {
    use rand::Rng;
    let config = FakerConfig::current();
    let mut rng = config.rng.borrow_mut();
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

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
pub fn user_agent(vendor: Option<&str>) -> string::String {
    let config = FakerConfig::current();

    // List of supported vendors (same as Ruby Faker)
    let vendors = [
        "chrome",
        "safari",
        "firefox",
        "internet_explorer",
        "opera",
        "netscape",
        "aol",
    ];

    let selected_vendor = match vendor {
        Some(v) => {
            if vendors.contains(&v) {
                v
            } else {
                return generate_fallback_user_agent(&config);
            }
        }
        None => {
            let idx = config.rand_range(0, vendors.len() as u32) as usize;
            vendors[idx]
        }
    };

    // Try to get agents from locale - handle both key formats
    let path = format!("user_agent.{}", selected_vendor);

    let result =
        fetch_locale(&format!("internet.{}", path), "en").or_else(|| fetch_locale(&path, "en"));

    result
        .map(|agents| sample(&agents))
        .unwrap_or_else(|| generate_fallback_user_agent(&config))
}

fn generate_fallback_user_agent(config: &FakerConfig) -> string::String {
    let version = config.rand_range(80, 120);
    format!(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
        version
    )
}

/// Generate a bot user agent
pub fn bot_user_agent(vendor: Option<&str>) -> string::String {
    let config = FakerConfig::current();

    // List of supported vendors (same as Ruby Faker)
    let vendors = [
        "googlebot",
        "bingbot",
        "duckduckbot",
        "baiduspider",
        "yandexbot",
    ];

    let selected_vendor = match vendor {
        Some(v) => {
            // Validate vendor is supported
            if vendors.contains(&v) {
                v
            } else {
                return sample(&FALLBACK_BOT_AGENTS).to_string();
            }
        }
        None => {
            let idx = config.rand_range(0, vendors.len() as u32) as usize;
            vendors[idx]
        }
    };

    // Try to get agents from locale - use a simpler key format
    let path = format!("bot_user_agent.{}", selected_vendor);

    // Try with 'internet.' prefix first
    let result =
        fetch_locale(&format!("internet.{}", path), "en").or_else(|| fetch_locale(&path, "en"));

    result
        .map(|agents| sample(&agents))
        .unwrap_or_else(|| sample(&FALLBACK_BOT_AGENTS).to_string())
}

/// Generate a random base64 string
pub fn base64(length: usize, padding: bool, urlsafe: bool) -> string::String {
    let config = FakerConfig::current();
    let charset: Vec<char> = if urlsafe {
        let mut c: Vec<char> = ('0'..='9').collect();
        c.extend('A'..='Z');
        c.extend('a'..='z');
        c.push('-');
        c.push('_');
        c
    } else {
        let mut c: Vec<char> = ('0'..='9').collect();
        c.extend('A'..='Z');
        c.extend('a'..='z');
        c.push('+');
        c.push('/');
        c
    };

    let mut result: string::String = (0..length).map(|_| config.sample(&charset)).collect();

    if padding {
        while result.len() % 4 != 0 {
            result.push('=');
        }
    }

    result
}

/// Generate a random internet user hash
pub fn user(fields: Vec<&str>) -> std::collections::HashMap<string::String, string::String> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    if fields.is_empty() {
        map.insert("username".to_string(), username(None));
        map.insert("email".to_string(), email(None, None, None));
    } else {
        for field in fields {
            match field {
                "username" => {
                    map.insert("username".to_string(), username(None));
                }
                "email" => {
                    map.insert("email".to_string(), email(None, None, None));
                }
                "password" => {
                    map.insert("password".to_string(), password(12, 20, true, true));
                }
                _ => {}
            }
        }
    }
    map
}

fn sanitize_email_local(local: string::String) -> string::String {
    local
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-' || *c == '+')
        .collect()
}

fn sanitize_username(name: &str) -> string::String {
    name.replace("'", "").replace('.', " ")
}

mod string {
    pub type String = std::string::String;
}

const DOMAIN_SUFFIXES: &[&str] = &[
    "com", "net", "org", "io", "co", "biz", "info", "me", "us", "uk", "ca", "de", "fr", "jp",
];

const SAFE_DOMAIN_SUFFIXES: &[&str] = &["test", "example", "localhost"];

const FALLBACK_DOMAIN_SUFFIXES: &[&str] = &[
    "com", "net", "org", "io", "co", "biz", "info", "me", "us", "uk", "ca", "de", "fr", "jp",
];

const FALLBACK_SAFE_DOMAIN_SUFFIXES: &[&str] = &["test", "example", "localhost"];

const FALLBACK_BOT_AGENTS: &[&str] = &[
    "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Chrome/99.0.4844.84 Safari/537.36",
    "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm) Chrome/86.0.4240.68 Safari/537.36 Edg/86.0.622.31",
    "DuckDuckBot/1.0; (+http://duckduckgo.com/duckduckbot.html)",
    "Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)",
    "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)",
];

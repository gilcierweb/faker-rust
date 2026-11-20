//! OmniAuth generator

use crate::base::sample;
use crate::config::FakerConfig;
use crate::locale::fetch_locale;

/// Generate a random OmniAuth provider
pub fn provider() -> String {
    fetch_locale("omniauth.providers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_PROVIDERS).to_string())
}

/// Generate a random OmniAuth UID
pub fn uid() -> String {
    let config = FakerConfig::current();
    let numbers: String = (0..17)
        .map(|_| config.rand_char(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
        .collect();
    numbers
}

/// Generate a random OmniAuth name
pub fn name() -> String {
    let names = [
        "google_oauth2",
        "facebook",
        "twitter",
        "github",
        "linkedin",
        "developer",
        "saml",
        "openid_connect",
        "auth0",
        "okta",
    ];
    sample(&names).to_string()
}

/// Generate a random directory name (shared with [`crate::source::directory`]).
pub use crate::source::directory;

// Fallback data
const FALLBACK_PROVIDERS: &[&str] = &[
    "google_oauth2",
    "facebook",
    "twitter",
    "github",
    "linkedin",
    "developer",
    "saml",
    "openid_connect",
    "auth0",
    "okta",
    "microsoft",
    "slack",
    "discord",
];

// --- gem parity generators (auth hashes) ---

fn config() -> FakerConfig {
    FakerConfig::current()
}

fn auth_full_name(name: Option<&str>) -> String {
    name.map(str::to_string).unwrap_or_else(crate::name::name)
}

fn auth_first_name(name: Option<&str>) -> String {
    match name {
        Some(value) => value.split_whitespace().next().unwrap_or(value).to_string(),
        None => crate::name::first_name(),
    }
}

fn auth_last_name(name: Option<&str>) -> String {
    match name {
        Some(value) => value.split_whitespace().last().unwrap_or(value).to_string(),
        None => crate::name::last_name(),
    }
}

fn auth_email(email: Option<&str>, name: &str) -> String {
    email
        .map(str::to_string)
        .unwrap_or_else(|| crate::internet::email(Some(name), None, None))
}

fn auth_uid(explicit: Option<&str>, digits: usize) -> String {
    match explicit {
        Some(value) => value.to_string(),
        None => (0..digits)
            .map(|_| char::from(b'0' + config().rand_u32(10) as u8))
            .collect(),
    }
}

fn auth_image() -> String {
    crate::placeholdit::image()
}

fn auth_token() -> String {
    crate::crypto::md5()
}

fn auth_city_state() -> String {
    format!(
        "{}, {}",
        crate::address::city(),
        crate::address::state_abbr()
    )
}

fn auth_description() -> String {
    crate::lorem::sentence(Some(1))
}

fn auth_boolean() -> bool {
    config().rand_bool()
}

fn auth_range(from: u32, to: u32) -> u32 {
    config().rand_range(from, to)
}

fn auth_future_timestamp() -> i64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default();
    now + auth_range(1, 86_400) as i64
}

fn auth_hex(digits: usize) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    (0..digits)
        .map(|_| char::from(crate::base::sample(HEX)))
        .collect()
}

fn auth_root(provider: &str, uid: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert(
        "provider".into(),
        serde_json::Value::String(provider.into()),
    );
    map.insert("uid".into(), serde_json::Value::String(uid.into()));
    map
}

fn auth_credentials(extra: serde_json::Map<String, serde_json::Value>) -> serde_json::Value {
    let mut map = extra;
    map.insert("token".into(), serde_json::Value::String(auth_token()));
    map.insert(
        "expires_at".into(),
        serde_json::Value::Number(serde_json::Number::from(auth_future_timestamp())),
    );
    map.insert("expires".into(), serde_json::Value::Bool(true));
    serde_json::Value::Object(map)
}

/// Gem-compatible generator: Google OmniAuth auth hash.
pub fn google(name: Option<&str>, email: Option<&str>, uid: Option<&str>) -> serde_json::Value {
    let uid = auth_uid(uid, 9);
    let full = auth_full_name(name);
    let first = auth_first_name(name);
    let last = auth_last_name(name);
    let mail = auth_email(email, &full);
    let picture = auth_image();
    let mut root = auth_root("google_oauth2", &uid);

    let mut info = serde_json::Map::new();
    info.insert("name".into(), serde_json::Value::String(full.clone()));
    info.insert(
        "first_name".into(),
        serde_json::Value::String(first.clone()),
    );
    info.insert("last_name".into(), serde_json::Value::String(last.clone()));
    info.insert("email".into(), serde_json::Value::String(mail.clone()));
    info.insert("image".into(), serde_json::Value::String(picture.clone()));
    root.insert("info".into(), serde_json::Value::Object(info));
    root.insert("credentials".into(), {
        let mut credentials = serde_json::Map::new();
        credentials.insert(
            "refresh_token".into(),
            serde_json::Value::String(auth_token()),
        );
        auth_credentials(credentials)
    });

    let mut raw_info = serde_json::Map::new();
    raw_info.insert("sub".into(), serde_json::Value::String(uid.clone()));
    raw_info.insert("email".into(), serde_json::Value::String(mail.clone()));
    raw_info.insert(
        "email_verified".into(),
        serde_json::Value::String(auth_boolean().to_string()),
    );
    raw_info.insert("name".into(), serde_json::Value::String(full.clone()));
    raw_info.insert(
        "given_name".into(),
        serde_json::Value::String(first.clone()),
    );
    raw_info.insert(
        "family_name".into(),
        serde_json::Value::String(last.clone()),
    );
    raw_info.insert(
        "profile".into(),
        serde_json::Value::String(format!("https://plus.google.com/{uid}")),
    );
    raw_info.insert("picture".into(), serde_json::Value::String(picture));
    raw_info.insert(
        "gender".into(),
        serde_json::Value::String(crate::base::sample(&["male", "female"]).to_string()),
    );
    raw_info.insert("locale".into(), serde_json::Value::String("en".to_string()));
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

/// Gem-compatible generator: Facebook OmniAuth auth hash.
pub fn facebook(
    name: Option<&str>,
    email: Option<&str>,
    username: Option<&str>,
    uid: Option<&str>,
) -> serde_json::Value {
    let uid = auth_uid(uid, 7);
    let full = auth_full_name(name);
    let first = auth_first_name(name);
    let last = auth_last_name(name);
    let handle = username
        .map(str::to_string)
        .unwrap_or_else(|| format!("{}{}", first.to_lowercase(), last.to_lowercase()));
    let mail = auth_email(email, &full);
    let mut root = auth_root("facebook", &uid);

    let mut info = serde_json::Map::new();
    info.insert("email".into(), serde_json::Value::String(mail.clone()));
    info.insert("name".into(), serde_json::Value::String(full.clone()));
    info.insert(
        "first_name".into(),
        serde_json::Value::String(first.clone()),
    );
    info.insert("last_name".into(), serde_json::Value::String(last.clone()));
    info.insert("image".into(), serde_json::Value::String(auth_image()));
    info.insert("verified".into(), serde_json::Value::Bool(auth_boolean()));
    root.insert("info".into(), serde_json::Value::Object(info));
    root.insert(
        "credentials".into(),
        auth_credentials(serde_json::Map::new()),
    );

    let mut raw_info = serde_json::Map::new();
    raw_info.insert("id".into(), serde_json::Value::String(uid.clone()));
    raw_info.insert("name".into(), serde_json::Value::String(full.clone()));
    raw_info.insert(
        "link".into(),
        serde_json::Value::String(format!("http://www.facebook.com/{handle}")),
    );
    raw_info.insert("username".into(), serde_json::Value::String(handle));
    raw_info.insert(
        "location".into(),
        serde_json::Value::String(auth_city_state()),
    );
    raw_info.insert(
        "gender".into(),
        serde_json::Value::String(crate::base::sample(&["male", "female"]).to_string()),
    );
    raw_info.insert("email".into(), serde_json::Value::String(mail));
    raw_info.insert(
        "locale".into(),
        serde_json::Value::String("en_US".to_string()),
    );
    raw_info.insert("verified".into(), serde_json::Value::Bool(auth_boolean()));
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

/// Gem-compatible generator: Twitter OmniAuth auth hash.
pub fn twitter(name: Option<&str>, nickname: Option<&str>, uid: Option<&str>) -> serde_json::Value {
    let uid = auth_uid(uid, 6);
    let full = auth_full_name(name);
    let handle = nickname
        .map(str::to_string)
        .unwrap_or_else(|| full.to_lowercase().replace(' ', ""));
    let location = auth_city_state();
    let description = auth_description();
    let mut root = auth_root("twitter", &uid);

    let mut info = serde_json::Map::new();
    info.insert("nickname".into(), serde_json::Value::String(handle.clone()));
    info.insert("name".into(), serde_json::Value::String(full.clone()));
    info.insert(
        "location".into(),
        serde_json::Value::String(location.clone()),
    );
    info.insert("image".into(), serde_json::Value::String(auth_image()));
    info.insert(
        "description".into(),
        serde_json::Value::String(description.clone()),
    );
    let mut urls = serde_json::Map::new();
    urls.insert("Website".into(), serde_json::Value::Null);
    urls.insert(
        "Twitter".into(),
        serde_json::Value::String(format!("https://twitter.com/{handle}")),
    );
    info.insert("urls".into(), serde_json::Value::Object(urls));
    root.insert("info".into(), serde_json::Value::Object(info));

    let mut credentials = serde_json::Map::new();
    credentials.insert("secret".into(), serde_json::Value::String(auth_token()));
    root.insert("credentials".into(), auth_credentials(credentials));

    let mut raw_info = serde_json::Map::new();
    raw_info.insert("name".into(), serde_json::Value::String(full));
    raw_info.insert("location".into(), serde_json::Value::String(location));
    raw_info.insert("description".into(), serde_json::Value::String(description));
    raw_info.insert("id".into(), serde_json::Value::String(uid.clone()));
    raw_info.insert("id_str".into(), serde_json::Value::String(uid));
    raw_info.insert("screen_name".into(), serde_json::Value::String(handle));
    raw_info.insert("lang".into(), serde_json::Value::String("en".to_string()));
    raw_info.insert("verified".into(), serde_json::Value::Bool(auth_boolean()));
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

/// Gem-compatible generator: LinkedIn OmniAuth auth hash.
pub fn linkedin(name: Option<&str>, email: Option<&str>, uid: Option<&str>) -> serde_json::Value {
    let uid = auth_uid(uid, 6);
    let full = auth_full_name(name);
    let first = auth_first_name(name);
    let last = auth_last_name(name);
    let mail = auth_email(email, &full);
    let description = auth_description();
    let industry = parity_department();
    let url = format!(
        "http://www.linkedin.com/in/{}{}",
        first.to_lowercase(),
        last.to_lowercase()
    );
    let mut root = auth_root("linkedin", &uid);

    let mut info = serde_json::Map::new();
    info.insert("name".into(), serde_json::Value::String(full.clone()));
    info.insert("email".into(), serde_json::Value::String(mail.clone()));
    info.insert("nickname".into(), serde_json::Value::String(full.clone()));
    info.insert(
        "first_name".into(),
        serde_json::Value::String(first.clone()),
    );
    info.insert("last_name".into(), serde_json::Value::String(last.clone()));
    info.insert(
        "location".into(),
        serde_json::Value::String(auth_city_state()),
    );
    info.insert(
        "description".into(),
        serde_json::Value::String(description.clone()),
    );
    info.insert("image".into(), serde_json::Value::String(auth_image()));
    info.insert(
        "headline".into(),
        serde_json::Value::String(description.clone()),
    );
    info.insert(
        "industry".into(),
        serde_json::Value::String(industry.clone()),
    );
    let mut urls = serde_json::Map::new();
    urls.insert(
        "public_profile".into(),
        serde_json::Value::String(url.clone()),
    );
    info.insert("urls".into(), serde_json::Value::Object(urls));
    root.insert("info".into(), serde_json::Value::Object(info));
    root.insert("credentials".into(), {
        let mut credentials = serde_json::Map::new();
        credentials.insert("secret".into(), serde_json::Value::String(auth_token()));
        auth_credentials(credentials)
    });

    let mut raw_info = serde_json::Map::new();
    raw_info.insert("firstName".into(), serde_json::Value::String(first));
    raw_info.insert("headline".into(), serde_json::Value::String(description));
    raw_info.insert("id".into(), serde_json::Value::String(uid));
    raw_info.insert("industry".into(), serde_json::Value::String(industry));
    raw_info.insert("lastName".into(), serde_json::Value::String(last));
    raw_info.insert("publicProfileUrl".into(), serde_json::Value::String(url));
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

fn parity_department() -> String {
    crate::locale::fetch_locale("company.department", "en")
        .map(|values| crate::base::sample(&values))
        .unwrap_or_else(|| "Engineering".to_string())
}

/// Gem-compatible generator: Github OmniAuth auth hash.
pub fn github(name: Option<&str>, email: Option<&str>, uid: Option<&str>) -> serde_json::Value {
    let uid = auth_uid(uid, 8);
    let full = auth_full_name(name);
    let login = full.to_lowercase().replace(' ', "-");
    let html_url = format!("https://github.com/{login}");
    let api_url = format!("https://api.github.com/users/{login}");
    let mail = auth_email(email, &full);
    let mut root = auth_root("github", &uid);

    let mut info = serde_json::Map::new();
    info.insert("nickname".into(), serde_json::Value::String(login.clone()));
    info.insert("email".into(), serde_json::Value::String(mail.clone()));
    info.insert("name".into(), serde_json::Value::String(full.clone()));
    info.insert("image".into(), serde_json::Value::String(auth_image()));
    let mut urls = serde_json::Map::new();
    urls.insert("GitHub".into(), serde_json::Value::String(html_url.clone()));
    info.insert("urls".into(), serde_json::Value::Object(urls));
    root.insert("info".into(), serde_json::Value::Object(info));

    let mut credentials = serde_json::Map::new();
    credentials.insert("expires".into(), serde_json::Value::Bool(false));
    root.insert("credentials".into(), {
        credentials.insert("token".into(), serde_json::Value::String(auth_token()));
        serde_json::Value::Object(credentials)
    });

    let mut raw_info = serde_json::Map::new();
    raw_info.insert("login".into(), serde_json::Value::String(login));
    raw_info.insert("id".into(), serde_json::Value::String(uid));
    raw_info.insert("url".into(), serde_json::Value::String(api_url.clone()));
    raw_info.insert("html_url".into(), serde_json::Value::String(html_url));
    raw_info.insert(
        "followers_url".into(),
        serde_json::Value::String(format!("{api_url}/followers")),
    );
    raw_info.insert("type".into(), serde_json::Value::String("User".to_string()));
    raw_info.insert("name".into(), serde_json::Value::String(full));
    raw_info.insert("email".into(), serde_json::Value::String(mail));
    raw_info.insert(
        "public_repos".into(),
        serde_json::Value::Number(serde_json::Number::from(auth_range(1, 1000))),
    );
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

/// Gem-compatible generator: Apple OmniAuth auth hash.
pub fn apple(name: Option<&str>, email: Option<&str>, uid: Option<&str>) -> serde_json::Value {
    let uid = match uid {
        Some(value) => value.to_string(),
        None => format!(
            "{}.{}.{}",
            auth_uid(None, 6),
            auth_hex(32),
            auth_uid(None, 4)
        ),
    };
    let full = auth_full_name(name);
    let first = auth_first_name(name);
    let last = auth_last_name(name);
    let mail = auth_email(email, &full);
    let mut root = auth_root("apple", &uid);

    let mut info = serde_json::Map::new();
    info.insert("sub".into(), serde_json::Value::String(uid.clone()));
    info.insert("email".into(), serde_json::Value::String(mail.clone()));
    info.insert("first_name".into(), serde_json::Value::String(first));
    info.insert("last_name".into(), serde_json::Value::String(last));
    root.insert("info".into(), serde_json::Value::Object(info));
    root.insert("credentials".into(), {
        let mut credentials = serde_json::Map::new();
        credentials.insert(
            "refresh_token".into(),
            serde_json::Value::String(auth_token()),
        );
        auth_credentials(credentials)
    });

    let mut raw_info = serde_json::Map::new();
    raw_info.insert(
        "iss".into(),
        serde_json::Value::String("https://appleid.apple.com".to_string()),
    );
    raw_info.insert(
        "aud".into(),
        serde_json::Value::String("CLIENT_ID".to_string()),
    );
    raw_info.insert(
        "exp".into(),
        serde_json::Value::Number(serde_json::Number::from(auth_future_timestamp())),
    );
    raw_info.insert(
        "iat".into(),
        serde_json::Value::Number(serde_json::Number::from(auth_future_timestamp())),
    );
    raw_info.insert("sub".into(), serde_json::Value::String(uid));
    raw_info.insert("email".into(), serde_json::Value::String(mail));
    raw_info.insert("email_verified".into(), serde_json::Value::Bool(true));
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

/// Gem-compatible generator: Auth0 OmniAuth auth hash.
pub fn auth0(name: Option<&str>, email: Option<&str>, uid: Option<&str>) -> serde_json::Value {
    let uid = match uid {
        Some(value) => value.to_string(),
        None => format!("auth0|{}", auth_hex(24)),
    };
    let full = auth_full_name(name);
    let mail = auth_email(email, &full);
    let mut root = auth_root("auth0", &uid);

    let mut info = serde_json::Map::new();
    info.insert("name".into(), serde_json::Value::String(uid.clone()));
    info.insert("nickname".into(), serde_json::Value::String(full));
    info.insert("email".into(), serde_json::Value::String(mail.clone()));
    info.insert("image".into(), serde_json::Value::String(auth_image()));
    root.insert("info".into(), serde_json::Value::Object(info));

    let mut credentials = serde_json::Map::new();
    credentials.insert(
        "token_type".into(),
        serde_json::Value::String("Bearer".to_string()),
    );
    credentials.insert(
        "id_token".into(),
        serde_json::Value::String(crate::crypto::sha256()),
    );
    credentials.insert(
        "refresh_token".into(),
        serde_json::Value::String(auth_token()),
    );
    root.insert("credentials".into(), auth_credentials(credentials));

    let mut raw_info = serde_json::Map::new();
    raw_info.insert("email".into(), serde_json::Value::String(mail));
    raw_info.insert("email_verified".into(), serde_json::Value::Bool(true));
    raw_info.insert(
        "iss".into(),
        serde_json::Value::String("https://auth0.com/".to_string()),
    );
    raw_info.insert("sub".into(), serde_json::Value::String(uid));
    raw_info.insert(
        "aud".into(),
        serde_json::Value::String("Auth012345".to_string()),
    );
    raw_info.insert(
        "iat".into(),
        serde_json::Value::Number(serde_json::Number::from(auth_future_timestamp())),
    );
    raw_info.insert(
        "exp".into(),
        serde_json::Value::Number(serde_json::Number::from(auth_future_timestamp())),
    );
    let mut extra = serde_json::Map::new();
    extra.insert("raw_info".into(), serde_json::Value::Object(raw_info));
    root.insert("extra".into(), serde_json::Value::Object(extra));
    serde_json::Value::Object(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider() {
        assert!(!provider().is_empty());
    }

    #[test]
    fn test_uid() {
        let id = uid();
        assert_eq!(id.len(), 17);
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_name() {
        assert!(!name().is_empty());
    }
}

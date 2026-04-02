//! Internet HTTP generator

use crate::base::sample;
use crate::locale::fetch_locale;

/// Generate a random HTTP method
pub fn method() -> String {
    fetch_locale("internet_http.methods", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_METHODS).to_string())
}

/// Generate a random HTTP status code
pub fn status_code() -> String {
    fetch_locale("internet_http.status_codes", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_STATUS_CODES).to_string())
}

/// Generate a random HTTP header
pub fn header() -> String {
    fetch_locale("internet_http.headers", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_HEADERS).to_string())
}

/// Generate a random HTTP content type
pub fn content_type() -> String {
    fetch_locale("internet_http.content_types", "en")
        .map(|v| sample(&v))
        .unwrap_or_else(|| sample(FALLBACK_CONTENT_TYPES).to_string())
}

// Fallback data
const FALLBACK_METHODS: &[&str] = &[
    "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS", "CONNECT", "TRACE",
];

const FALLBACK_STATUS_CODES: &[&str] = &[
    "200", "201", "204", "301", "302", "304", "400", "401", "403", "404",
    "405", "500", "502", "503", "504",
];

const FALLBACK_HEADERS: &[&str] = &[
    "Accept", "Accept-Charset", "Accept-Encoding", "Accept-Language",
    "Authorization", "Cache-Control", "Connection", "Content-Length",
    "Content-Type", "Cookie", "Date", "Host", "If-Match", "If-Modified-Since",
    "If-None-Match", "Origin", "Referer", "User-Agent",
];

const FALLBACK_CONTENT_TYPES: &[&str] = &[
    "application/json", "application/xml", "text/html", "text/plain",
    "text/css", "text/javascript", "application/pdf", "image/jpeg",
    "image/png", "image/gif", "application/octet-stream",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        assert!(!method().is_empty());
    }

    #[test]
    fn test_status_code() {
        assert!(!status_code().is_empty());
    }

    #[test]
    fn test_header() {
        assert!(!header().is_empty());
    }

    #[test]
    fn test_content_type() {
        assert!(!content_type().is_empty());
    }
}

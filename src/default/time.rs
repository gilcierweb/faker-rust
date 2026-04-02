//! Time generator - generates random dates and times

use crate::config::FakerConfig;
use chrono::{DateTime, Duration, Utc};
// Removed unused Rng

/// Generate a random date/time between two given dates
pub fn between(from: DateTime<Utc>, to: DateTime<Utc>) -> DateTime<Utc> {
    let config = FakerConfig::current();
    let from_ts = from.timestamp();
    let to_ts = to.timestamp();
    
    if from_ts >= to_ts {
        return from;
    }
    
    let ts = config.rand_range_i64(from_ts, to_ts);
    DateTime::from_timestamp(ts, 0).unwrap_or(from)
}

/// Generate a random date/time in the past (up to given number of days)
pub fn backward(days: i64) -> DateTime<Utc> {
    let now = Utc::now();
    let from = now - Duration::days(days);
    between(from, now)
}

/// Generate a random date/time in the future (up to given number of days)
pub fn forward(days: i64) -> DateTime<Utc> {
    let now = Utc::now();
    let to = now + Duration::days(days);
    between(now, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between() {
        let from = Utc::now() - Duration::days(10);
        let to = Utc::now();
        let result = between(from, to);
        assert!(result >= from && result <= to);
    }
}

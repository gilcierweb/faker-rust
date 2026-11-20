//! Time generator - generates random dates and times

use crate::config::FakerConfig;
use time::{Duration, OffsetDateTime};

/// Generate a random date/time between two given dates
pub fn between(from: OffsetDateTime, to: OffsetDateTime) -> OffsetDateTime {
    let config = FakerConfig::current();
    let from_ts = from.unix_timestamp();
    let to_ts = to.unix_timestamp();

    if from_ts >= to_ts {
        return from;
    }

    let ts = config.rand_range_i64(from_ts, to_ts);
    OffsetDateTime::from_unix_timestamp(ts).unwrap_or(from)
}

/// Generate a random date/time in the past (up to given number of days)
pub fn backward(days: i64) -> OffsetDateTime {
    let now = OffsetDateTime::now_utc();
    let from = now - Duration::days(days);
    between(from, now)
}

/// Generate a random date/time in the future (up to given number of days)
pub fn forward(days: i64) -> OffsetDateTime {
    let now = OffsetDateTime::now_utc();
    let to = now + Duration::days(days);
    between(now, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between() {
        let from = OffsetDateTime::now_utc() - Duration::days(10);
        let to = OffsetDateTime::now_utc();
        let result = between(from, to);
        assert!(result >= from && result <= to);
    }
}

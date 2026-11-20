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
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: formatted datetime string between two instants.
pub fn between_dates(from: &str, to: &str, _period: &str, format: Option<&str>) -> String {
    use time::format_description::well_known::Rfc3339;
    let from =
        time::OffsetDateTime::parse(from, &Rfc3339).unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
    let to = time::OffsetDateTime::parse(to, &Rfc3339).unwrap_or(from);
    let chosen = between(from, to);
    match format {
        Some(fmt) => chosen
            .format(
                &time::format_description::parse_borrowed::<2>(fmt).unwrap_or_else(|_| {
                    time::format_description::parse_borrowed::<2>("[year]-[month]-[day]").unwrap()
                }),
            )
            .unwrap_or_default(),
        None => chosen.format(&Rfc3339).unwrap_or_default(),
    }
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

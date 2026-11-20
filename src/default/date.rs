//! Date and time generator - generates random dates and times

use time::format_description::well_known::Iso8601;
use time::{Date, Duration, Month, OffsetDateTime};

/// Format a date as `YYYY-MM-DD`.
fn format_date(date: Date) -> String {
    date.format(&Iso8601::DATE)
        .unwrap_or_else(|_| "1970-01-01".to_string())
}

/// Parse a `YYYY-MM-DD` date, falling back to today on invalid input.
fn parse_or_today(value: &str) -> Date {
    Date::parse(value, &Iso8601::DATE).unwrap_or_else(|_| OffsetDateTime::now_utc().date())
}

/// Generate a random date in the past
pub fn backward(days: Option<i64>, _from: Option<&str>, _to: Option<&str>) -> String {
    let days_back = days.unwrap_or(365);
    let config = crate::config::FakerConfig::current();
    let offset = config.rand_range(0, days_back.max(0) as u32) as i64;
    let today = OffsetDateTime::now_utc().date();
    let target = today.checked_sub(Duration::days(offset)).unwrap_or(today);
    format_date(target)
}

/// Generate a random date in the future
pub fn forward(days: Option<i64>, _from: Option<&str>, _to: Option<&str>) -> String {
    let days_forward = days.unwrap_or(365);
    let config = crate::config::FakerConfig::current();
    let offset = config.rand_range(0, days_forward.max(0) as u32) as i64;
    let today = OffsetDateTime::now_utc().date();
    let target = today.checked_add(Duration::days(offset)).unwrap_or(today);
    format_date(target)
}

/// Generate a random date between two dates
pub fn between(from: &str, to: &str) -> String {
    let start = parse_or_today(from);
    let end = parse_or_today(to);

    let config = crate::config::FakerConfig::current();
    let range = (end - start).whole_days();
    if range <= 0 {
        return from.to_string();
    }

    let days_offset = config.rand_range(0, range as u32) as i64;
    let target = start
        .checked_add(Duration::days(days_offset))
        .unwrap_or(start);
    format_date(target)
}

/// Generate a random birthday
pub fn birthday(min_age: Option<i32>, max_age: Option<i32>) -> String {
    let config = crate::config::FakerConfig::current();
    let min = min_age.unwrap_or(18) as u32;
    let max = max_age.unwrap_or(65) as u32;
    let age = config.rand_range(min, max);

    let today = OffsetDateTime::now_utc().date();

    // Approximate birthday calculation
    let year = today.year() - age as i32;
    let day_of_year = config.rand_range(1, 365) as i64;

    let base_date = Date::from_calendar_date(year, Month::January, 1).unwrap_or(today);
    let target = base_date
        .checked_add(Duration::days(day_of_year - 1))
        .unwrap_or(base_date);

    format_date(target)
}

/// Generate a random date in a specific period
pub fn in_date_period(start_date: &str, end_date: &str, _format: Option<&str>) -> String {
    between(start_date, end_date)
}

/// Generate a random date on a specific day of week
pub fn on_day_of_week_between(_day_of_week: &str, from: &str, to: &str) -> String {
    // Simplified implementation - just return a random date between from and to
    between(from, to)
}
// --- gem parity generators (algorithmic) ---
/// Gem-compatible generator: date between two dates excluding one value.
pub fn between_except(from: &str, to: &str, excepted: &str) -> String {
    loop {
        let candidate = between(from, to);
        if candidate != excepted {
            return candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Date {
        Date::parse(s, &Iso8601::DATE).unwrap_or_else(|e| panic!("bad date {s}: {e}"))
    }

    #[test]
    fn backward_and_forward_stay_within_window() {
        let today = OffsetDateTime::now_utc().date();
        for _ in 0..50 {
            let back = parse(&backward(Some(7), None, None));
            let fwd = parse(&forward(Some(7), None, None));
            let back_days = (today - back).whole_days();
            let fwd_days = (fwd - today).whole_days();
            assert!((0..=7).contains(&back_days), "back {back_days}");
            assert!((0..=7).contains(&fwd_days), "fwd {fwd_days}");
        }
    }

    #[test]
    fn between_is_inclusive_of_bounds() {
        let start = "2020-01-01";
        let end = "2020-01-31";
        for _ in 0..50 {
            let d = parse(&between(start, end));
            assert!(d >= parse(start) && d <= parse(end), "date {d}");
        }
    }

    #[test]
    fn backward_is_deterministic_with_seed() {
        crate::config::FakerConfig::set_seed(11);
        let first = backward(Some(30), None, None);
        crate::config::FakerConfig::set_seed(11);
        let second = backward(Some(30), None, None);
        assert_eq!(first, second);
    }
}

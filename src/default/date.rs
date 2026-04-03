//! Date and time generator - generates random dates and times

use chrono::{Duration, NaiveDate, Utc};

/// Generate a random date in the past
pub fn backward(days: Option<i64>, _from: Option<&str>, _to: Option<&str>) -> String {
    let days_back = days.unwrap_or(365);
    let today = Utc::now().date_naive();
    let target = today - Duration::try_days(days_back).unwrap();
    target.format("%Y-%m-%d").to_string()
}

/// Generate a random date in the future
pub fn forward(days: Option<i64>, _from: Option<&str>, _to: Option<&str>) -> String {
    let days_forward = days.unwrap_or(365);
    let today = Utc::now().date_naive();
    let target = today + Duration::try_days(days_forward).unwrap();
    target.format("%Y-%m-%d").to_string()
}

/// Generate a random date between two dates
pub fn between(from: &str, to: &str) -> String {
    let start =
        NaiveDate::parse_from_str(from, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive());
    let end = NaiveDate::parse_from_str(to, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive());

    let config = crate::config::FakerConfig::current();
    let range = (end - start).num_days();
    if range <= 0 {
        return from.to_string();
    }

    let days_offset = config.rand_range(0, range as u32) as i64;
    let target = start + Duration::try_days(days_offset).unwrap();
    target.format("%Y-%m-%d").to_string()
}

/// Generate a random birthday
pub fn birthday(min_age: Option<i32>, max_age: Option<i32>) -> String {
    let config = crate::config::FakerConfig::current();
    let min = min_age.unwrap_or(18) as u32;
    let max = max_age.unwrap_or(65) as u32;
    let age = config.rand_range(min, max);

    let today = Utc::now().date_naive();
    let days_in_year = 365;

    // Approximate birthday calculation
    let year = today.format("%Y").to_string().parse::<i32>().unwrap() - age as i32;
    let day_of_year = config.rand_range(1, days_in_year as u32) as i64;

    let base_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or(today);
    let target = base_date + Duration::try_days(day_of_year - 1).unwrap();

    target.format("%Y-%m-%d").to_string()
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

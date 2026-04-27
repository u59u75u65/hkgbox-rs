//! Time conversion utilities
//!
//! This module provides functions for converting timestamps to formatted strings.

use chrono::{Local, Utc, TimeZone};

/// Convert millisecond timestamp to formatted date and time strings
///
/// # Arguments
/// * `ms` - Timestamp in milliseconds since Unix epoch
///
/// # Returns
/// Tuple of (date_string, time_string) in local timezone
///
/// # Examples
/// ```
/// use hkg::time_utils;
///
/// let (date, time) = time_utils::timestamp_to_strings(1609459200000);
/// assert!(date.contains("/"));
/// assert!(time.contains(":"));
/// ```
#[must_use]
pub fn timestamp_to_strings(ms: i64) -> (String, String) {
    // Convert milliseconds since epoch to local time
    let timestamp = ms / 1000;
    let dt_utc = Utc.timestamp_opt(timestamp, 0).unwrap();
    let dt_local = dt_utc.with_timezone(&Local);

    let date = dt_local.format("%d/%m/%Y").to_string();
    let time = dt_local.format("%H:%M").to_string();

    (date, time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_strings() {
        let (date, time) = timestamp_to_strings(1609459200000); // 2021-01-01 00:00:00 UTC
        assert!(!date.is_empty());
        assert!(!time.is_empty());
        assert!(date.contains("/"));
        assert!(time.contains(":"));
    }

    #[test]
    fn test_timestamp_format() {
        let (date, time) = timestamp_to_strings(0);
        // Date should be in DD/MM/YYYY format
        assert_eq!(date.len(), 10);
        // Time should be in HH:MM format
        assert_eq!(time.len(), 5);
    }
}

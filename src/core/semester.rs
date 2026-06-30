//! Semester computation from file modification timestamps.
//!
//! Maps a file's modification time to an academic semester string used as
//! a directory component in the output tree.

use chrono::{DateTime, Datelike, Local, TimeZone};
use std::time::SystemTime;

/// Compute the academic semester from a file's modification time.
///
/// Returns `"{year}-I"` for January–June and `"{year}-II"` for July–December.
/// Uses the local system timezone for date conversion.
///
/// # Fallback
/// If `modified` is before the Unix epoch (`duration_since` fails), the
/// current local date is used as a fallback.
///
/// # Example
///
/// ```rust
/// use sortcrab::core::semester::semester_from_time;
/// use std::time::SystemTime;
///
/// // January 2025 → "2025-I"
/// let jan = semester_from_time(&SystemTime::UNIX_EPOCH);
/// // Note: actual value depends on timezone
/// ```
pub fn semester_from_time(modified: &SystemTime) -> String {
    let datetime: DateTime<Local> = match modified.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => Local
            .timestamp_opt(duration.as_secs() as i64, 0)
            .earliest()
            .unwrap_or_else(Local::now),
        Err(_) => Local::now(),
    };

    let year = datetime.year();
    let month = datetime.month();

    if month <= 6 {
        format!("{}-I", year)
    } else {
        format!("{}-II", year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, NaiveDate, NaiveTime};
    use std::time::SystemTime;

    fn local_time(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> SystemTime {
        let naive_dt = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(hour, min, sec).unwrap());
        let local_dt: DateTime<Local> = Local.from_local_datetime(&naive_dt).unwrap();
        SystemTime::from(local_dt)
    }

    #[test]
    fn test_january() {
        let t = local_time(2025, 1, 15, 10, 0, 0);
        assert_eq!(semester_from_time(&t), "2025-I");
    }

    #[test]
    fn test_august() {
        let t = local_time(2025, 8, 20, 14, 30, 0);
        assert_eq!(semester_from_time(&t), "2025-II");
    }

    #[test]
    fn test_june_boundary() {
        let t = local_time(2025, 6, 30, 23, 59, 59);
        assert_eq!(semester_from_time(&t), "2025-I");
    }

    #[test]
    fn test_july_boundary() {
        let t = local_time(2025, 7, 1, 0, 0, 0);
        assert_eq!(semester_from_time(&t), "2025-II");
    }

    #[test]
    fn test_epoch() {
        // Use local_time to avoid timezone shift from UTC epoch
        let epoch = local_time(1970, 1, 1, 0, 0, 0);
        assert_eq!(semester_from_time(&epoch), "1970-I");
    }

    #[test]
    fn test_future_date() {
        let t = local_time(3000, 1, 1, 0, 0, 0);
        assert_eq!(semester_from_time(&t), "3000-I");
    }
}

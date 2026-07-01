//! Semester computation from file modification timestamps.
//!
//! Maps a file's modification time to a configurable period string used as
//! a directory component in the output tree.

use chrono::{DateTime, Datelike, Local, TimeZone};
use std::time::SystemTime;

/// Convert an integer to a Roman numeral (supports 1–12).
fn to_roman(n: u32) -> &'static str {
    match n {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        11 => "XI",
        12 => "XII",
        _ => "?",
    }
}

/// Compute a period label from a file's modification time.
///
/// The year and period are extracted from `modified` using the local timezone,
/// then formatted according to `folder_format`, which supports the variables
/// `{year}`, `{period}` (1-based numeric), and `{roman}` (Roman numeral).
///
/// `months_per_period` determines how many months each period spans
/// (6 = semesters, 4 = trimesters, 3 = quarters, 12 = yearly).
///
/// # Fallback
/// If `modified` is before the Unix epoch (`duration_since` fails), the
/// current local date is used as a fallback.
///
/// # Example
///
/// ```rust
/// use sortcrab::core::semester::semester_label;
/// use chrono::{Local, NaiveDate, NaiveTime, TimeZone};
/// use std::time::SystemTime;
///
/// // June 15 2025 at noon local time
/// let naive = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()
///     .and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap());
/// let dt: chrono::DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
/// let t = SystemTime::from(dt);
/// assert_eq!(semester_label(&t, 6, "{year}-{roman}"), "2025-I");
/// ```
pub fn semester_label(
    modified: &SystemTime,
    months_per_period: u32,
    folder_format: &str,
) -> String {
    let datetime: DateTime<Local> = match modified.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => Local
            .timestamp_opt(duration.as_secs() as i64, 0)
            .earliest()
            .unwrap_or_else(Local::now),
        Err(_) => Local::now(),
    };

    let year = datetime.year().to_string();
    let month = datetime.month();

    let period = ((month - 1) / months_per_period) + 1;

    folder_format
        .replace("{year}", &year)
        .replace("{period}", &period.to_string())
        .replace("{roman}", to_roman(period))
}

/// Legacy wrapper — equivalent to `semester_label(modified, 6, "{year}-{roman}")`.
///
/// Provided for backward compatibility. Prefer [`semester_label`] in new code.
pub fn semester_from_time(modified: &SystemTime) -> String {
    semester_label(modified, 6, "{year}-{roman}")
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

    // ── semester_from_time (legacy wrapper, months=6) ────────────────────

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
        let epoch = local_time(1970, 1, 1, 0, 0, 0);
        assert_eq!(semester_from_time(&epoch), "1970-I");
    }

    #[test]
    fn test_future_date() {
        let t = local_time(3000, 1, 1, 0, 0, 0);
        assert_eq!(semester_from_time(&t), "3000-I");
    }

    // ── semester_label with variable months_per_period ───────────────────

    #[test]
    fn test_label_semesters_default_format() {
        let jan = local_time(2025, 1, 15, 10, 0, 0);
        let jul = local_time(2025, 7, 1, 0, 0, 0);
        assert_eq!(semester_label(&jan, 6, "{year}-{roman}"), "2025-I");
        assert_eq!(semester_label(&jul, 6, "{year}-{roman}"), "2025-II");
    }

    #[test]
    fn test_label_quarters() {
        let jan = local_time(2025, 1, 15, 10, 0, 0);
        let apr = local_time(2025, 4, 1, 0, 0, 0);
        let jul = local_time(2025, 7, 1, 0, 0, 0);
        let oct = local_time(2025, 10, 1, 0, 0, 0);
        assert_eq!(semester_label(&jan, 3, "{year}Q{period}"), "2025Q1");
        assert_eq!(semester_label(&apr, 3, "{year}Q{period}"), "2025Q2");
        assert_eq!(semester_label(&jul, 3, "{year}Q{period}"), "2025Q3");
        assert_eq!(semester_label(&oct, 3, "{year}Q{period}"), "2025Q4");
    }

    #[test]
    fn test_label_trimesters() {
        let jan = local_time(2025, 1, 15, 10, 0, 0);
        let may = local_time(2025, 5, 1, 0, 0, 0);
        let sep = local_time(2025, 9, 1, 0, 0, 0);
        assert_eq!(semester_label(&jan, 4, "{year}-S{period}"), "2025-S1");
        assert_eq!(semester_label(&may, 4, "{year}-S{period}"), "2025-S2");
        assert_eq!(semester_label(&sep, 4, "{year}-S{period}"), "2025-S3");
    }

    #[test]
    fn test_label_yearly() {
        let jan = local_time(2025, 1, 15, 10, 0, 0);
        let dec = local_time(2025, 12, 1, 0, 0, 0);
        assert_eq!(semester_label(&jan, 12, "{year}"), "2025");
        assert_eq!(semester_label(&dec, 12, "{year}"), "2025");
    }

    #[test]
    fn test_label_roman_format() {
        let jan = local_time(2025, 1, 15, 10, 0, 0);
        let feb = local_time(2025, 2, 1, 0, 0, 0);
        assert_eq!(semester_label(&jan, 1, "{roman}"), "I");
        assert_eq!(semester_label(&feb, 1, "{roman}"), "II");
    }

    #[test]
    fn test_label_custom_format_only_period() {
        let jan = local_time(2025, 1, 15, 10, 0, 0);
        assert_eq!(semester_label(&jan, 6, "period-{period}"), "period-1");
    }
}

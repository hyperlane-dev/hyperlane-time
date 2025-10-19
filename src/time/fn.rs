use crate::*;

/// Leap Year
pub const LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// Common Year
pub const COMMON_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// Days
pub const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
/// Months
pub const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Determines if a year is a leap year.
///
/// # Arguments
///
/// - `u64` - The year to check.
///
/// # Returns
///
/// - `bool` - Whether the year is a leap year.
#[inline]
pub fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Gets the current time, including the date and time.
///
/// # Returns
///
/// - `String` - The formatted time as "YYYY-MM-DD HH:MM:SS"
pub fn time() -> String {
    let (year, month, day, hour, minute, second, _, _) = calculate_time();
    let mut date_time: String = String::new();
    write!(
        &mut date_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hour, minute, second
    )
    .unwrap_or_default();
    date_time
}

/// Gets the current day, without the time.
///
/// # Returns
///
/// - `String` - The formatted date as "YYYY-MM-DD"
pub fn date() -> String {
    let (year, month, day, _, _, _, _, _) = calculate_time();
    let mut date_time: String = String::new();
    write!(&mut date_time, "{:04}-{:02}-{:02}", year, month, day).unwrap_or_default();
    date_time
}

/// Computes the year, month, and day from days since Unix epoch (1970-01-01).
///
/// # Arguments
///
/// - `u64` - Number of days since Unix epoch.
///
/// # Returns
///
/// - `(u64, u64, u64)` - Tuple containing year, month and day.
pub fn compute_date(mut days_since_epoch: u64) -> (u64, u64, u64) {
    let mut year: u64 = 1970;
    loop {
        let days_in_year: u64 = if is_leap_year(year) { 366 } else { 365 };
        if days_since_epoch < days_in_year {
            break;
        }
        days_since_epoch -= days_in_year as u64;
        year += 1;
    }
    let mut month: u64 = 0;
    for (i, &days) in COMMON_YEAR.iter().enumerate() {
        let days_in_month = if i == 1 && is_leap_year(year) {
            days + 1
        } else {
            days
        };
        if days_since_epoch < days_in_month as u64 {
            month = i as u64 + 1;
            return (year, month, days_since_epoch + 1);
        }
        days_since_epoch -= days_in_month as u64;
    }

    (year, month, 1)
}

/// Gets the current date and time in GMT format.
///
/// # Returns
///
/// - `String` - The current date and time in GMT format.
pub fn gmt() -> String {
    let now: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = now.duration_since(UNIX_EPOCH).unwrap();
    let timestamp: u64 = duration_since_epoch.as_secs();
    let seconds_in_day: u64 = 86_400;
    let days_since_epoch: u64 = timestamp / seconds_in_day;
    let seconds_of_day: u64 = timestamp % seconds_in_day;
    let hours: u64 = seconds_of_day / 3600;
    let minutes: u64 = (seconds_of_day % 3600) / 60;
    let seconds: u64 = seconds_of_day % 60;
    let (year, month, day) = compute_date(days_since_epoch);
    let weekday: usize = ((days_since_epoch + 4) % 7) as usize;
    format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} GMT",
        DAYS[weekday],
        day,
        MONTHS[month as usize - 1],
        year,
        hours,
        minutes,
        seconds
    )
}

/// Gets the current year.
///
/// # Returns
///
/// - `u64` - The current year
pub fn year() -> u64 {
    calculate_time().0
}

/// Gets the current month.
///
/// # Returns
///
/// - `u64` - The current month (1-12)
pub fn month() -> u64 {
    calculate_time().1
}

/// Gets the current day.
///
/// # Returns
///
/// - `u64` - The current day of the month
pub fn day() -> u64 {
    calculate_time().2
}

/// Gets the current hour.
///
/// # Returns
///
/// - `u64` - The current hour (0-23)
pub fn hour() -> u64 {
    calculate_time().3
}

/// Gets the current minute.
///
/// # Returns
///
/// - `u64` - The current minute (0-59)
pub fn minute() -> u64 {
    calculate_time().4
}

/// Gets the current second.
///
/// # Returns
///
/// - `u64` - The current second (0-59)
pub fn second() -> u64 {
    calculate_time().5
}

/// Gets the current timestamp in milliseconds.
///
/// # Returns
///
/// - `u64` - The current timestamp in milliseconds since Unix epoch
pub fn millis() -> u64 {
    calculate_time().6
}

/// Gets the current timestamp in microseconds.
///
/// # Returns
///
/// - `u64` - The current timestamp in microseconds since Unix epoch
pub fn micros() -> u64 {
    calculate_time().7
}

/// Calculates the current year, month, day, hour, minute, second, millisecond and microsecond.
///
/// # Returns
///
/// - `(u64, u64, u64, u64, u64, u64, u64, u64)` - Tuple containing:
///   - Year
///   - Month
///   - Day
///   - Hour (0-23)
///   - Minute (0-59)
///   - Second (0-59)
///   - Milliseconds in current second
///   - Microseconds in current second
pub fn calculate_time() -> (u64, u64, u64, u64, u64, u64, u64, u64) {
    let start: SystemTime = SystemTime::now();
    let duration: Duration = start.duration_since(UNIX_EPOCH).unwrap();
    let total_seconds: u64 = duration.as_secs();
    let nanoseconds: u64 = duration.subsec_nanos() as u64;
    let milliseconds: u64 = nanoseconds / 1_000_000;
    let microseconds: u64 = nanoseconds / 1_000;
    let mut total_days: u64 = total_seconds / 86400;
    let mut year: u64 = 1970;
    while total_days >= if is_leap_year(year) { 366 } else { 365 } {
        total_days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }
    let mut month: u64 = 1;
    let month_days: [u64; 12] = if is_leap_year(year) {
        LEAP_YEAR
    } else {
        COMMON_YEAR
    };
    while total_days >= month_days[month as usize - 1] {
        total_days -= month_days[month as usize - 1];
        month += 1;
    }
    let day: u64 = total_days + 1;
    let remaining_seconds: u64 = total_seconds % 86400;
    let timezone_offset: u64 = from_env_var().value();
    let hour: u64 = ((remaining_seconds + timezone_offset) / 3600) % 24;
    let minute: u64 = (remaining_seconds % 3600) / 60;
    let second: u64 = remaining_seconds % 60;
    (
        year,
        month,
        day,
        hour,
        minute,
        second,
        milliseconds,
        microseconds,
    )
}

/// Gets the current time with milliseconds, including the date and time.
///
/// # Returns
///
/// - `String` - The formatted time as "YYYY-MM-DD HH:MM:SS.sss"
pub fn time_millis() -> String {
    let (year, month, day, hour, minute, second, millisecond, _) = calculate_time();
    let mut date_time: String = String::new();
    write!(
        &mut date_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        year, month, day, hour, minute, second, millisecond
    )
    .unwrap_or_default();
    date_time
}

/// Gets the current time with microseconds, including the date and time.
///
/// # Returns
///
/// - `String` - The formatted time as "YYYY-MM-DD HH:MM:SS.ssssss"
pub fn time_micros() -> String {
    let (year, month, day, hour, minute, second, _, microseconds) = calculate_time();
    let mut date_time: String = String::new();
    write!(
        &mut date_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
        year, month, day, hour, minute, second, microseconds
    )
    .unwrap_or_default();
    date_time
}

/// Gets the current timestamp in seconds since Unix epoch.
///
/// # Returns
///
/// - `u64` - The current timestamp in seconds
pub fn timestamp() -> u64 {
    let timezone_offset: u64 = from_env_var().value();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .saturating_add(timezone_offset)
}

/// Gets the current timestamp in milliseconds since Unix epoch.
///
/// # Returns
///
/// - `u64` - The current timestamp in milliseconds
pub fn timestamp_millis() -> u64 {
    let timezone_offset: u64 = from_env_var().value();
    let duration: Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.as_secs().saturating_add(timezone_offset)) * 1000 + duration.subsec_millis() as u64
}

/// Gets the current timestamp in microseconds since Unix epoch.
///
/// # Returns
///
/// - `u64` - The current timestamp in microseconds
pub fn timestamp_micros() -> u64 {
    let timezone_offset: u64 = from_env_var().value();
    let duration: Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.as_secs().saturating_add(timezone_offset)) * 1_000_000
        + duration.subsec_micros() as u64
}

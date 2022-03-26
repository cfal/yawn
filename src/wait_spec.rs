use std::fmt::{Display, Formatter};
use std::time::Duration;

use chrono::{DateTime, Local, NaiveTime, TimeZone};

#[derive(Debug, Clone)]
pub enum WaitSpec {
    Duration(Duration),
    DateTime(DateTime<Local>),
    NaiveTime(NaiveTime)
}

impl TryFrom<&str> for WaitSpec {
    type Error = String;
    fn try_from(value: &str) -> std::result::Result<WaitSpec, String> {
        if let Ok(d) = parse_duration_str(value) {
            return Ok(WaitSpec::Duration(d));
        }

        if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
            if dt < Local::now() {
                return Err(format!("rfc3339 date is in the past: {}", dt));
            }

            return Ok(WaitSpec::DateTime(dt.with_timezone(&Local)));
        }

        if let Ok(dt) = DateTime::parse_from_rfc2822(value) {
            if dt < Local::now() {
                return Err(format!("rfc2822 date is in the past: {}", dt));
            }

            return Ok(WaitSpec::DateTime(dt.with_timezone(&Local)));
        }

        for fmt in LOCAL_DATETIME_FORMATS {
            if let Ok(dt) = Local.datetime_from_str(value, fmt) {
                if dt < Local::now() {
                    return Err(format!(
                        "Date string ({}) is in the past: {}",
                        fmt, dt
                    ));
                }

                return Ok(WaitSpec::DateTime(dt));
            }
        }

        for fmt in NAIVE_TIME_FORMATS {
            if let Ok(nt) = NaiveTime::parse_from_str(value, fmt) {
                return Ok(WaitSpec::NaiveTime(nt));
            }
        }

        Err(String::from("Unknown duration string"))
    }
}

impl Display for WaitSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use WaitSpec::*;
        match self {
            Duration(d) => write!(f, "duration ({}s)", d.as_secs()),
            DateTime(dt) => write!(f, "exact date/time ({})", dt),
            NaiveTime(nt) => write!(f, "time ({})", nt)
        }
    }
}

// TODO: there's probably a cleaner/faster way than passing each of these to
// Local.datetime_from_str.
const LOCAL_DATETIME_FORMATS: &[&str] = &[
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M",
];

const NAIVE_TIME_FORMATS: &[&str] = &["%H:%M:%S", "%H:%M", "%H:%M:%S%.3f", "%l:%M %P"];

pub fn parse_duration_str(value: &str) -> std::result::Result<Duration, &str> {
    // if it's a simple number, parse it as seconds.
    if let Ok(s) = value.parse::<u64>() {
        return Ok(Duration::from_secs(s));
    }

    if let Ok(f) = value.parse::<f64>() {
        return Ok(Duration::from_nanos((f * 1e9) as u64));
    }

    parse_duration_str_with_units(value)
}

fn parse_duration_str_with_units(value: &str) -> std::result::Result<Duration, &str> {
    #[derive(PartialOrd, PartialEq)]
    enum Unit {
        Milliseconds,
        Seconds,
        Minutes,
        Hours,
        Days,
    }

    use Unit::*;

    let value_bytes = value.as_bytes();
    let mut total_duration = 0u64;
    let mut last_unit = None;
    let mut i = 0;
    let n = value.len();

    while i < n {
        let mut j = i;
        while value_bytes[j] >= b'0' && value_bytes[j] <= b'9' {
            j += 1;
            if j == n {
                // no units followed the number
                return Err("No unit following number");
            }
        }

        let mut duration = std::str::from_utf8(&value_bytes[i..j])
            .unwrap()
            .parse::<u64>()
            .map_err(|_| "Invalid value")?;

        let unit = match value_bytes[j] {
            b'd' => {
                duration = duration * 24 * 60 * 60 * 1000;
                Days
            }
            b'h' => {
                duration = duration * 60 * 60 * 1000;
                Hours
            }
            b'm' => {
                if j < n - 1 && value_bytes[j + 1] == b's' {
                    j += 1;
                    Milliseconds
                } else {
                    duration = duration * 60 * 1000;
                    Minutes
                }
            }
            b's' => {
                duration = duration * 1000;
                Seconds
            }
            _ => {
                return Err("Invalid unit");
            }
        };

        if let Some(l) = last_unit {
            if l <= unit {
                return Err("Invalid unit ordering");
            }
        }

        total_duration = total_duration
            .checked_add(duration)
            .ok_or("Duration overflow")?;
        last_unit = Some(unit);

        i = j + 1;
    }

    Ok(Duration::from_millis(total_duration))
}

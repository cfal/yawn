use std::time::Duration;

use chrono::offset::LocalResult;
use chrono::{DateTime, Local, NaiveTime, TimeZone};
use log::warn;

pub trait ToDuration {
    fn to_duration(self) -> Duration;
}

impl ToDuration for DateTime<Local> {
    fn to_duration(self) -> Duration {
        // When creating the date/time, we check that it's not in the past, but incase the
        // provided timestamp is very close to the future and we've already passed it,
        // to_std would return an error. Ignore it since this just means we dont have to wait.
        match self.signed_duration_since(Local::now()).to_std() {
            Ok(duration) => duration,
            Err(e) => {
                warn!(
                    "Duration could not be converted, probably in the past: {}",
                    e
                );
                Duration::ZERO
            }
        }
    }
}

impl ToDuration for NaiveTime {
    fn to_duration(self) -> Duration {
        let now = Local::now();
        let mut current_date = now.naive_local().date();
        if now.naive_local().time() > self {
            // This must be for the next day.
            current_date += chrono::Duration::days(1);
        }
        let dt = match Local.from_local_datetime(&current_date.and_time(self)) {
            LocalResult::Single(dt) => dt,
            LocalResult::None => {
                panic!("Invalid local time representation");
            }
            LocalResult::Ambiguous(dt1, dt2) => {
                panic!(
                    "Ambiguous local time representation: '{}' or '{}'",
                    dt1, dt2
                );
            }
        };
        dt.to_duration()
    }
}

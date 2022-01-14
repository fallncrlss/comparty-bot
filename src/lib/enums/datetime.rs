use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use std::str::FromStr;

const DAY_TIME_UNIT: &str = "d";
const HOUR_TIME_UNIT: &str = "h";
const MINUTE_TIME_UNIT: &str = "m";
const SECOND_TIME_UNIT: &str = "s";
const TIME_UNITS: [&str; 4] = [
    DAY_TIME_UNIT,
    HOUR_TIME_UNIT,
    MINUTE_TIME_UNIT,
    SECOND_TIME_UNIT,
];

pub enum TimeUnits {
    Day(i32),
    Hour(i32),
    Minute(i32),
    Second(i32),
}

impl FromStr for TimeUnits {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<TimeUnits, Self::Err> {
        let regexp = &format!(r"^(\d+)([{}])+$", TIME_UNITS.join(","));
        let re = regex::Regex::new(regexp).unwrap();
        let substring = re.captures(input).unwrap();
        let amount: i32 = substring[1].parse().unwrap();
        let unit: &str = &substring[2];
        match unit {
            DAY_TIME_UNIT => Ok(TimeUnits::Day(amount)),
            HOUR_TIME_UNIT => Ok(TimeUnits::Hour(amount)),
            MINUTE_TIME_UNIT => Ok(TimeUnits::Minute(amount)),
            SECOND_TIME_UNIT => Ok(TimeUnits::Second(amount)),
            _ => Err(anyhow::Error::msg("Unable convert to time unit".to_string())),
        }
    }
}

impl TimeUnits {
    pub fn to_duration(&self) -> Duration {
        match &self {
            TimeUnits::Day(amount) => chrono::Duration::days(*amount as i64),
            TimeUnits::Hour(amount) => chrono::Duration::hours(*amount as i64),
            TimeUnits::Minute(amount) => chrono::Duration::minutes(*amount as i64),
            TimeUnits::Second(amount) => chrono::Duration::seconds(*amount as i64),
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            TimeUnits::Day(amount) => format!("{}{}", amount, DAY_TIME_UNIT),
            TimeUnits::Hour(amount) => format!("{}{}", amount, HOUR_TIME_UNIT),
            TimeUnits::Minute(amount) => format!("{}{}", amount, MINUTE_TIME_UNIT),
            TimeUnits::Second(amount) => format!("{}{}", amount, SECOND_TIME_UNIT),
        }
    }

    pub fn to_expire_date(&self, date: i64) -> DateTime<Utc> {
        let current_timestamp = NaiveDateTime::from_timestamp(date, 0);
        DateTime::<Utc>::from_utc(current_timestamp, Utc) + self.to_duration()
    }
}

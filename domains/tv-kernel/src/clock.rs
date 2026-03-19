use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use std::sync::{Arc, Mutex};

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Debug, Default, Clone)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[derive(Debug, Clone)]
pub struct FrozenClock {
    inner: Arc<Mutex<DateTime<Utc>>>,
}

impl FrozenClock {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(at)),
        }
    }
    pub fn advance(&self, d: Duration) {
        let mut g = self.inner.lock().expect("clock lock");
        *g = *g + d;
    }
    pub fn set(&self, at: DateTime<Utc>) {
        *self.inner.lock().expect("clock lock") = at;
    }
}

impl Clock for FrozenClock {
    fn now(&self) -> DateTime<Utc> {
        *self.inner.lock().expect("clock lock")
    }
}

/// Business-day helpers used by settlement and SLA modules.
pub fn is_weekday(ts: DateTime<Utc>) -> bool {
    let w = ts.weekday().number_from_monday();
    w <= 5
}

pub fn next_business_day(ts: DateTime<Utc>) -> DateTime<Utc> {
    let mut cur = ts + Duration::days(1);
    while !is_weekday(cur) {
        cur = cur + Duration::days(1);
    }
    cur
}

pub fn add_business_days(ts: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    let mut cur = ts;
    let mut left = days;
    while left > 0 {
        cur = next_business_day(cur);
        left -= 1;
    }
    cur
}

pub fn elapsed_business_hours(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    if end <= start {
        return 0;
    }
    let mut hours = 0i64;
    let mut cursor = start;
    while cursor < end {
        if is_weekday(cursor) {
            let hour = cursor.time().hour();
            if (9..17).contains(&hour) {
                hours += 1;
            }
        }
        cursor = cursor + Duration::hours(1);
    }
    hours
}

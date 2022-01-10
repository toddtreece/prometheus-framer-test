use std::collections::HashMap;

use chrono::prelude::*;
use chrono::Duration;

pub const ONE_SECOND_MS: i64 = 1_000;
pub const ONE_SECOND_NS: i64 = 1_000_000_000;
pub const ONE_MILLISECOND_NS: i64 = 1_000_000;

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub step: Duration,
}

impl TimeRange {
    fn align(&self, timestamp: DateTime<Utc>) -> DateTime<Utc> {
        let nanos = (timestamp.timestamp_millis() / self.step.num_milliseconds())
            * self.step.num_milliseconds()
            * ONE_MILLISECOND_NS;

        Utc.timestamp(
            nanos / ONE_SECOND_NS,
            ((nanos % ONE_SECOND_NS) * ONE_SECOND_NS) as u32,
        )
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.align(self.start)
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.align(self.end)
    }

    pub fn generate(&self) -> (Vec<Option<DateTime<Utc>>>, HashMap<i64, usize>) {
        (self.start().timestamp_millis()..=self.end().timestamp_millis())
            .step_by(self.step.num_milliseconds() as usize)
            .enumerate()
            .fold(
                (vec![], HashMap::new()),
                |mut acc: (Vec<Option<DateTime<Utc>>>, HashMap<i64, usize>), v: (usize, i64)| {
                    acc.0.push(Some(Utc.timestamp(
                        v.1 / ONE_SECOND_MS,
                        ((v.1 % ONE_SECOND_MS) * ONE_SECOND_NS) as u32,
                    )));
                    acc.1.insert(v.1 * ONE_MILLISECOND_NS, v.0);
                    acc
                },
            )
    }
}

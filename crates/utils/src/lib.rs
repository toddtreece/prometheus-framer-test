use std::collections::HashMap;

use chrono::{prelude::*, Duration};
use core::{RangeVector, Sample, TimeRange};

pub fn generate_range_vectors(
    series_count: usize,
    row_count: usize,
) -> (TimeRange, Vec<RangeVector>) {
    let time_range = TimeRange {
        start: Utc.timestamp(0, 0),
        end: Utc.timestamp(row_count as i64, 0),
        step: Duration::seconds(1),
    };

    let rv: Vec<RangeVector> = (0..series_count)
        .map(|i| {
            let samples = (time_range.start().timestamp_millis()
                ..=(time_range.end().timestamp_millis()))
                .step_by(time_range.step.num_milliseconds() as usize)
                .filter_map(|j| {
                    if (j / 1000) as usize % (i + 1) != 0 {
                        return None;
                    }
                    Some(Sample {
                        timestamp: (j / 1000) as f64,
                        value: (((j / 1000) as usize / (i + 1)) as f64).to_string(),
                    })
                })
                .collect();

            let mut metric = HashMap::new();
            metric.insert("__name__".to_owned(), format!("every_{}", i));
            RangeVector { metric, samples }
        })
        .collect();

    (time_range, rv)
}

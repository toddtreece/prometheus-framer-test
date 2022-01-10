use chrono::prelude::*;
use grafana_plugin_sdk::data::Frame;
use grafana_plugin_sdk::prelude::*;
use rayon::prelude::*;

use crate::{RangeVector, TimeRange};

pub struct Matrix<'a>(&'a [RangeVector]);

impl<'a> Matrix<'a> {
    pub fn from(rv: &'a [RangeVector]) -> Self {
        Self(rv)
    }

    pub fn to_frames(&self, time_range: &TimeRange) -> Vec<Frame> {
        let (timestamps, time_map) = time_range.generate();

        self.0
            .par_iter()
            .map(|rv| {
                let mut values: Vec<Option<f64>> = vec![None; timestamps.len()];
                for s in rv.samples().iter() {
                    let ts = Utc.timestamp(s.timestamp() as i64, 0);
                    if let Some(idx) = time_map.get(&ts.timestamp_nanos()) {
                        if let Ok(val) = s.value().parse() {
                            values[*idx] = Some(val);
                        };
                    }
                }
                let value_field = values.into_opt_field("Value");
                let time_field = timestamps.clone().into_opt_field("Time");
                Frame::from_fields(rv.format_label(), [time_field, value_field])
            })
            .collect()
    }
}

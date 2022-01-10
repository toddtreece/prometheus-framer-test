use std::collections::HashMap;

use crate::Sample;

#[derive(Debug, Clone)]
pub struct RangeVector {
    pub metric: HashMap<String, String>,
    pub samples: Vec<Sample>,
}

impl RangeVector {
    pub fn metric(&self) -> &HashMap<String, String> {
        &self.metric
    }

    pub fn format_label(&self) -> String {
        self.metric
            .get("__name__")
            .unwrap_or(&"".to_owned())
            .to_owned()
    }

    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }
}

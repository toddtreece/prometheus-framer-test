#[derive(Debug, Clone)]
pub struct Sample {
    pub timestamp: f64,
    pub value: String,
}

impl Sample {
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

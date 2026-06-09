use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
}

impl Metrics {
    pub fn incr(&mut self, name: &str, by: u64) {
        *self.counters.entry(name.into()).or_insert(0) += by;
    }
    pub fn gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.into(), value);
    }
    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }
    pub fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({
            "counters": self.counters,
            "gauges": self.gauges,
        })
    }
}

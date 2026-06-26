// src/state.rs
pub struct DashboardState {
    pub worker_id: String,
    pub active_threads: u32,
    pub total_cycles: u64,
    pub event_log: Vec<String>,
}

impl DashboardState {
    pub fn new(id: &str) -> Self {
        Self {
            worker_id: id.to_string(),
            active_threads: 4,
            total_cycles: 0,
            event_log: vec!["System initialized.".to_string()],
        }
    }

    pub fn increment_cycles(&mut self, amount: u64) {
        self.total_cycles += amount;
    }

    pub fn push_log(&mut self, message: String) {
        self.event_log.push(message);
        if self.event_log.len() > 10 {
            self.event_log.remove(0); // Maintain fixed visual history window
        }
    }
}

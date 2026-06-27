// src/state.rs
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct XmrigSummary {
    pub hashrate: HashrateInfo,
    pub results: ResultsInfo,
    pub connection: ConnectionInfo,
}

#[derive(Deserialize, Debug)]
pub struct HashrateInfo {
    pub total: Vec<f32>, // [10s, 60s, 15m] hashrate
}

#[derive(Deserialize, Debug)]
pub struct ResultsInfo {
    pub shares_good: u32,
    pub shares_total: u32,
}

#[derive(Deserialize, Debug)]
pub struct ConnectionInfo {
    pub pool: String,
    pub ping: u32,
}

pub struct DashboardState {
    pub worker_id: String,
    pub hashrate: f32,
    pub accepted_shares: u32,
    pub pool_url: String,
    pub latency_ms: u32,
    pub event_log: Vec<String>,
}

impl DashboardState {
    pub fn new(id: &str) -> Self {
        Self {
            worker_id: id.to_string(),
            hashrate: 0.0,
            accepted_shares: 0,
            pool_url: "Connecting...".to_string(),
            latency_ms: 0,
            event_log: vec!["System initialized. Awaiting backend connection...".to_string()],
        }
    }

    pub fn poll_backend(&mut self) {
        // Query the local XMRig API endpoint
        match reqwest::blocking::get("http://127.0.0.1:2222/1/summary") {
            Ok(response) => {
                if let Ok(summary) = response.json::<XmrigSummary>() {
                    self.hashrate = summary.hashrate.total.first().cloned().unwrap_or(0.0);
                    self.accepted_shares = summary.results.shares_good;
                    self.pool_url = summary.connection.pool;
                    self.latency_ms = summary.connection.ping;
                    
                    if self.event_log.len() < 2 || self.event_log.last().unwrap().contains("Error") {
                        self.push_log("Successfully synchronized with XMRig engine.".to_string());
                    }
                }
            }
            Err(_) => {
                self.hashrate = 0.0;
                self.push_log("Error: Cannot reach XMRig backend daemon on port 2222.".to_string());
            }
        }
    }

    pub fn push_log(&mut self, message: String) {
        self.event_log.push(message);
        if self.event_log.len() > 8 {
            self.event_log.remove(0);
        }
    }
}

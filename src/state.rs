// src/state.rs
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct XmrigSummary {
    pub hashrate: HashrateInfo,
    pub results: ResultsInfo,
    pub connection: ConnectionInfo,
    pub cpu: CpuInfo,
}

#[derive(Deserialize, Debug)]
pub struct HashrateInfo {
    pub total: Vec<f32>,
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

#[derive(Deserialize, Debug)]
pub struct CpuInfo {
    pub brand: String,
}

pub struct DashboardState {
    pub worker_id: String,
    pub device_name: String,
    pub hashrate: f32,
    pub accepted_shares: u32,
    pub total_shares: u32,
    pub pool_url: String,
    pub latency_ms: u32,
    pub event_log: Vec<String>,
}

impl DashboardState {
    pub fn new(id: &str) -> Self {
        Self {
            worker_id: id.to_string(),
            device_name: "Detecting CPU...".to_string(),
            hashrate: 0.0,
            accepted_shares: 0,
            total_shares: 0,
            pool_url: "Disconnected".to_string(),
            latency_ms: 0,
            event_log: vec!["System initiated. Synchronizing with background daemon...".to_string()],
        }
    }

    pub fn poll_backend(&mut self) {
        match reqwest::blocking::get("http://127.0.0.1:2222/1/summary") {
            Ok(response) => {
                if let Ok(summary) = response.json::<XmrigSummary>() {
                    self.hashrate = summary.hashrate.total.first().cloned().unwrap_or(0.0);
                    self.accepted_shares = summary.results.shares_good;
                    self.total_shares = summary.results.shares_total;
                    self.pool_url = summary.connection.pool;
                    self.latency_ms = summary.connection.ping;
                    self.device_name = summary.cpu.brand;
                    
                    if self.event_log.len() < 2 || self.event_log.last().unwrap().contains("Offline") {
                        self.push_log("Network established with XMRig API daemon.".to_string());
                    }
                }
            }
            Err(_) => {
                self.hashrate = 0.0;
                self.pool_url = "Disconnected".to_string();
                self.push_log("Offline Error: Cannot connect to loopback port 2222.".to_string());
            }
        }
    }

    pub fn push_log(&mut self, message: String) {
        self.event_log.push(message);
        if self.event_log.len() > 6 {
            self.event_log.remove(0);
        }
    }
}

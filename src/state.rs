// src/state.rs
use serde::Deserialize;

// These structs match the exact JSON payload that XMRig's local HTTP API outputs
#[derive(Deserialize, Debug, Default)]
pub struct XmrigResponse {
    pub connection: ConnectionInfo,
    pub hashrate: HashrateInfo,
    pub results: ResultsInfo,
}

#[derive(Deserialize, Debug, Default)]
pub struct ConnectionInfo {
    pub pool: String,
    pub uptime: u64,
    pub ping: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct HashrateInfo {
    pub total: Vec<Option<f64>>, // Index 0 gives the 10-second hashrate average
}

#[derive(Deserialize, Debug, Default)]
pub struct ResultsInfo {
    pub shares_good: u64,
    pub shares_total: u64,
}

pub struct DashboardState {
    pub worker_id: String,
    pub hashrate: f64,
    pub pool: String,
    pub ping: u64,
    pub uptime: u64,
    pub shares_verified: String,
    pub status: String,
}

impl DashboardState {
    pub fn new(worker_id: &str) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            hashrate: 0.0,
            pool: "Connecting...".to_string(),
            ping: 0,
            uptime: 0,
            shares_verified: "0 / 0".to_string(),
            status: "System Initiated. Synchronizing...".to_string(),
        }
    }

    // This method polls the local XMRig API and pulls all the values into the UI state
    pub fn poll_backend(&mut self) {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(800)) // Snappy response timeout
            .build();

        if let Ok(cli) = client {
            // Talk directly to the local backend port we configured in main.rs
            match cli.get("http://127.0.0.1:2222/1/summary").send() {
                Ok(response) => {
                    if let Ok(data) = response.json::<XmrigResponse>() {
                        // Extract and set values into the UI properties safely
                        self.hashrate = data.hashrate.total.first().and_then(|x| *x).unwrap_or(0.0);
                        self.pool = data.connection.pool;
                        self.ping = data.connection.ping;
                        self.uptime = data.connection.uptime;
                        self.shares_verified = format!("{}/{}", data.results.shares_good, data.results.shares_total);
                        self.status = "Online".to_string();
                    } else {
                        self.set_offline("Parsing Error: Invalid Payload Structure.");
                    }
                }
                Err(_) => {
                    self.set_offline("Offline Error: Cannot connect to loopback port 2222.");
                }
            }
        } else {
            self.set_offline("System Error: Client initialization failed.");
        }
    }

    fn set_offline(&mut self, error_msg: &str) {
        self.hashrate = 0.0;
        pub fn hashrate(&self) -> f64 { self.hashrate }
        self.status = error_msg.to_string();
    }
}

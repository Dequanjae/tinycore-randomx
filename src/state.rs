// src/state.rs
pub struct DashboardState {
    pub worker_id: String,
    pub hashrate: f64,
    pub pool: String,
    pub ping: u64,
    pub uptime: u64,
    pub shares_verified: String,
    pub status: String,
    pub event_log: Vec<String>,
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
            event_log: vec!["System interface brought online safely.".to_string()],
        }
    }

    pub fn poll_backend(&mut self) {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(800))
            .build();

        if let Ok(cli) = client {
            match cli.get("http://127.0.0.1:2222/1/summary").send() {
                Ok(response) => {
                    // FIXED: Parse as a dynamic JSON object value to bypass structural parsing strictness
                    if let Ok(json) = response.json::<serde_json::Value>() {
                        
                        // Safely pull the hashrate array values dynamically
                        if let Some(hashrates) = json["hashrate"]["total"].as_array() {
                            self.hashrate = hashrates.first()
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.0);
                        }

                        // Pull connection fields safely
                        if let Some(pool_str) = json["connection"]["pool"].as_str() {
                            self.pool = pool_str.to_string();
                        }
                        
                        self.ping = json["connection"]["ping"].as_u64().unwrap_or(0);
                        self.uptime = json["connection"]["uptime"].as_u64().unwrap_or(0);

                        // Pull share submission counts safely
                        let good = json["results"]["shares_good"].as_u64().unwrap_or(0);
                        let total = json["results"]["shares_total"].as_u64().unwrap_or(0);
                        self.shares_verified = format!("{} / {}", good, total);

                        self.status = "Online".to_string();
                    } else {
                        self.set_offline("Parsing Error: Unexpected API response payload configuration.");
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
        self.status = error_msg.to_string();
        if self.event_log.len() > 8 { self.event_log.remove(0); }
        
        // Prevent spamming the dashboard with identical offline warnings
        let formatted_err = format!("[!] Keep-alive warning: {}", error_msg);
        if self.event_log.last() != Some(&formatted_err) {
            self.event_log.push(formatted_err);
        }
    }
}

// src/ui.rs
use crate::state::DashboardState;

pub fn render(state: &DashboardState) {
    // Return cursor to home position
    print!("\x1B[H"); 
    println!("=================================================================");
    println!(" 👑  TINY CORE LINUX LIVE PRODUCER MONITOR   ");
    println!("=================================================================");
    println!(" 💻 Hardware Device  : {}", state.device_name);
    println!(" 🆔 Worker Target ID : {}", state.worker_id);
    println!(" 🌐 Endpoint Network : {}", state.pool_url);
    println!("-----------------------------------------------------------------");
    println!(" ⚡ Real Hashrate    : {:.2} H/s", state.hashrate);
    println!(" 📡 Network Latency  : {} ms", state.latency_ms);
    println!(" 📊 Shares Verified  : {} / {}", state.accepted_shares, state.total_shares);
    println!("=================================================================");
    println!(" [DAEMON TELEMETRY LOGS]");
    for log in &state.event_log {
        println!("  👉 {}", log);
    }
    println!("=================================================================");
}

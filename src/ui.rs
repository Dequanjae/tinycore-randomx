// src/ui.rs
use crate::state::DashboardState;

pub fn render(state: &DashboardState) {
    print!("\x1B[H"); 
    println!("=================================================================");
    println!(" 👑  TinyCore Linux Monero Dashboard Engine   [LIVE DATA]");
    println!("=================================================================");
    println!(" 💻 Worker ID     : {}", state.worker_id);
    println!(" 🌐 Connected Pool: {}", state.pool_url);
    println!(" ⚡ Real Hashrate : {:.2} H/s | Latency: {} ms", state.hashrate, state.latency_ms);
    println!(" 📊 Shares Passed : {}", state.accepted_shares);
    println!("-----------------------------------------------------------------");
    println!(" [ENGINE LOGS]");
    for log in &state.event_log {
        println!("  👉 {}", log);
    }
    println!("=================================================================");
}

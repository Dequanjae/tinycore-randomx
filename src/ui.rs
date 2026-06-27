// src/ui.rs
use crate::state::DashboardState;

pub fn render(state: &DashboardState) {
    print!("\x1B[H\x1B[J"); // Clean screen refresh
    println!("=================================================================");
    println!(" 👑  TINY CORE LINUX LIVE PRODUCER MONITOR                      ");
    println!("=================================================================");
    println!(" 💻 Worker Target ID  : {}", state.worker_id);
    println!(" 🌐 Connected Pool    : {}", state.pool);
    println!(" ⏱️  System Uptime     : {} seconds", state.uptime);
    println!("-----------------------------------------------------------------");
    println!(" ⚡ Real Hashrate     : {:.2} H/s", state.hashrate);
    println!(" 📡 Network Latency   : {} ms", state.ping);
    println!(" 📊 Shares Verified   : {}", state.shares_verified);
    println!("=================================================================");
    println!(" [DAEMON TELEMETRY LOGS] ");
    println!(" Status: {}", state.status);
    println!("-----------------------------------------------------------------");
    for log in &state.event_log {
        println!("  {}", log);
    }
    println!("=================================================================");
}

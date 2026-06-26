// src/ui.rs
use crate::state::DashboardState;

pub fn render(state: &DashboardState) {
    // Jump cursor to top-left to redraw cleanly without flickering
    print!("\x1B[H"); 
    println!("==================================================");
    println!(" NODE MONITORING DASHBOARD | Worker: {}", state.worker_id);
    println!("==================================================");
    println!(" Threads Active: {} | Total Computations: {}", state.active_threads, state.total_cycles);
    println!("--------------------------------------------------");
    println!(" Recent Events Log:");
    for log in &state.event_log {
        println!("  👉 {}", log);
    }
    println!("==================================================");
}

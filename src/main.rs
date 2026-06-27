// src/main.rs
mod state;
mod ui;

use std::thread;
use std::time::Duration;
use state::DashboardState;

fn main() {
    // Clear terminal layout area completely
    println!("\x1B[2J\x1B[H");
    
    let mut state = DashboardState::new("tcl_node_01");

    loop {
        // Feed the true telemetry data into state variables
        state.poll_backend();
        
        // Draw console display output
        ui::render(&state);
        
        // Refresh timer
        thread::sleep(Duration::from_secs(2));
    }
}

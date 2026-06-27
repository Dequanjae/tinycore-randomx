// src/main.rs
mod state;
mod ui;

use std::thread;
use std::time::Duration;
use state::DashboardState;

fn main() {
    // Clear screen
    println!("\x1B[2J\x1B[H");
    
    let mut state = DashboardState::new("tcl_node_01");

    loop {
        // Poll the true API backend data
        state.poll_backend();
        
        // Render via your clean layout module
        ui::render(&state);
        
        // Polling interval
        thread::sleep(Duration::from_secs(2));
    }
}

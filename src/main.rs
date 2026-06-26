// src/main.rs
mod state;
mod ui;

use state::DashboardState;
use std::thread;
use std::time::Duration;

fn main() {
    let mut app_state = DashboardState::new("tcl_node_01");
    
    // Clear the screen once completely at application start
    println!("\x1B[2J\x1B[H");

    loop {
        // 1. Process data calculations or simulated event ticks
        app_state.increment_cycles(120);
        
        // 2. Refresh the display layout with the new data states
        ui::render(&app_state);
        
        // 3. Maintain steady execution intervals
        thread::sleep(Duration::from_millis(500));
    }
}

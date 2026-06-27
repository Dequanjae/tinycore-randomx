// src/main.rs
mod state;
mod ui;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use state::DashboardState;

// Define your permanent wallet address here so you never have to type it out manually again
const MY_PERSONAL_WALLET: &str = "8AaKnpAEK8MgHR5hJM2rLjhRaPuLYvtrb98uipf7gxVxh1uzsFJcrJ8SfXoWsxSTUE7ZdeGzRRQV9gUHFVzunKp5RNzg1fc";

fn prompt_input(prompt: &str, default: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() { default.to_string() } else { trimmed }
}

fn main() {
    println!("\x1B[2J\x1B[H"); // Clear layout terminal view
    println!("=================================================================");
    println!(" 👑  TINY CORE DEPLOYMENT SETTINGS PROFILE                      ");
    println!("=================================================================");

    // 1. Support/Automation Selection
    let support_choice = prompt_input(" 👉 Use developer default wallet address? [Y/n]: ", "Y");
    
    let target_wallet = if support_choice.to_uppercase() == "Y" {
        println!(" [+] Using hardcoded developer wallet profile.");
        MY_PERSONAL_WALLET.to_string()
    } else {
        prompt_input(" 👉 Enter your custom Monero Wallet Address: ", MY_PERSONAL_WALLET)
    };

    // 2. Standard Miner Variables
    let worker = prompt_input(" 👉 Enter Worker Node ID [default: tcl_node_01]: ", "tcl_node_01");
    let pool = prompt_input(" 👉 Enter Pool Connection URL [default: pool.supportxmr.com:443]: ", "pool.supportxmr.com:443");

    // 3. Write updates down to XMRig's config file dynamically
    println!("\x1B[2J\x1B[H");
    println!("[+] Provisioning runtime engine configuration parameters...");
    
    // Create the updated configuration file structure dynamically
    let config_json = format!(
        r#"{{
    "api": {{ "id": null, "worker-id": null }},
    "http": {{ "enabled": true, "host": "127.0.0.1", "port": 2222, "access-token": null, "restricted": false }},
    "autosave": true,
    "background": true,
    "pools": [
        {{
            "algo": "rx/0",
            "coin": "monero",
            "url": "{}",
            "user": "{}",
            "pass": "{}",
            "tls": true
        }}
    ]
}}"#,
        pool, target_wallet, worker
    );

    std::fs::write("config.json", config_json).expect("Fatal Error: Failed to write system config mapping layout.");

    // 4. Fire up background engine safely
    let _ = std::process::Command::new("sudo")
        .args(["killall", "xmrig"])
        .output();
        
    let _ = std::process::Command::new("sudo")
        .args(["./xmrig"])
        .spawn()
        .expect("Fatal Error: Background system component failed to initialize.");

    // 5. Spin up real-time telemetry rendering loop
    let mut state = DashboardState::new(&worker);
    loop {
        state.poll_backend();
        ui::render(&state);
        thread::sleep(Duration::from_secs(2));
    }
}

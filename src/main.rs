// src/main.rs
mod state;
mod ui;

use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use state::DashboardState;

// Define your permanent wallet address here
const MY_PERSONAL_WALLET: &str = "8ApdEka2j6CUaaNKp12H1VBi1bziZB2T9Dhju1fPzgiTC8KBLWEEddVeZnpZjg7Ni4KCENsPLfSDfh2nbMhbFqngM5wKwHE";

fn prompt_input(prompt: &str, default: &str) -> String {
    // FIXED: Added "== 1" to cleanly turn the C integer return into a Rust boolean
    if unsafe { libc::isatty(7) } != 1 && unsafe { libc::isatty(0) } != 1 {
        return default.to_string();
    }
    
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return default.to_string();
    }
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() { default.to_string() } else { trimmed }
}

// Custom interactive menu using arrow keys
fn prompt_pool_menu() -> String {
    let pools = vec![
        "pool.supportxmr.com:443",
        "xmr.2miners.com:2222",
        "xmr-us-east1.nanopool.org:14444",
        "mine.p2pool.observer:3333",
        "Custom (Enter your own)...",
    ];
    
    // FIXED: Added "== 1" comparison for type safety matching
    if unsafe { libc::isatty(0) } != 1 {
        return pools[0].to_string();
    }
    
    let mut selected_index = 0;
    
    loop {
        print!("\x1B[H\x1B[J"); // Clear screen and move to top
        println!("=================================================================");
        println!(" 🌐  SELECT YOUR MINING POOL CONNECTION                          ");
        println!("    (Use Up/Down Arrow Keys to navigate, press Enter to select)  ");
        println!("=================================================================");
        
        for (i, pool) in pools.iter().enumerate() {
            if i == selected_index {
                println!("  👉 \x1B[1;36m[ * ] {}\x1B[0m", pool); // Highlighting selected item in Cyan
            } else {
                println!("     [   ] {}", pool);
            }
        }
        println!("=================================================================");
        io::stdout().flush().unwrap();

        // Save current terminal state configuration safely
        let output = std::process::Command::new("stty")
            .arg("-g")
            .output();
            
        let old_stty = match output {
            Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
            Err(_) => return pools[0].to_string(),
        };

        // Put terminal in raw mode via system shell
        if std::process::Command::new("stty").arg("raw").arg("-echo").status().is_err() {
            return pools[0].to_string();
        }
        
        let mut key_buf = [0; 1];
        let read_result = io::stdin().read_exact(&mut key_buf);

        let mut final_key = if read_result.is_ok() { key_buf[0] } else { b'\n' };
        
        if final_key == 27 { // Escape sequence detected (Arrow keys)
            let mut seq = [0; 2];
            if io::stdin().read_exact(&mut seq).is_ok() && seq[0] == b'[' {
                if seq[1] == b'A' { final_key = 65; } // Up Arrow
                if seq[1] == b'B' { final_key = 66; } // Down Arrow
            }
        }

        // Restore terminal to old stable mode
        let _ = std::process::Command::new("stty").arg(&old_stty).status();

        // Handle the keys
        match final_key {
            65 => { // Up Arrow
                if selected_index > 0 { selected_index -= 1; }
            }
            66 => { // Down Arrow
                if selected_index < pools.len() - 1 { selected_index += 1; }
            }
            13 | 10 => { // Enter Key
                if selected_index == pools.len() - 1 {
                    print!("\x1B[H\x1B[J");
                    let mut custom_pool = String::new();
                    while custom_pool.is_empty() {
                        custom_pool = prompt_input(" 👉 Enter your custom Pool Connection URL: ", "");
                    }
                    return custom_pool;
                } else {
                    return pools[selected_index].to_string();
                }
            }
            _ => {}
        }
    }
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
        let mut custom_wallet = String::new();
        while custom_wallet.is_empty() {
            custom_wallet = prompt_input(" 👉 Enter your custom Monero Wallet Address: ", "");
            if custom_wallet.is_empty() {
                println!(" ❌ Error: Wallet address cannot be empty when opting out of developer support.");
            }
        }
        custom_wallet
    };

    // 2. Standard Miner Variables
    let worker = prompt_input(" 👉 Enter Worker Node ID [default: tcl_node_01]: ", "tcl_node_01");
    
    // Interactive Arrow Key Menu for Pool selection
    let pool = prompt_pool_menu();

    // 3. Write updates down to XMRig's config file dynamically
    println!("\x1B[2J\x1B[H");
    println!("[+] Provisioning runtime engine configuration parameters...");
    
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
        
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        
    let _ = std::process::Command::new("sudo")
        .current_dir(current_dir)
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

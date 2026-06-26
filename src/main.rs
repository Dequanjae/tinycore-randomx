use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

// Helper to simulate system stats since Tiny Core is minimal
fn get_mock_performance() -> (f32, u32, f32) {
    // Generates slightly shifting values so the dashboard looks alive
    let rand_factor = (Instant::now().elapsed().as_millis() % 100) as f32 / 100.0;
    let cpu = 75.0 + (rand_factor * 20.0); // 75% - 95%
    let hashrate = 450 + ((rand_factor * 100.0) as u32); // 450 - 550 H/s
    let ram = 42.1 + (rand_factor * 2.5); // RAM %
    (cpu, hashrate, ram)
}

fn main() {
    let start_time = Instant::now();
    let wallet_address = "44AFFq5kSiGbU8S789Cabc1234567890QWERTYUIOPASDFGHJKLZXCVBNM1234567890"; // Put your actual wallet address here
    let device_name = "Tiny Core Linux Device";
    
    let mut total_mined = 0.00000000;
    let mut loop_counter = 0;

    println!("\x1B[2J\x1B[H"); // Clear screen once at start

    loop {
        loop_counter += 1;
        let elapsed = start_time.elapsed();
        let hours = elapsed.as_secs() / 3600;
        let minutes = (elapsed.as_secs() % 3600) / 60;
        let seconds = elapsed.as_secs() % 60;

        let (cpu, hashrate, ram) = get_mock_performance();
        
        // Simulating the mining progress bar matching the loop
        let progress_ticks = loop_counter % 11;
        let mut progress_bar = String::new();
        for i in 0..10 {
            if i < progress_ticks { progress_bar.push('█'); } else { progress_bar.push('░'); }
        }

        // Simulate mining rewards tracking
        let mut attempt_status = String::new();
        // Every 5 seconds, simulate finding a successful share
        if loop_counter % 5 == 0 {
            let reward = 0.00001245;
            total_mined += reward;
            attempt_status = format!("\x1B[32m[✓] Correct! Received \"{:.8}\" XMR\x1B[0m", reward);
        } else {
            attempt_status = format!("\x1B[31m[✗] Try again. Received \"0\" XMR\x1B[0m");
        }

        // Generate a fake hash string to show it's trying combinations
        let simulated_hash = format!("{:x}", Instant::now().elapsed().as_nanos());
        let truncated_hash = if simulated_hash.len() > 16 { &simulated_hash[0..16] } else { &simulated_hash };

        // --- DRAW INTERFACE USING ANSI ESCAPE CODES ---
        print!("\x1B[H"); // Move cursor to top left instead of clearing (prevents flickering!)
        
        println!("=================================================================================");
        println!(" 👑  \x1B[1;36mTinyCore Linux XMR Monero Dashboard\x1B[0m              [\x1B[32m● ONLINE\x1B[0m]");
        println!("=================================================================================");
        println!(" 👛 Wallet: \x1B[33m{}\x1B[0m [\x1B[32mCONNECTED\x1B[0m]", wallet_address);
        println!(" 💻 Device: \x1B[35m{}\x1B[0m", device_name);
        println!("---------------------------------------------------------------------------------");
        println!(" ⏱️  Session Time: {:02}:{:02}:{:02} | ⏳ Progress: [{}]", hours, minutes, seconds, progress_bar);
        println!("---------------------------------------------------------------------------------");
        println!(" [SYSTEM PERFORMANCE]");
        println!("    🔥 CPU Usage: {:.1}%  |  🧠 RAM Usage: {:.1}%", cpu, ram);
        println!("    ⚡ Current Hashrate: \x1B[1;32m{} H/s\x1B[0m", hashrate);
        println!("---------------------------------------------------------------------------------");
        println!(" [LIVE NETWORK MINING ENGINE LOGS]");
        println!("    Testing Target Combo: 0x{}...", truncated_hash);
        println!("    Result: {}", attempt_status);
        println!("---------------------------------------------------------------------------------");
        println!(" 💰 Total Monero Mined This Session: \x1B[1;32m{:.8} XMR\x1B[0m", total_mined);
        let est_value_usd = total_mined * 160.0; // Mock price multiplier (e.g. $160/XMR)
        println!(" 📈 Estimated Session Value:         \x1B[1;34m${:.4} USD\x1B[0m", est_value_usd);
        println!("=================================================================================");

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1)); // Refresh screen every 1 second
    }
}

use std::io::{self, Write};
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use chrono::Local;
use serde::{Serialize, Deserialize};

const CONFIG_FILE: &str = "config.toml";

// 1. CONFIGURATION DATA STRUCT
#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    wallet_address: String,
    worker_id: String,
    pool_url: String,
}

// 2. CENTRALIZED BACKEND STATE
struct DashboardState {
    config: AppConfig,
    device_name: String,
    difficulty: String,
    current_block: u64,
    threads_active: u32,
    
    hashrate: u32,
    cpu_usage: f32,
    ram_usage: f32,
    latency_ms: u32,
    
    accepted_shares: u32,
    rejected_shares: u32,
    total_xmr: f64,
    progress_percent: u32,
    
    current_nonce: String,
    current_hash: String,
    reward_log: Vec<String>,
    spinner_frame: usize,
}

impl DashboardState {
    fn new(config: AppConfig) -> Self {
        Self {
            config,
            device_name: "Tiny Core Linux Device".to_string(),
            difficulty: "120.5G".to_string(),
            current_block: 3124592,
            threads_active: 4,
            hashrate: 500,
            cpu_usage: 85.0,
            ram_usage: 42.0,
            latency_ms: 45,
            accepted_shares: 0,
            rejected_shares: 0,
            total_xmr: 0.0,
            progress_percent: 0,
            current_nonce: "00000000".to_string(),
            current_hash: "0000000000000000".to_string(),
            reward_log: vec![
                "\x1B[33m[~] Configuration profile verified. Connecting to pool... \x1B[0m".to_string(); 7
            ],
            spinner_frame: 0,
        }
    }

    fn update_state(&mut self) {
        let mut rng = rand::thread_rng();
        let timestamp = Local::now().format("%H:%M:%S").to_string();

        self.spinner_frame = (self.spinner_frame + 1) % 8;
        self.progress_percent += rng.gen_range(5..15);
        
        let mut round_complete = false;
        if self.progress_percent >= 100 {
            self.progress_percent = 0;
            round_complete = true;
            self.current_block += rng.gen_range(0..2) % 2;
        }

        self.hashrate = rng.gen_range(480..525);
        self.cpu_usage = rng.gen_range(78.5..96.2);
        self.ram_usage = rng.gen_range(41.2..43.8);
        self.latency_ms = rng.gen_range(38..52);

        self.current_hash = format!("{:016x}", rng.gen::<u64>());
        self.current_nonce = format!("{:08x}", rng.gen::<u32>());

        if round_complete {
            let log_entry = if rng.gen_ratio(1, 8) {
                let reward = rng.gen_range(0.00005000..0.00080000);
                self.total_xmr += reward;
                self.accepted_shares += 1;
                format!("[\x1B[32m{}\x1B[0m] \x1B[32mFOUND\x1B[0m              {:.8} XMR", timestamp, reward)
            } else if rng.gen_ratio(1, 50) {
                self.rejected_shares += 1;
                format!("[\x1B[32m{}\x1B[0m] \x1B[31mREJECTED\x1B[0m           0.00000000 XMR", timestamp)
            } else {
                format!("[\x1B[32m{}\x1B[0m] \x1B[38;5;208mTRY AGAIN\x1B[0m          0.00000000 XMR", timestamp)
            };

            self.reward_log.push(log_entry);
            if self.reward_log.len() > 7 {
                self.reward_log.remove(0);
            }
        }
    }
}

// 3. UI RENDERING FRAMEWORK
fn draw_dashboard(state: &DashboardState, uptime_secs: u64) {
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let seconds = uptime_secs % 60;

    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧'];
    let current_spinner = spinner_chars[state.spinner_frame];

    let filled_blocks = (state.progress_percent / 4) as usize;
    let mut progress_bar = String::new();
    for i in 0..25 {
        if i < filled_blocks { progress_bar.push('█'); } else { progress_bar.push('░'); }
    }

    print!("\x1B[H"); // Cursor resetting code to prevent screen strobing
    println!("=================================================================================");
    println!(" 👑  \x1B[1;36mTinyCore Linux XMR Monero Dashboard\x1B[0m             [\x1B[32m● ONLINE\x1B[0m]");
    println!("=================================================================================");
    println!(" 👛 Wallet Address: \x1B[33m{}\x1B[0m [\x1B[32mCONNECTED\x1B[0m]", state.config.wallet_address);
    println!(" 💻 Target Node   : \x1B[35m{}\x1B[0m | ID: {}", state.device_name, state.config.worker_id);
    println!(" 🌐 Pool Server   : \x1B[34m{}\x1B[0m | Latency: {}ms", state.config.pool_url, state.latency_ms);
    println!("---------------------------------------------------------------------------------");
    println!(" ⏱️  Uptime: {:02}:{:02}:{:02} | {} Mining Round: [{}] {:3}%", 
             hours, minutes, seconds, current_spinner, progress_bar, state.progress_percent);
    println!("---------------------------------------------------------------------------------");
    println!(" [SYSTEM PERFORMANCE]                            [POOL & METRIC COUNTERS]");
    println!("    🔥 CPU Allocation:  {:.1}%                    📊 Block Height : {}", state.cpu_usage, state.current_block);
    println!("    🧠 RAM Utilization: {:.1}%                    ⚙️  Network Diff : {}", state.ram_usage, state.difficulty);
    println!("    ⚡ Current Hashrate: \x1B[1;32m{} H/s\x1B[0m                🧵 Active Threads: {}", state.hashrate, state.threads_active);
    println!("                                                 🟢 Accepted     : \x1B[32m{}\x1B[0m", state.accepted_shares);
    println!("    Dataset: RandomX (Light Mode)                🔴 Rejected     : \x1B[31m{}\x1B[0m", state.rejected_shares);
    println!("---------------------------------------------------------------------------------");
    println!(" [LIVE ENGINE LOGS]");
    println!("    Current Nonce: 0x{} | Target Pipeline Check: 0x{}...", state.current_nonce, state.current_hash);
    println!("---------------------------------------------------------------------------------");
    println!("========================================= LIVE REWARD LOG ======================");
    for log in &state.reward_log {
        println!("  {}", log);
    }
    println!("=================================================================================");
    println!(" 💰 Total Monero Mined This Session: \x1B[1;32m{:.8} XMR\x1B[0m", state.total_xmr);
    println!(" 📈 Estimated Session Value:         \x1B[1;34m${:.4} USD\x1B[0m", state.total_xmr * 155.40);
    println!("=================================================================================");
    
    io::stdout().flush().unwrap();
}

// 4. INTERACTIVE INITIALIZATION AND UTILITIES
fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn load_or_create_config() -> AppConfig {
    if Path::new(CONFIG_FILE).exists() {
        // Read existing config
        let contents = std::fs::read_to_string(CONFIG_FILE).expect("Failed to read configuration profile.");
        toml::from_str(&contents).expect("Invalid configuration parsing style.")
    } else {
        // Display setup banner
        println!("==================================================");
        println!(" 🛠️  INITIAL TINY CORE NODE SETUP PROFILE");
        println!("==================================================");
        println!(" No existing 'config.toml' profile found.");
        println!(" Please supply your node environment information below:\n");

        let mut wallet = prompt_input(" 👉 Enter Monero Receive Wallet Address: ");
        if wallet.is_empty() {
            wallet = "44AFFq5kSiGbU8S789Cabc1234567890QWERTYUIOPASDFGHJKLZXCVBNM1234567890".to_string();
        }
        
        let mut worker = prompt_input(" 👉 Enter Worker/Device Node Identifier: ");
        if worker.is_empty() {
            worker = "tcl_node_01".to_string();
        }

        let mut pool = prompt_input(" 👉 Enter Pool Connection Endpoint URL: ");
        if pool.is_empty() {
            pool = "pool.supportxmr.com:443".to_string();
        }

        let new_config = AppConfig {
            wallet_address: wallet,
            worker_id: worker,
            pool_url: pool,
        };

        // Serialize into TOML formatting string
        let toml_string = toml::to_string_pretty(&new_config).unwrap();
        let mut file = File::create(CONFIG_FILE).expect("Failed to construct profile path.");
        file.write_all(toml_string.as_bytes()).expect("Failed to write to destination profile.");
        
        println!("\n[✓] Configuration successfully saved to './config.toml'!");
        println!(" Press Enter to jump to the running node interface...");
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();

        new_config
    }
}

fn main() {
    let config = load_or_create_config();
    let start_time = Instant::now();
    let mut state = DashboardState::new(config);

    println!("\x1B[2J\x1B[H"); // Wipe screen clean before showing the loop

    loop {
        state.update_state();
        draw_dashboard(&state, start_time.elapsed().as_secs());
        thread::sleep(Duration::from_millis(250));
    }
}

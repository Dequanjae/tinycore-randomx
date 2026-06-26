#!/bin/sh
# Fail immediately if any command flags an error
set -e

echo "=== TINY CORE LINUX ENVIRONMENT DEPLOYMENT ==="

# 1. Grab Core build utilities via Tiny Core Extension System
echo "[+] Syncing core dependencies from repository mirrors..."
tce-load -wi wget squashfs-tools make gcc glibc_apps bash

# 2. Check for Rust and handle portable local extraction if missing
if command -v rustc >/dev/null 2>&1 || [ -f "$HOME/rust-local/rustc/bin/rustc" ]; then
    echo "[✓] Rust compiler framework detected."
    # Make sure local paths are loaded just in case
    if [ -d "$HOME/rust-local" ]; then
        export PATH="$HOME/rust-local/rustc/bin:$HOME/rust-local/cargo/bin:$PATH"
    fi
else
    echo "[-] Rust compiler was not found."
    printf "👉 Rust is required to build the dashboard. Install it now? (y/n): "
    read -r user_choice
    
    if [ "$user_choice" = "y" ] || [ "$user_choice" = "Y" ]; then
        echo "[+] Downloading standalone Rust package..."
        wget https://static.rust-lang.org/dist/rust-1.78.0-x86_64-unknown-linux-gnu.tar.gz
        
        echo "[+] Extracting Rust compiler locally..."
        tar -xf rust-1.78.0-x86_64-unknown-linux-gnu.tar.gz
        
        # Move the compiled binaries straight into your home folder to protect them from system wipes
        mv rust-1.78.0-x86_64-unknown-linux-gnu "$HOME/rust-local"
        
        # Manually inject the portable binaries directly into our active script session path
        export PATH="$HOME/rust-local/rustc/bin:$HOME/rust-local/cargo/bin:$PATH"
        
        # Clean up the compressed download file to instantly free up RAM
        rm -f rust-1.78.0-x86_64-unknown-linux-gnu.tar.gz
    else
        echo "❌ Installation cancelled by user. Rust is needed to finish the build."
        exit 1
    fi
fi

# Ensure the shell path includes our portable binaries for the remainder of this script run
if [ -d "$HOME/rust-local" ]; then
    export PATH="$HOME/rust-local/rustc/bin:$HOME/rust-local/cargo/bin:$PATH"
fi

# 3. Retrieve pre-built XMRig engine tailored for raw Linux architectures
if [ ! -f "./xmrig" ]; then
    echo "[+] Downloading optimized RandomX engine component..."
    wget https://github.com/xmrig/xmrig/releases/download/v6.21.0/xmrig-6.21.0-linux-static-x64.tar.gz
    tar -xf xmrig-6.21.0-linux-static-x64.tar.gz
    mv xmrig-6.21.0/xmrig .
    rm -rf xmrig-6.21.0*
fi

# 4. Compile the custom Rust monitor framework
echo "[+] Generating optimized local interface profile..."
if [ ! -f "./Cargo.toml" ]; then
    # Create simple build configuration inline if missing
    echo '[package]
name = "miner_dashboard"
version = "0.1.0"
edition = "2021"' > Cargo.toml
    mkdir -p src
    mv main.rs src/main.rs 2>/dev/null || true
fi

# Build your custom dashboard binary using our localized path
echo "[+] Invoking compiler toolchain..."
cargo build --release
cp target/release/miner_dashboard ./monitor

# Optional cleanup: Wipes out the large local compiler after compilation to free up your RAM!
echo "[+] Cleaning up local builder environment..."
rm -rf "$HOME/rust-local"

echo "=================================================="
echo " SETUP COMPLETED SUCCESSFULLY"
echo " Run dashboard using: ./monitor"
echo "=================================================="

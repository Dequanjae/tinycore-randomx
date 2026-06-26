#!/bin/sh
# Fail immediately if any command flags an error
set -e

echo "=== TINY CORE LINUX ENVIRONMENT DEPLOYMENT ==="

# 1. Grab Core build utilities via Tiny Core Extension System
echo "[+] Syncing core dependencies from repository mirrors..."
tce-load -wi wget squashfs-tools make gcc glibc_apps

# 2. Grab Rust bootstrap binary tooling directly
if ! command -v rustc >/dev/null 2>&1; then
    echo "[+] Bootstrapping Rust compiler toolchain safely..."
    export TMPDIR=$HOME
    wget -O- https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
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

cargo build --release
cp target/release/miner_dashboard ./monitor

echo "=================================================="
echo " SETUP COMPLETED SUCCESSFULLY"
echo " Run dashboard using: ./monitor"
echo "=================================================="

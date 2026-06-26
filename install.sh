#!/bin/sh
# Fail immediately if any command flags an error
set -e

echo "=== TINY CORE LINUX ENVIRONMENT DEPLOYMENT ==="

# 1. Grab Core utilities via Tiny Core Extension System
echo "[+] Syncing core dependencies from repository mirrors..."
tce-load -wi wget

# 2. Retrieve pre-built XMRig engine tailored for raw Linux architectures
if [ ! -f "./xmrig" ]; then
    echo "[+] Downloading optimized RandomX engine component..."
    wget https://github.com/xmrig/xmrig/releases/download/v6.21.0/xmrig-6.21.0-linux-static-x64.tar.gz
    tar -xf xmrig-6.21.0-linux-static-x64.tar.gz
    mv xmrig-6.21.0/xmrig .
    rm -rf xmrig-6.21.0*
fi

# 3. Download the pre-compiled custom monitor binary built by GitHub Actions
echo "[+] Downloading pre-compiled machine-code interface profile..."
wget -O monitor https://raw.githubusercontent.com/Dequanjae/tinycore-randomx/main/monitor
chmod +x monitor

echo "=================================================="
echo " SETUP COMPLETED SUCCESSFULLY"
echo " Run dashboard using: ./monitor"
echo "=================================================="

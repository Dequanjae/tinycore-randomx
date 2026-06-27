#!/bin/sh
# Fail immediately if any command flags an error
set -e

echo "=========================================================="
echo " ⚡ DEPLOYING HIGH-PERFORMANCE TINY CORE LINUX ENVIRONMENT "
echo "=========================================================="

# 1. Establish absolute path inside the persistent user space
WORK_DIR="/home/tc/tinycore-randomx"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

# 2. Grab Core utilities via Tiny Core Extension System
echo "[+] Syncing core dependencies from repository mirrors..."
tce-load -wi wget

# 3. Retrieve pre-built XMRig engine tailored for Linux architectures
if [ ! -f "./xmrig" ]; then
    echo "[+] Downloading optimized RandomX engine component..."
    wget https://github.com/xmrig/xmrig/releases/download/v6.21.0/xmrig-6.21.0-linux-static-x64.tar.gz
    tar -xf xmrig-6.21.0-linux-static-x64.tar.gz
    mv xmrig-6.21.0/xmrig .
    rm -rf xmrig-6.21.0*
fi

# 4. Fetch the precompiled user interface binary from your GitHub repository
echo "[+] Fetching operational interface monitor..."
wget -O monitor https://raw.githubusercontent.com/Dequanjae/tinycore-randomx/main/monitor
chmod +x monitor

# 5. Configure Linux kernel memory optimization parameters (Huge Pages)
echo "[+] Tuning kernel parameters: Allocating Huge Pages..."
sudo sysctl -w vm.nr_hugepages=1280

# 6. Coordinate Process Safety: Terminate older duplicate processes
echo "[+] Clearing out conflicting process records..."
sudo killall xmrig 2>/dev/null || true

# 7. Register working directory with Tiny Core's backup configuration registry
echo "[+] Registering binaries to Tiny Core backup system..."
TARGET_LINE="home/tc/tinycore-randomx"
if ! grep -Fxq "$TARGET_LINE" /opt/.filetool.lst; then
    echo "$TARGET_LINE" >> /opt/.filetool.lst
fi

# Force system state sync to persistence medium
echo "[+] Backing up current system layer to storage device..."
filetool.sh -b

echo "=========================================================="
echo "         CONFIGURATION & DEPLOYMENT COMPLETE              "
echo "=========================================================="
echo " To configure your wallet & start mining, run:"
echo " ./monitor"
echo "=========================================================="

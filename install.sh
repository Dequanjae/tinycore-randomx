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

# 4. Create the performance-optimized configuration profile with the HTTP API active
echo "[+] Constructing local backend config profile..."
cat << 'EOF' > config.json
{
    "api": {
        "id": null,
        "worker-id": null
    },
    "http": {
        "enabled": true,
        "host": "127.0.0.1",
        "port": 2222,
        "access-token": null,
        "restricted": false
    },
    "autosave": true,
    "background": true,
    "pools": [
        {
            "algo": "rx/0",
            "coin": "monero",
            "url": "pool.supportxmr.com:443",
            "user": "44AFFq5kSiGbU8S789Cabc1234567890QWERTYUIOPASDFGHJKLZXCVBNM1234567890",
            "pass": "tcl_node_01",
            "tls": true
        }
    ]
}
EOF

# 5. Fetch the precompiled user interface binary
echo "[+] Fetching operational interface monitor..."
wget -O monitor https://raw.githubusercontent.com/Dequanjae/tinycore-randomx/main/monitor
chmod +x monitor

# 6. Configure Linux kernel memory optimization parameters (Huge Pages)
echo "[+] Tuning kernel parameters: Allocating Huge Pages..."
sudo sysctl -w vm.nr_hugepages=1280

# 7. Coordinate Process Safety: Terminate older duplicate processes
echo "[+] Clearing out conflicting process records..."
sudo killall xmrig 2>/dev/null || true

# 8. Register working directory with Tiny Core's backup configuration registry
echo "[+] Registering binaries to Tiny Core backup system..."
TARGET_LINE="home/tc/tinycore-randomx"
if ! grep -Fxq "$TARGET_LINE" /opt/.filetool.lst; then
    echo "$TARGET_LINE" >> /opt/.filetool.lst
fi

# Force system state sync to persistence medium
echo "[+] Backing up current system layer to storage device..."
filetool.sh -b

echo "=========================================================="
echo " CONFIGURATION COMPLETE - BACKEND LAUNCHED SUCCESSFULLY"
echo "=========================================================="
echo " To boot the engine manually:   sudo ./xmrig"
echo " To run the display interface:  ./monitor"
echo "=========================================================="

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

# 3. Create the configuration profile enabling the HTTP API daemon
echo "[+] Creating local API engine configuration layer..."
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

# 4. Download the pre-compiled custom monitor binary built by GitHub Actions
echo "[+] Downloading pre-compiled machine-code interface profile..."
wget -O monitor https://raw.githubusercontent.com/Dequanjae/tinycore-randomx/main/monitor
chmod +x monitor

echo "=================================================="
echo " SETUP COMPLETED SUCCESSFULLY"
echo " Start backend miner:  sudo ./xmrig"
echo " Run visual dashboard: ./monitor"
echo "=================================================="

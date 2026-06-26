# Tiny Core Linux XMR Monero Dashboard 🚀

A highly optimized, lightweight terminal dashboard built in Rust for monitoring RandomX engines on Tiny Core Linux environments. 

Instead of dealing with massive compiler setups or memory limitations on raw RAM-based operating systems, this repository utilizes **GitHub Actions** to automatically compile the Rust monitoring tool into a standalone, statically linked machine-code binary (`monitor`). Your Tiny Core device simply downloads the pre-compiled file and runs it instantly!

---

## 🖥️ Dashboard Visual Overview

The custom terminal interface features:
* **Live Network Status:** Displays an `[● ONLINE]` indicator and connection state verification.
* **Dynamic Statistics:** Shifting CPU usage, RAM allocations, and live hash speeds.
* **Mining Simulation Feedback:** Displays detailed progress blocks alongside successes (`[✓] Correct!`) and misses (`[✗] Try again`).
* **Financial Tracking:** Real-time metrics counting total XMR generated this session and its estimated USD value.

---

## ⚡ Quick Start (On Tiny Core Linux)

To deploy the environment on your Tiny Core system, clear any previous installation folders and execute the clean bootstrapping script:

```bash
rm -rf *
wget https://raw.githubusercontent.com/Dequanjae/tinycore-randomx/main/install.sh
chmod +x install.sh
./install.sh

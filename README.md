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

---

Once the deployment script completes successfully, launch your live monitoring interface by typing:

```bash
./monitor

---

## 🛠️ How It Works (Under the Hood)
Automated Pipeline (.github/workflows/build.yml): Every time you commit changes to src/main.rs, GitHub fires up an automated Ubuntu runner.

MUSL Compilation: GitHub compiles your Rust project targeting x86_64-unknown-linux-musl. This packages all necessary libraries directly inside the binary.

No-Dependency Deployment: Your Tiny Core machine pulls the finished binary file. Because it is statically built, it executes perfectly without needing cargo, rustc, or bash installed on the client machine.

---

## 🗃️ Repository Structure
src/main.rs — The primary Rust source file controlling the dashboard logic, escape codes, and terminal refresh rates.

**Cargo.toml** — The configuration file managing dependencies and language editions.

**install.sh** — The deployment shell script optimized to download dependencies and grab the pre-built binaries.

**.github/workflows/build.yml** — The GitHub Actions automation script that builds the binaries for you.

---

## ❓ Troubleshooting
**The terminal displays "Permission Denied" when launching** ./monitor
Ensure the file has executable permissions assigned to it by running:

```bash
chmod +x monitor

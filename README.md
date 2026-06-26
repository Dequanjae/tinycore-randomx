# Tiny Core Linux XMR Monero Dashboard 🚀

A lightweight terminal dashboard written in Rust for monitoring RandomX mining activity on Tiny Core Linux.

Instead of installing the full Rust toolchain on a RAM-based operating system, this repository uses **GitHub Actions** to automatically build a standalone, statically linked binary (`monitor`). Your Tiny Core system simply downloads the precompiled binary and runs it.

---

# 🖥️ Dashboard Features

- **Live Network Status** – Displays an `● ONLINE` indicator and connection status.
- **Dynamic Statistics** – Shows CPU usage, memory usage, and current hash rate.
- **Mining Status** – Displays mining progress along with accepted and rejected shares (or other mining events, depending on your implementation).
- **Session Statistics** – Displays runtime statistics such as total hashes, uptime, and any additional metrics collected by the dashboard.

---

# ⚡ Quick Start (Tiny Core Linux)

Remove any previous installation files, then download and run the installer:

```bash
rm -rf *
wget https://raw.githubusercontent.com/Dequanjae/tinycore-randomx/main/install.sh
chmod +x install.sh
./install.sh
```

After installation completes, start the dashboard:

```bash
./monitor
```

---

# 🛠️ How It Works

### GitHub Actions

Whenever changes are pushed to the repository, GitHub Actions automatically builds the project.

### Static MUSL Build

The project is compiled for:

```
x86_64-unknown-linux-musl
```

This produces a fully statically linked executable that requires no Rust installation on the target system.

### Simple Deployment

The Tiny Core installer downloads the latest prebuilt binary, allowing the dashboard to run without installing:

- Rust
- Cargo
- Additional runtime libraries

---

# 🗂️ Repository Structure

```
.
├── src/
│   └── main.rs              # Dashboard source code
├── Cargo.toml               # Rust project configuration
├── install.sh               # Tiny Core installer
└── .github/
    └── workflows/
        └── build.yml        # GitHub Actions build workflow
```

---

# ❓ Troubleshooting

### Permission denied when launching `./monitor`

Make the binary executable:

```bash
chmod +x monitor
```

### Binary won't run

Verify that you're using a build compiled for your CPU architecture.

### Installer failed

Check your internet connection and verify that GitHub is reachable from your Tiny Core system.

---

# 🚀 Why This Project?

- Lightweight and fast
- Designed for Tiny Core Linux
- No Rust toolchain required on the target system
- Automatic GitHub Actions builds
- Statically linked executable for maximum portability
````

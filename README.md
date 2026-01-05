# Vortex Network Scanner

**Vortex** is a high-performance, asynchronous network port scanner written in Rust. It is designed to be lightweight, incredibly fast, and visually pleasing (Production Ready Edition).

## Features

- ⚡ **Blazing Fast**: Uses the `Tokio` async runtime to scan thousands of ports in seconds.
- 📦 **Lightweight**: Heavily optimized binary size (~600KB) with zero heavy external dependencies.
- 🎨 **Modern Design**: Pixel-perfect CLI output with manual table rendering for stability across terminals.
- 🔍 **Smart Detection**: Identifies services running on open ports automatically.
- 🛠 **Flexible**: Full control over concurrency, timeouts, and port ranges.

## Installation

### Build from Source
Requirements: [Rust Toolchain](https://rustup.rs/)

```sh
git clone https://github.com/ZoniBoy00/vortex.git
cd vortex
cargo build --release
```

The executable will be at `target/release/vortex.exe`.

## Usage

### 🚀 Quick Start
Scan a website or IP for common ports (default 1-1000).
```powershell
.\vortex.exe --target google.com
```

### 🎯 Specific Port Range with High Speed
Scan all ports (1-65535) with 5000 parallel threads.
```powershell
.\vortex.exe --target 192.168.1.5 --start-port 1 --end-port 65535 --concurrency 5000
```

### ⚙ Options
| Flag | Description | Default |
|------|-------------|---------|
| `-t, --target` | Target IP or Domain | **Required** |
| `-s, --start-port` | First port to scan | `1` |
| `-e, --end-port` | Last port to scan | `1000` |
| `-c, --concurrency` | Number of simultaneous tasks | `1000` |
| `--timeout` | Timeout per port (ms) | `1000` |

## Example Output

```text
  _    __            __       
 | |  / /___  ____  / /____  _  __
 | | / / __ \/ ___\/ __/ _ \| |/_/
 | |/ / /_/ / /   / /_/  __/>  <
 |___/\____/_/    \__/\___/_/|_|
                              
  VORTEX NETWORK SCANNER v0.1
  High-Performance Security Tool

  ┌── SCAN TARGET ───────────────────────────────┐
  │ Target:      google.com                      │
  │ Range:       1-65535                         │
  │ Strategy:    5000 threads, 1000ms timeout    │
  └──────────────────────────────────────────────┘

  [00:00:05] [########################################] 65535/65535 (0s)

  ┌──────────┬──────────┬─────────────────────────┐
  │ PORT     │ STATUS   │ SERVICE                 │
  ├──────────┼──────────┼─────────────────────────┤
  │ 80       │ OPEN     │ HTTP                    │
  │ 443      │ OPEN     │ HTTPS                   │
  └──────────┴──────────┴─────────────────────────┘

  Scan Complete.
  • 2 open ports found
  • Time elapsed: 5.42s
```

## License
MIT License

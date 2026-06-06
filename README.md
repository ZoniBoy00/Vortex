# Vortex Network Scanner ⚡

A high-performance, asynchronous network port scanner written in Rust. Built on Tokio for blazing-fast, non-blocking scans with a beautiful CLI output.

## Features

- ⚡ **Blazing Fast** — async I/O with Tokio for concurrent scanning (thousands of ports/second)
- 🎨 **Beautiful CLI** — pixel-perfect output with color-coded results
- 🔍 **Smart Detection** — identifies services on 10,000+ well-known ports
- 🏷️ **Banner Grabbing** — reads service banners to detect exact versions (`--verbose`)
- 🌐 **CIDR Support** — scan entire subnets (`vortex --target 192.168.1.0/24`)
- 📋 **Multiple Targets** — scan from a file (`vortex --targets targets.txt`)
- 🚀 **Top Ports Mode** — quick scan of the most common ports (`--top-ports 100`)
- 📤 **Export Results** — save results as JSON or CSV (`--output results.json`)
- ❤️ **Ping Check** — test if hosts are alive before scanning (`--ping`)
- 🎯 **Flexible** — configurable port range, timeout, and concurrency
- 📦 **Lightweight** — heavily optimized binary (~600KB) with zero heavy dependencies

## Installation

### Build from Source

```bash
# Requires Rust toolchain: https://rustup.rs/
git clone https://github.com/ZoniBoy00/vortex.git
cd vortex
cargo build --release
```

The binary will be at `target/release/vortex` (or `vortex.exe` on Windows).

### Download Pre-built

Download the latest binary from the [Releases](https://github.com/ZoniBoy00/vortex/releases) page.

## Usage

### Quick Start

```bash
# Scan common ports on a target
vortex --target google.com

# Scan all ports (1-65535) with high concurrency
vortex --target 192.168.1.1 --start-port 1 --end-port 65535 --concurrency 5000
```

### Advanced Examples

```bash
# Scan top 100 ports with verbose banner grabbing
vortex --target scanme.nmap.org --top-ports 100 --verbose

# Scan a subnet with ping check, results to JSON
vortex --target 192.168.1.0/24 --top-ports 50 --ping --output results.json

# Scan multiple targets from a file
echo "192.168.1.1" > targets.txt
echo "10.0.0.1" >> targets.txt
vortex --targets targets.txt --top-ports 100

# Full port range scan with custom timeout
vortex --target example.com --start-port 1 --end-port 65535 --timeout 2000 --concurrency 10000
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `--target` | Target IP, domain, or CIDR range | — |
| `--targets` | File containing targets (one per line) | — |
| `-s, --start-port` | Start port | 1 |
| `-e, --end-port` | End port | 1000 |
| `--top-ports N` | Scan only top N common ports | — |
| `--timeout` | Timeout per port (ms) | 1000 |
| `-c, --concurrency` | Concurrent scan tasks | 1000 |
| `-v, --verbose` | Enable banner grabbing | false |
| `--ping` | Ping host before scanning | false |
| `-o, --output` | Export path (.json or .csv) | — |

## Discord Embed Preview

Vortex is also used by [rust-monitor](https://github.com/ZoniBoy00/rust-monitor) for live system monitoring via Discord webhooks.

## License

MIT

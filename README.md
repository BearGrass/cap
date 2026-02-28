# cap

High-performance network packet capture and analysis tool written in Rust.

## Features

- **Real-time packet capture** from network interfaces
- **TUI visual interface** with real-time traffic display
- **5 major protocol support**: Ethernet, IP (IPv4/IPv6), TCP, UDP, HTTP, DNS
- **Protocol identification** based on port analysis
- **Multiple output formats**: Console (colored tables), JSON
- **BPF filter support** for targeted capture

## Quick Start

```bash
# Build and setup (one-time)
./setup.sh

# Then use cap command anywhere
cap tui -i enp89s0    # Real-time TUI interface
cap live -i eth0      # Real-time capture (CLI mode)
cap --help            # Show help
```

## Installation

### Option 1: Automated Setup (Recommended)

```bash
# Build and install to /usr/local/bin with proper permissions
./setup.sh
```

This script:
1. Builds the release binary
2. Sets network capture capabilities (no root needed)
3. Installs `cap` to `/usr/local/bin`

### Option 2: Manual Build

```bash
# Build from source
cargo build --release

# The binary will be at target/release/cap
# Note: Requires root privileges for packet capture
sudo ./target/release/cap live -i eth0
```

### Requirements

- Rust 1.70+
- libpcap development library: `sudo apt-get install libpcap-dev`

## Usage

### TUI Visual Interface

```bash
# Real-time capture with TUI (recommended)
cap tui -i enp89s0

# Demo mode (no capture, simulated data)
cap tui
```

**TUI Controls:**
- `q` - Quit
- `p` - Pause/Resume
- `r` - Reset statistics

**TUI Display:**
- Real-time traffic waveform (packets/second, bytes/second)
- Protocol distribution bar chart (TCP/UDP/HTTP/DNS)
- Current and average throughput statistics
- Live connection list

### Real-time Capture (CLI)

```bash
# Capture from default interface
cap live

# Specify network interface
cap live -i eth0

# Apply BPF filter
cap live -i eth0 -f "port 80"

# Limit packet count
cap live -i eth0 -c 100
```

### List Network Devices

```bash
cap devices
```

### Offline Analysis (TODO)

```bash
# Analyze pcap file (not yet implemented)
cap analyze capture.pcap

# Filter by protocol
cap analyze capture.pcap -p http
```

### Traffic Statistics (TODO)

```bash
cap stats capture.pcap
```

## Project Structure

```
cap/
├── .claude/                    # Claude Code configuration
│   ├── requirements.md         # Project requirements
│   ├── rules/                  # Code standards
│   └── memory/                 # Session memory
├── src/
│   ├── capture/
│   │   ├── live.rs             # Real-time packet capture
│   │   ├── offline.rs          # Offline pcap analysis (TODO)
│   │   └── device.rs           # Network device enumeration (TODO)
│   ├── protocol/
│   │   ├── parser.rs           # PacketParser trait
│   │   ├── ethernet.rs         # L2 Ethernet frame
│   │   ├── ip.rs               # L3 IP protocol
│   │   ├── tcp.rs              # L4 TCP protocol
│   │   ├── udp.rs              # L4 UDP protocol
│   │   ├── http.rs             # L7 HTTP protocol
│   │   └── dns.rs              # L7 DNS protocol
│   ├── filter/
│   │   ├── bpf.rs              # BPF filter (TODO)
│   │   └── expression.rs       # Filter expression (TODO)
│   ├── output/
│   │   ├── console.rs          # Console table output
│   │   └── json.rs             # JSON output
│   ├── stats/
│   │   └── analyzer.rs         # Traffic statistics
│   └── tui/
│       ├── app.rs              # TUI application state
│       └── ui.rs               # TUI rendering
├── examples/
│   └── basic_capture.rs        # Example code
├── setup.sh                    # Build and install script
├── CLAUDE.md                   # Claude Code quick reference
└── README.md                   # This file
```

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# View dependency tree
cargo tree
```

## Protocol Support

| Protocol | Layer | Parsed Content |
|----------|-------|----------------|
| Ethernet | L2 | Source/Dest MAC, Frame type |
| IP (IPv4) | L3 | Source/Dest IP, Protocol, TTL |
| TCP | L4 | Source/Dest Port, Sequence, Flags |
| UDP | L4 | Source/Dest Port, Length |
| HTTP | L7 | Method, Path, Status (via port detection) |
| DNS | L7 | Query/Response (via port detection) |

## License

MIT

# cap

High-performance network packet capture and analysis tool written in Rust.

## Features

- **Real-time packet capture** from network interfaces
- **Offline pcap file analysis**
- **5 major protocol support**: Ethernet, IP (IPv4/IPv6), TCP, HTTP, DNS
- **Multiple output formats**: Console (colored tables), JSON
- **BPF filter support** for targeted capture

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at target/release/cap
```

## Usage

### Real-time Capture

```bash
# Capture from default interface
cap live

# Specify interface
cap live -i eth0

# Apply BPF filter
cap live -i eth0 -f "port 80"

# Output as JSON
cap live --json
```

### Offline Analysis

```bash
# Analyze pcap file
cap analyze capture.pcap

# Filter by protocol
cap analyze capture.pcap -p http
```

### List Devices

```bash
cap devices
```

### Traffic Statistics

```bash
cap stats capture.pcap
```

## Project Structure

```
cap/
├── .claude/                    # Claude Code 配置
│   ├── requirements.md         # 项目需求文档
│   ├── rules/                  # 代码规范和指令
│   └── memory/                 # 会话记忆
├── src/
│   ├── capture/      # 抓包核心
│   ├── protocol/     # 协议解析
│   ├── filter/       # BPF 过滤
│   ├── output/       # 输出
│   └── stats/        # 统计
├── tests/fixtures/   # 测试数据
└── examples/         # 示例代码
```

## Development

```bash
# Run all tests
cargo test

# Run specific test
cargo test -- test_http_parser

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Run example
cargo run --example basic_capture
```

## License

MIT

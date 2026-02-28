# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust 高性能网络抓包分析工具，支持实时抓包和 TUI 可视化界面。

## Build & Run Commands

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release

# 一键安装（构建 + 设置权限）
./setup.sh

# 运行程序
cap tui -i eth0          # TUI 实时抓包界面
cap live -i eth0         # 命令行实时抓包
cap devices              # 列出网络设备
cap analyze file.pcap    # 分析离线文件（待实现）
```

## Project Structure

```
src/
├── main.rs              # CLI 入口，clap 命令解析
├── lib.rs               # 库模块导出
├── capture/             # 抓包核心模块
│   ├── live.rs          # 实时抓包（✅ 已实现）
│   ├── offline.rs       # 离线 pcap 文件读取（待实现）
│   └── device.rs        # 网络设备枚举（待实现）
├── protocol/            # 协议解析模块
│   ├── parser.rs        # PacketParser trait
│   ├── ethernet.rs      # L2 以太网帧
│   ├── ip.rs            # L3 IP 协议
│   ├── tcp.rs           # L4 TCP 协议
│   ├── udp.rs           # L4 UDP 协议
│   ├── http.rs          # L7 HTTP 协议（端口识别）
│   └── dns.rs           # L7 DNS 协议（端口识别）
├── filter/              # BPF 过滤器（待实现）
├── output/              # 输出模块
│   ├── console.rs       # 终端表格输出
│   └── json.rs          # JSON 输出
├── stats/               # 流量统计
│   └── analyzer.rs      # 统计模块
└── tui/                 # TUI 可视化（✅ 已实现）
    ├── app.rs           # 应用状态管理
    └── ui.rs            # 界面渲染
```

## Architecture

- **抓包层**: 使用 `pcap` crate 进行底层抓包，支持 BPF 过滤
- **解析层**: 使用 `etherparse` crate 解析协议头，端口识别协议
- **输出层**: 终端彩色表格（comfy-table）和 JSON 格式
- **TUI 层**: ratatui 框架，实时流量波形图 + 协议分布

## Current Status

**Phase 1-4 已完成**:
- ✅ 项目结构和 AI 配置文件
- ✅ 5 种协议解析器（Ethernet/IP/TCP/UDP/HTTP/DNS）+ 单元测试
- ✅ CLI 框架（clap）和输出模块
- ✅ 实时抓包功能 (`capture/live.rs`)
- ✅ TUI 可视化模块（实时流量、协议分布）
- ✅ 安装脚本 (`setup.sh`)

**待实现**:
- ⏳ 离线 pcap 分析 (`capture/offline.rs`)
- ⏳ 网络设备枚举 (`capture/device.rs`)
- ⏳ BPF 过滤器完善 (`filter/bpf.rs`)
- ⏳ 活动连接跟踪显示

## Development Conventions

- 使用 `thiserror` 定义错误类型
- 协议解析器统一实现 `PacketParser` trait
- 所有公开 API 必须有文档注释
- 使用 `rustfmt` 默认格式

## Testing

```bash
# 运行测试
cargo test

# 运行单个测试
cargo test -- test_http_parser
```

## Dependencies

核心依赖：
- `pcap` - 底层抓包
- `etherparse` - 协议头解析
- `ratatui` + `crossterm` - TUI 框架
- `clap` - CLI 框架
- `comfy-table` - 终端表格

## File Permissions

实时抓包需要网络权限。`setup.sh` 会自动设置：
```bash
sudo setcap cap_net_raw,cap_net_admin=eip ./target/release/cap
```

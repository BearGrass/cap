# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust 高性能网络抓包分析工具，支持实时抓包和离线 pcap 文件分析。

## Build & Run Commands

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release

# 运行程序
cargo run -- live              # 实时抓包（默认网卡）
cargo run -- live -i eth0      # 指定网卡
cargo run -- analyze file.pcap # 分析离线文件

# 运行测试
cargo test
cargo test -- --nocapture      # 显示测试输出

# 运行单个测试
cargo test -- test_http_parser

# 代码格式化
cargo fmt

# Lint 检查
cargo clippy -- -D warnings

# 查看依赖树
cargo tree
```

## Project Structure

```
src/
├── main.rs           # CLI 入口，使用 clap 解析命令
├── lib.rs            # 库模块导出
├── capture/          # 抓包核心模块
│   ├── live.rs       # 实时抓包（pcap crate）
│   ├── offline.rs    # 离线 pcap 文件读取
│   └── device.rs     # 网络设备枚举
├── protocol/         # 协议解析模块（核心）
│   ├── parser.rs     # 通用 Parser trait
│   ├── ethernet.rs   # L2 以太网帧
│   ├── ip.rs         # L3 IP 协议
│   ├── tcp.rs        # L4 TCP 协议
│   ├── udp.rs        # L4 UDP 协议
│   ├── http.rs       # L7 HTTP 协议
│   └── dns.rs        # L7 DNS 协议
├── filter/           # BPF 过滤器
├── output/           # 输出模块（console/json）
├── stats/            # 流量统计
└── tui/              # TUI 可视化（新增）
```

## Architecture

- **抓包层**: 使用 `pcap` crate 进行底层抓包，支持 BPF 过滤
- **解析层**: 使用 `etherparse` crate 解析协议头，自定义 `PacketParser` trait
- **输出层**: 支持终端彩色表格（comfy-table）和 JSON 格式

## AI Configuration Files

项目使用 `.claude/` 目录管理 AI 相关配置：

| 文件 | 用途 |
|------|------|
| `.claude/requirements.md` | 项目需求文档（核心） |
| `.claude/rules/rust-standards.md` | Rust 代码规范 |
| `.claude/rules/project-instructions.md` | Claude Code 项目指令 |
| `.claude/memory/MEMORY.md` | Claude 会话记忆 |

## Current Status

**Phase 1-3 已完成**:
- ✅ 项目结构和 AI 配置文件
- ✅ 5 种协议解析器（Ethernet/IP/TCP/UDP/HTTP/DNS）+ 单元测试
- ✅ CLI 框架（clap）和输出模块骨架
- ✅ TUI 可视化模块（演示模式）

**待实现**:
- ⏳ 实时抓包功能 (`capture/live.rs`)
- ⏳ 离线 pcap 分析 (`capture/offline.rs`)
- ⏳ 网络设备枚举 (`capture/device.rs`)
- ⏳ BPF 过滤器 (`filter/bpf.rs`)
- ⏳ TUI 与抓包集成

## Development Conventions

- 使用 `thiserror` 定义错误类型
- 使用 `tokio` 异步运行时处理高并发
- 协议解析器统一实现 `PacketParser` trait
- 所有公开 API 必须有文档注释

## Testing

- 单元测试：每个协议解析器独立测试
- 集成测试：使用 `tests/fixtures/` 中的 pcap 文件
- 测试数据包使用 Wireshark 生成

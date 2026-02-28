# Claude Code 项目指令

本文档提供项目特定的 Claude Code 使用指令。

## 项目背景

这是一个用 Rust 实现的网络抓包分析工具，项目名称为 `cap`。

## 核心能力

1. **实时抓包** - 从网络接口实时捕获数据包
2. **离线分析** - 读取并分析 pcap/pcapng 文件
3. **协议解析** - 支持 5 种主流协议分析

## 当生成代码时

### 必须遵循

1. **使用现有的 `PacketParser` trait**
   - 所有协议解析器都必须实现这个 trait
   - 参考 `src/protocol/tcp.rs` 的实现模式

2. **错误处理**
   - 使用 `thiserror` 定义错误类型
   - 错误信息要具体、可操作

3. **代码结构**
   - 遵循现有模块结构
   - 新代码放在合适的模块中
   - 避免重复创建已有功能的代码

4. **测试**
   - 为新功能编写单元测试
   - 使用现有的测试数据格式

### 避免

1. 不要修改 `Cargo.toml` 中的依赖版本（除非明确要求）
2. 不要删除已有的公开 API
3. 不要忽略错误处理
4. 不要生成没有测试的代码

## 常用命令

```bash
# 构建
cargo build
cargo build --release

# 测试
cargo test
cargo test -- --nocapture

# 运行
cargo run -- live -i eth0
cargo run -- analyze file.pcap

# 代码质量
cargo fmt
cargo clippy -- -D warnings
```

## 关键文件路径

| 文件 | 用途 |
|------|------|
| `src/main.rs` | CLI 入口 |
| `src/lib.rs` | 库导出 |
| `src/protocol/parser.rs` | Parser trait 定义 |
| `src/capture/live.rs` | 实时抓包（待实现） |
| `src/capture/offline.rs` | 离线分析（待实现） |
| `.claude/requirements.md` | 项目需求文档 |

## AI 编程最佳实践

### 1. 增量开发
- 小步提交，频繁验证
- 每次修改后运行 `cargo build` 和 `cargo test`

### 2. 代码审查
- 生成代码后检查警告
- 确保没有引入 `dead_code` 或 `unused` 警告

### 3. 测试驱动
- 先写测试用例
- 再实现功能
- 确保测试通过

### 4. 文档同步
- 新功能添加文档注释
- 更新 CLAUDE.md 记录变更

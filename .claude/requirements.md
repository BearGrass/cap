# 项目需求文档

## 项目概述

**项目名称**: cap (Capture And Protocol analyzer)

**项目描述**: 用 Rust 实现的高性能网络抓包分析工具，支持实时抓包和离线 pcap 文件分析。

---

## 核心需求

### 1. 实时抓包功能
- 从指定网络接口实时捕获数据包
- 支持默认网卡自动选择
- 支持 BPF 过滤器（如 "port 80", "host 192.168.1.1"）
- 支持限制抓包数量（用于测试）

### 2. 离线分析功能
- 读取和分析 pcap/pcapng 格式文件
- 支持按协议类型过滤显示
- 支持多种输出格式（终端表格、JSON）

### 3. 协议分析支持
必须支持的 5 种主流协议：

| 协议 | 层级 | 解析内容 |
|------|------|----------|
| Ethernet | L2 | 源/目的 MAC 地址、帧类型 |
| IP (IPv4/IPv6) | L3 | 源/目的 IP、协议类型、TTL |
| TCP | L4 | 源/目的端口、序列号、标志位 |
| HTTP/1.x | L7 | 请求方法、路径、状态码、主机头 |
| DNS | L7 | 查询类型、域名、响应记录 |

### 4. 输出功能
- **终端输出**: 彩色表格展示，分层显示协议信息
- **JSON 输出**: 结构化数据，便于后续处理
- **统计信息**: 流量统计、协议分布
- **TUI 可视化**: 实时动态显示网络流量（新增）

### 4.1 TUI 可视化功能（新增）
**目标**: 在终端界面提供友好的可视化演示效果

**功能要求**:
- 实时刷新显示（每秒更新）
- 流量速率图表（使用 ASCII 或 Braille 字符）
- 协议分布饼图/条形图
- 活动连接列表（动态刷新）
- 颜色区分不同协议类型
- 支持离线 pcap 文件回放演示模式

**可视化内容**:
| 视图 | 内容 |
|------|------|
| 实时流量图 | 每秒包数/字节数波形图 |
| 协议分布 | TCP/UDP/HTTP/DNS 占比条形图 |
| Top 会话 | 最活跃的来源/目的 IP 对 |
| 端口统计 | 最常用端口号 |
| 告警信息 | 异常流量或错误提示

**新增依赖**:
- `ratatui` - TUI 框架
- `crossterm` - 终端操作（已有部分依赖）
- `unicode-bar` 或自定义 ASCII 图表

### 5. CLI 命令设计
```bash
cap live                          # 实时抓包（默认网卡）
cap live -i eth0                  # 指定网卡
cap live -f "port 80"             # BPF 过滤
cap live --json                   # JSON 输出

cap analyze file.pcap             # 分析离线文件
cap analyze file.pcap -p http     # 按协议过滤

cap devices                       # 列出可用设备
cap stats file.pcap               # 显示统计信息

# TUI 可视化（新增）
cap tui                           # 启动 TUI 界面
cap tui -i eth0                   # 实时抓包 + TUI
cap tui --replay file.pcap        # 离线文件回放演示
```

---

## 技术要求

### 开发环境
- Rust 2021 Edition
- Cargo 包管理器

### 核心依赖
- `pcap` - 底层抓包
- `etherparse` - 协议头解析
- `tokio` - 异步运行时
- `clap` - CLI 框架（derive 模式）
- `thiserror` - 错误处理
- `comfy-table` - 终端表格输出
- `serde` + `serde_json` - JSON 序列化
- `ratatui` - TUI 框架（新增）
- `crossterm` - 终端操作

### 代码规范
- 所有公开 API 必须有文档注释 `///`
- 错误处理使用 `thiserror` 定义类型化错误
- 协议解析器统一实现 `PacketParser` trait
- 使用 `rustfmt` 默认格式
- 使用 `clippy` 进行代码检查

---

## 项目结构要求

```
cap/
├── .claude/                  # Claude Code 配置
│   ├── rules/                # 代码规范和指令
│   ├── memory/               # 会话记忆
│   └── requirements.md       # 项目需求文档
├── src/
│   ├── capture/              # 抓包核心
│   ├── protocol/             # 协议解析
│   ├── filter/               # 过滤器
│   ├── output/               # 输出模块
│   ├── stats/                # 统计分析
│   └── tui/                  # TUI 可视化（新增）
├── tests/
│   └── fixtures/             # 测试用 pcap 文件
├── examples/                 # 示例代码
├── CLAUDE.md                 # Claude Code 快速指南
├── README.md                 # 项目说明
└── .gitignore
```

---

## 验收标准

### Phase 1 - 项目初始化 ✅
- [x] Cargo 项目结构完整
- [x] AI 辅助配置完成
- [x] 模块骨架搭建

### Phase 2 - 协议解析器 ✅
- [x] Ethernet 解析 + 单元测试
- [x] IP (IPv4/IPv6) 解析 + 单元测试
- [x] TCP 解析 + 单元测试
- [x] UDP 解析 + 单元测试
- [x] HTTP 解析 + 单元测试
- [x] DNS 解析 + 单元测试
- [x] 所有测试通过

### Phase 3 - CLI 框架 ✅
- [x] clap 命令行解析
- [x] 4 个子命令框架（live/analyze/devices/stats）
- [x] 输出模块骨架

### Phase 4 - 核心抓包功能 ⏳
- [ ] 实时抓包实现并测试
- [ ] 离线 pcap 文件读取
- [ ] 网络设备枚举
- [ ] BPF 过滤器编译和应用

### Phase 5 - 完整功能 ⏳
- [ ] 协议解析器与抓包集成
- [ ] 终端彩色输出
- [ ] JSON 导出功能
- [ ] 流量统计

### Phase 6 - TUI 可视化（新增）⏳
- [ ] TUI 框架搭建（ratatui）
- [ ] 实时流量波形图
- [ ] 协议分布条形图
- [ ] 活动连接列表
- [ ] 离线 pcap 回放模式
- [ ] 键盘交互（退出、暂停、切换视图）

---

## 修改历史

| 日期 | 变更内容 | 状态 |
|------|----------|------|
| 2026-02-28 | 初始需求文档创建 | ✅ |
| 2026-02-28 | 项目结构初始化 | ✅ |
| 2026-02-28 | 5 种协议解析器完成 | ✅ |
| 2026-02-28 | 迁移到 .claude/ 目录 | ✅ |
| 2026-02-28 | 新增 TUI 可视化需求 | ✅ |

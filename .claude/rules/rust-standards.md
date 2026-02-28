# Rust 代码规范

本文档定义项目的 Rust 代码编写规范，AI 助手生成代码时应遵循这些规范。

## 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 结构体 | PascalCase | `PacketParser`, `TcpHeader` |
| 枚举 | PascalCase | `PacketLayer`, `Protocol` |
| Trait | PascalCase | `PacketParser`, `Capture` |
| 函数 | snake_case | `parse_packet`, `list_devices` |
| 变量 | snake_case | `packet_count`, `src_mac` |
| 常量 | UPPER_SNAKE_CASE | `MAX_PACKET_SIZE` |
| 错误类型 | `{Name}Error` | `PacketError`, `ParseError` |

## 错误处理

### 库代码
使用 `thiserror` 定义类型化错误：

```rust
#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    #[error("Packet too short: expected {expected} bytes, got {actual}")]
    TooShort { expected: usize, actual: usize },

    #[error("Invalid protocol version: {0}")]
    InvalidVersion(u8),
}
```

### 应用代码
使用 `anyhow::Result` 作为主程序返回类型：

```rust
fn main() -> anyhow::Result<()> {
    // ...
}
```

### 错误信息规范
- 清晰描述问题
- 包含可能的原因
- 提供解决方案提示

## 协议解析器规范

所有协议解析器必须实现 `PacketParser` trait：

```rust
pub trait PacketParser {
    type Output;
    type Error;

    /// 解析原始数据包数据为结构化格式
    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error>;

    /// 获取协议名称
    fn protocol_name(&self) -> &'static str;
}
```

### 解析器实现要求
1. 提供 `new()` 构造函数
2. 实现 `PacketParser` trait
3. 定义类型化的错误类型
4. 编写单元测试覆盖正常和异常情况

## 文档注释

### 公开 API 必须有文档注释

```rust
/// 从网络接口捕获数据包
///
/// # 参数
/// * `interface` - 网络接口名称，如 "eth0"
/// * `filter` - BPF 过滤器表达式
///
/// # 返回
/// * `Ok(())` - 捕获成功
/// * `Err(CaptureError)` - 捕获失败
pub fn capture(interface: &str, filter: Option<&str>) -> Result<()> {
    // ...
}
```

### 模块文档
每个模块文件开头添加模块级文档：

```rust
//! 协议解析模块
//!
//! 提供 Ethernet、IP、TCP、HTTP、DNS 等协议的解析功能。
//!
//! # 示例
//!
//! ```rust
//! let parser = TcpParser::new();
//! let result = parser.parse(&data);
//! ```

```

## 代码风格

- 使用 `rustfmt` 默认配置
- 行宽限制 100 字符
- 复杂逻辑添加行内注释 `//`
- 避免过长的函数（建议 < 50 行）

## 测试规范

### 单元测试位置
测试代码放在被测试模块的 `#[cfg(test)] mod tests` 中：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_normal() {
        // ...
    }
}
```

### 测试用例命名
- `test_{function}_normal` - 正常情况
- `test_{function}_empty` - 空输入
- `test_{function}_invalid` - 无效输入
- `test_{function}_boundary` - 边界条件

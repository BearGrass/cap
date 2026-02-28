#!/bin/bash
# 网络抓包工具 - 构建和权限设置脚本
# 用法：./setup.sh

set -e

echo "=== cap 构建和权限设置 ==="
echo ""

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 1. 构建发布版本
echo "[1/3] 编译发布版本..."
cargo build --release

# 2. 设置 capabilities（允许非 root 用户抓包）
echo "[2/3] 设置网络抓包权限..."
BINARY_PATH="./target/release/cap"

if [ ! -f "$BINARY_PATH" ]; then
    echo "错误：找不到二进制文件 $BINARY_PATH"
    exit 1
fi

# 设置 cap_net_raw 和 cap_net_admin 权限
sudo setcap cap_net_raw,cap_net_admin=eip "$BINARY_PATH"

# 3. 创建全局命令（复制到 /usr/local/bin）
echo "[3/3] 创建全局命令..."
SUDO_CMD=""
if ! [ -w "/usr/local/bin" ]; then
    SUDO_CMD="sudo"
fi

# 复制二进制文件到 /usr/local/bin
$SUDO_CMD cp "$BINARY_PATH" /usr/local/bin/cap

# 对 /usr/local/bin/cap 也设置权限
$SUDO_CMD setcap cap_net_raw,cap_net_admin=eip /usr/local/bin/cap

# 4. 验证设置
echo "[4/4] 验证权限设置..."
if getcap /usr/local/bin/cap | grep -q "cap_net_raw"; then
    echo "✓ 权限设置成功！"
    echo ""
    echo "=== 完成 ==="
    echo ""
    echo "现在可以在任何地方运行："
    echo "  cap tui -i enp89s0    # 实时抓包 TUI 界面"
    echo "  cap live -i enp89s0   # 实时抓包（命令行）"
    echo "  cap devices           # 列出网络设备"
    echo "  cap --help            # 查看帮助"
    echo ""
    echo "如果命令未找到，请运行："
    echo "  export PATH=\$PATH:/usr/local/bin"
    echo ""
else
    echo "✗ 权限设置可能失败，请检查错误信息"
    exit 1
fi

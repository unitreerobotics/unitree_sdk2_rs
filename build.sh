#!/bin/bash
# 构建测试 bin（bms_test / rpc_test_server / rpc_test_client）并绿色打包
# 仅 Linux（Docker）使用：C++ FFI 正常编译链接。
#
# .so 拷贝：使用无版本文件名（libddsc.so / libddscxx.so，不用 .0），
# 二进制 NEEDED 记录的是 SDK SONAME（libddsc.so.0），
# 因此用 patchelf 把 NEEDED 改写为无版本名 + rpath 设为 $ORIGIN/../lib。
set -euo pipefail
export PATH=/root/.cargo/bin:$PATH

cargo build --release

apt-get install -y -qq patchelf 2>/dev/null || true

# ── 构建产物根：用 cargo 查真实 target 目录（workspace 共享 target 或独立 crate 都兼容）──
TARGET_DIR=$(cargo metadata --format-version 1 --no-deps 2>/dev/null \
    | grep -o '"target_directory":"[^"]*"' | head -1 | cut -d'"' -f4)
[ -n "$TARGET_DIR" ] || TARGET_DIR=target
BIN_DIR="$TARGET_DIR/release"

# ── 定位 SDK thirdparty lib ──
# 优先级：
#   ① build.rs 写出的 sdk_dir.txt（实际链接用的 SDK：覆盖 config PATH / env / clone 所有来源）
#   ② 环境变量显式设置：UNITREE_SDK2_PATH / UNITREE_SDK2_DIR
#   ③ clone 产物兜底：$TARGET_DIR/release/build/*/out/tmp_sdk2
ARCH=$(uname -m)
SDK=""
SDK_FILE=$(find "$TARGET_DIR" -maxdepth 4 -name sdk_dir.txt 2>/dev/null | head -1)
if [ -n "$SDK_FILE" ]; then
    SDK_ROOT=$(head -1 "$SDK_FILE")
    if [ -n "$SDK_ROOT" ] && [ -d "$SDK_ROOT/thirdparty/lib/$ARCH" ]; then
        SDK="$SDK_ROOT/thirdparty/lib/$ARCH"
    fi
fi
if [ -z "$SDK" ] && [ -n "${UNITREE_SDK2_PATH:-}" ] && [ -d "$UNITREE_SDK2_PATH/thirdparty/lib/$ARCH" ]; then
    SDK="$UNITREE_SDK2_PATH/thirdparty/lib/$ARCH"
elif [ -z "$SDK" ] && [ -n "${UNITREE_SDK2_DIR:-}" ] && [ -d "$UNITREE_SDK2_DIR/thirdparty/lib/$ARCH" ]; then
    SDK="$UNITREE_SDK2_DIR/thirdparty/lib/$ARCH"
fi
if [ -z "$SDK" ]; then
    SDK_DIR=$(find "$TARGET_DIR/release/build" -maxdepth 3 -type d -name tmp_sdk2 -path "*/out/tmp_sdk2" 2>/dev/null | head -1)
    if [ -n "$SDK_DIR" ] && [ -d "$SDK_DIR/thirdparty/lib/$ARCH" ]; then
        SDK="$SDK_DIR/thirdparty/lib/$ARCH"
    fi
fi
if [ -z "$SDK" ]; then
    echo "错误: 找不到 SDK thirdparty lib（设置 UNITREE_SDK2_PATH / UNITREE_SDK2_DIR，或先构建以 clone 出 SDK）" >&2
    exit 1
fi

# ── 打包 ──
rm -rf install
mkdir -p install/bin install/lib
for b in bms_test rpc_test_server rpc_test_client; do
    [ -x "$BIN_DIR/$b" ] && cp "$BIN_DIR/$b" install/bin/
done

# 拷贝无版本名 .so
cp "$SDK"/libddsc.so "$SDK"/libddscxx.so install/lib/

# patchelf：NEEDED 改写为无版本名（匹配拷贝的文件名）+ rpath 统一 $ORIGIN/../lib
for b in install/bin/bms_test install/bin/rpc_test_server install/bin/rpc_test_client; do
    [ -x "$b" ] || continue
    patchelf --replace-needed libddsc.so.0   libddsc.so   "$b"
    patchelf --replace-needed libddscxx.so.0 libddscxx.so "$b"
    patchelf --set-rpath '$ORIGIN/../lib' "$b"
done

echo "✓ 打包完成: install/（SDK: $SDK）"

#!/usr/bin/env sh
# 工具链引导：从零把本仓库的全部依赖装到用户态（不碰系统、不需 sudo，除 build-essential）。
# 幂等：已装的跳过。版本号只写在这里——升级改这一处，别处不许再抄。
# 记录于 docs/architecture.md 待定项 3（2026-08-15 落定）。
set -eu

# ── 版本 SSOT ────────────────────────────────────────────────
DITA_OT_VERSION="4.4"
TEMURIN_MAJOR="17"
# oasis-tcs/dita 锁定版本：upstream-index 的来源之一，索引头记的就是这个值。
# 用 tag 不用分支名——分支会动，索引与来源就对不上了。
# v2.0-beta03 = commit 0a8920b3590e28dd87b1d70dd38d60d8e7406ddd（2026-06-30）
OASIS_DITA_REF="v2.0-beta03"
TOOLS_DIR="$HOME/ws/tools"
BIN_DIR="$HOME/.local/bin"
# ─────────────────────────────────────────────────────────────

mkdir -p "$TOOLS_DIR" "$BIN_DIR"
say() { printf '%s\n' "$*"; }

# 1. C 链接器（Rust 编译必需；唯一需要 sudo 的一步，只提示不代跑）
if ! command -v cc >/dev/null 2>&1; then
  say "⚠ 缺 C 链接器：请自行执行  sudo apt install -y build-essential  后重跑本脚本"
  exit 1
fi

# 2. uv（kb/scripts 里的 .py 靠它跑：脚本内 PEP 723 头声明 requires-python，
#    不依赖系统 python——Ubuntu 的 python3 是 externally-managed，装不了任何依赖）
if ! command -v uv >/dev/null 2>&1 && [ ! -x "$BIN_DIR/uv" ]; then
  say "装 uv…"
  curl -LsSf https://astral.sh/uv/install.sh | sh
fi
# 安装器可能听 XDG_BIN_HOME 而落在别处；统一软链进 BIN_DIR，
# 与 java/dita/cargo 同一套出口，"把 BIN_DIR 加进 PATH"才是充分条件
UV_BIN="$(command -v uv || true)"
[ -n "$UV_BIN" ] || UV_BIN="$(find "$HOME" -maxdepth 4 -type f -name uv -perm -u+x 2>/dev/null | head -1)"
[ -n "$UV_BIN" ] && ln -sf "$UV_BIN" "$BIN_DIR/uv"
say "✓ uv → ${UV_BIN:-未找到，请手动确认}"

# 3. Temurin JRE
JRE_DIR=$(find "$TOOLS_DIR" -maxdepth 1 -type d -name "jdk-${TEMURIN_MAJOR}*" | head -1)
if [ -z "$JRE_DIR" ]; then
  say "装 Temurin ${TEMURIN_MAJOR} JRE…"
  curl -sL "https://api.adoptium.net/v3/binary/latest/${TEMURIN_MAJOR}/ga/linux/x64/jre/hotspot/normal/eclipse" \
    -o "$TOOLS_DIR/jre.tar.gz"
  tar xzf "$TOOLS_DIR/jre.tar.gz" -C "$TOOLS_DIR" && rm "$TOOLS_DIR/jre.tar.gz"
  JRE_DIR=$(find "$TOOLS_DIR" -maxdepth 1 -type d -name "jdk-${TEMURIN_MAJOR}*" | head -1)
fi
ln -sf "$JRE_DIR/bin/java" "$BIN_DIR/java"
say "✓ java → $JRE_DIR"

# 4. DITA-OT
OT_DIR="$TOOLS_DIR/dita-ot-${DITA_OT_VERSION}"
if [ ! -d "$OT_DIR" ]; then
  say "装 DITA-OT ${DITA_OT_VERSION}…"
  curl -sL "https://github.com/dita-ot/dita-ot/releases/download/${DITA_OT_VERSION}/dita-ot-${DITA_OT_VERSION}.zip" \
    -o "$TOOLS_DIR/dita-ot.zip"
  unzip -q "$TOOLS_DIR/dita-ot.zip" -d "$TOOLS_DIR" && rm "$TOOLS_DIR/dita-ot.zip"
fi
# dita 包装器：保证 JAVA_HOME 就位；review.sh 的 Saxon 探测由 dita 可执行位置反推
cat > "$BIN_DIR/dita" <<WRAP
#!/bin/sh
export JAVA_HOME="\${JAVA_HOME:-$JRE_DIR}"
export PATH="\$JAVA_HOME/bin:\$PATH"
exec "$OT_DIR/bin/dita" "\$@"
WRAP
chmod +x "$BIN_DIR/dita"
say "✓ dita → $OT_DIR（markdown 插件 org.lwdita 随 4.4 自带）"

# 5. OASIS DITA 规范源（只读克隆；dita-tools upstream-index 的来源之一）
OASIS_DIR="$TOOLS_DIR/oasis-dita"
if [ ! -d "$OASIS_DIR/.git" ]; then
  say "克隆 oasis-tcs/dita…（约 34 MB）"
  git clone --quiet https://github.com/oasis-tcs/dita.git "$OASIS_DIR"
fi
# 已有克隆也按 SSOT 的 ref 同步：不同机器上的索引必须由同一份源生成，
# 否则"上游改名"与"我这份克隆旧了"两种 diff 分不开
OASIS_WANT="$(git -C "$OASIS_DIR" rev-parse --verify --quiet "${OASIS_DITA_REF}^{commit}" || true)"
if [ -z "$OASIS_WANT" ]; then
  git -C "$OASIS_DIR" fetch --quiet --tags origin
  OASIS_WANT="$(git -C "$OASIS_DIR" rev-parse "${OASIS_DITA_REF}^{commit}")"
fi
if [ "$(git -C "$OASIS_DIR" rev-parse HEAD)" != "$OASIS_WANT" ]; then
  git -C "$OASIS_DIR" checkout --quiet --detach "$OASIS_DITA_REF"
fi
say "✓ oasis-dita → $OASIS_DIR（$OASIS_DITA_REF）"

# 6. Rust + just + dita-tools
if ! command -v cargo >/dev/null 2>&1 && [ ! -x "$HOME/.cargo/bin/cargo" ]; then
  say "装 rustup…"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
fi
CARGO="$HOME/.cargo/bin/cargo"
for t in cargo rustc rustfmt; do ln -sf "$HOME/.cargo/bin/$t" "$BIN_DIR/$t"; done
command -v just >/dev/null 2>&1 || { say "装 just…"; "$CARGO" install --quiet just; }
ln -sf "$HOME/.cargo/bin/just" "$BIN_DIR/just"

REPO="$(cd "$(dirname "$0")/.." && pwd)"
say "装 dita-tools…"
(cd "$REPO/dita-tools" && "$CARGO" install --quiet --path apps/dita_cli)
ln -sf "$HOME/.cargo/bin/dita-tools" "$BIN_DIR/dita-tools"

say ""
say "完成。请确认 $BIN_DIR 在 PATH 上，然后在仓库根目录跑：just check"

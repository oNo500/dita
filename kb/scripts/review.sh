#!/usr/bin/env sh
# 机器兜底：一条命令串全套——RNG 结构校验 + 业务规则 R1–R10 + 术语扫描。
# 只用 DITA-OT 自带工具（dita validate + 自带 Saxon + python3），不装额外东西。
# 有 error 则退出非零，可挡入库 / 接 git hook / CI。
# 设计见 dita2 cases/kb-redesign/machine-checks-design.md。
set -u
KB="$(cd "$(dirname "$0")/.." && pwd)"
XSL="$KB/scripts/check-rules.xsl"

# 探测 DITA-OT 自带 Saxon 及其依赖 jar（lib/*），不硬编码版本与安装位置。
# 顺序：显式 DITA_HOME → dita 可执行文件所在目录反推 → 常见安装路径（Linux 用户态 / macOS homebrew）。
if [ -z "${DITA_HOME:-}" ]; then
  DITA_BIN="$(command -v dita 2>/dev/null || true)"
  if [ -n "$DITA_BIN" ]; then
    # dita 可能是包装器或软链，逐层解到真身再取上级目录
    while [ -L "$DITA_BIN" ]; do DITA_BIN="$(readlink -f "$DITA_BIN")"; done
    DITA_HOME="$(dirname "$(dirname "$DITA_BIN")")"
  fi
fi
SEARCH_DIRS="${DITA_HOME:-} $HOME/ws/tools/dita-ot-* /opt/dita-ot-* /opt/homebrew/Cellar/dita-ot /usr/local/Cellar/dita-ot"
SAXON_JAR="$(find $SEARCH_DIRS -name 'Saxon-HE-*.jar' 2>/dev/null | head -1)"
skipped=0
if [ -z "$SAXON_JAR" ]; then
  echo "找不到 DITA-OT 自带 Saxon，业务规则检查跳过（改这里的探测路径）。" >&2
  CP=""
  skipped=1
else
  CP="$(dirname "$SAXON_JAR")/*"
fi

fail=0

echo "== 1. 结构校验（RNG）+ 业务规则（R1–R10）=="
for f in $(find "$KB/topics" -name '*.dita' | sort); do
  rel="${f#"$KB"/}"
  # 结构（RNG shell）
  if ! dita validate --input="$f" >/tmp/kb-rv.log 2>&1; then
    echo "[structure] $rel"; sed 's/^/    /' /tmp/kb-rv.log; fail=1
  fi
  # 业务规则
  if [ -n "$CP" ]; then
    out="$(java -cp "$CP" net.sf.saxon.Transform -s:"$f" -xsl:"$XSL" 2>/dev/null)"
    if [ -n "$out" ]; then
      echo "$out" | sed "s#^#[rules] $rel: #"
      echo "$out" | grep -q 'error' && fail=1
    fi
  fi
done

# 维度覆盖度已归 IA 治理（库的形状，非单篇规则），看 `just ia`；
# 脚本 dimension-coverage.py 于 2026-08-15 走完吸收五关退役。

echo
echo "== 2. 术语规整建议（报告版，不阻断入库）=="
uv run --script "$KB/scripts/term-normalize.py"

echo
# 跳过 ≠ 通过：Saxon 缺失时 R1–R10 一条都没跑，此时报"全过"是假绿，必须挡住。
if [ "$skipped" -ne 0 ]; then
  echo "⚠️  业务规则 R1–R10 未执行（找不到 Saxon），本次结果不能当作通过依据"
  exit 2
fi
if [ "$fail" -eq 0 ]; then
  echo "✅ 结构与业务规则全过"
else
  echo "❌ 有 error（见上），阻断入库"
fi
exit "$fail"

#!/usr/bin/env bash
# 机器兜底：一条命令串全套——RNG 结构校验 + 业务规则 R1–R10 + 题材文体 R12–R15 + 术语扫描。
# 依赖：DITA-OT（dita validate + 自带 Saxon）与 uv（跑 kb/scripts 下的 .py）。
# 两者缺任何一个都不会静默放行——缺什么就少跑什么，且结果不得当作通过。
# 有 error 则退出非零，可挡入库 / 接 git hook / CI。
# 设计见 dita2 cases/kb-redesign/machine-checks-design.md。
set -uo pipefail
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

term_skipped=0
lint_skipped=0
if ! command -v uv >/dev/null 2>&1; then
  echo "找不到 uv，术语扫描跳过（装：scripts/setup-env.sh）。" >&2
  term_skipped=1
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
echo "== 2. 题材与文体 R12–R15（dita-tools lint；draft 记 warning，晋级门）=="
if command -v dita-tools >/dev/null 2>&1; then
  dita-tools lint --vocab "$KB/vocab/subjectScheme.ditamap" "$KB/topics" || fail=1
else
  echo "找不到 dita-tools，R12–R15 未执行（装：scripts/setup-env.sh）" >&2
  lint_skipped=1
fi

echo
echo "== 3. 术语规整建议（报告版，不阻断入库）=="
if [ "$term_skipped" -eq 0 ]; then
  uv run --script "$KB/scripts/term-normalize.py"
else
  echo "（跳过：uv 不可用）"
fi

echo
# 跳过 ≠ 通过：某项检查没跑就报"全过"是假绿。但"没跑"也不能盖住"真失败"——
# 确定的失败先说，未执行随后说，两者可同时成立，退出码取更确定的那个（失败 1 > 跳过 2）。
if [ "$skipped" -ne 0 ]; then
  echo "⚠️  业务规则 R1–R10 未执行（找不到 Saxon），本次结果不能当作通过依据"
fi
if [ "$term_skipped" -ne 0 ]; then
  echo "⚠️  术语扫描未执行（找不到 uv）"
fi
if [ "$lint_skipped" -ne 0 ]; then
  echo "⚠️  R12–R15 未执行（找不到 dita-tools）"
fi
if [ "$fail" -ne 0 ]; then
  echo "❌ 有 error（见上），阻断入库"
  exit 1
fi
if [ "$skipped" -ne 0 ] || [ "$term_skipped" -ne 0 ] || [ "$lint_skipped" -ne 0 ]; then
  exit 2
fi
if [ "$fail" -eq 0 ]; then
  echo "✅ 结构与业务规则全过"
else
  echo "❌ 有 error（见上），阻断入库"
fi
exit "$fail"

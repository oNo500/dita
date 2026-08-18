#!/usr/bin/env bash
# 机器兜底：一条命令串全套——RNG 结构校验 + 业务规则 R1–R10 + 体裁文体 R12–R15 + 术语扫描。
# 依赖：DITA-OT（dita validate + 自带 Saxon）与 uv（跑 kb/scripts 下的 .py）。
# 两者缺任何一个都不会静默放行——缺什么就少跑什么，且结果不得当作通过。
# 有 error 则退出非零，可挡入库 / 接 git hook / CI。
# 设计见 dita2 cases/kb-redesign/machine-checks-design.md。
#
# 性能改造（2026-08-16，见 .superpowers/sdd/2026-08-16-notes-to-kb-migration/
# review-batching-report.md）：结构校验与业务规则原先各文件起一次 JVM，73 篇
# ×2＝146 次；改批量调用后每轮固定 2–3 次（root map 1 次 ＋ deliverables map
# 若干次 ＋ Saxon 业务规则 1 次），检查覆盖面与严格性不变，只是把"文件级循环"挪进
# DITA-OT 自己的 map 遍历和 XSLT 3.0 的 collection()，不再逐篇起 JVM。
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
# 多个候选时取版本号最大的那个（sort -V），不是文件系统碰巧先返回的那个：
# check-rules.xsl 声明 version="3.0" 且用 uri-collection()，装了两版时挑到旧的会整层失效。
SAXON_JAR="$(find $SEARCH_DIRS -name 'Saxon-HE-*.jar' 2>/dev/null | sort -V | tail -1)"
skipped=0
skip_reason=""
if [ -z "$SAXON_JAR" ]; then
  echo "找不到 DITA-OT 自带 Saxon，业务规则检查跳过（改这里的探测路径）。" >&2
  CP=""
  skipped=1
  skip_reason="找不到 Saxon"
else
  CP="$(dirname "$SAXON_JAR")/*"
  # 版本门：XSLT 3.0（uri-collection()/xsl:try）自 Saxon-HE 9.8 起支持。更旧的版本
  # 会在编译期就报错——那条路下面的退出码检查也拦得住，但那时的报错是一堆 XSLT 语法
  # 错误，指不到根因。这里先问一句版本，把"环境不合格"和"规则真违规"分开报。
  # net.sf.saxon.Version 把版本横幅写 stderr（不是 stdout），所以这里必须 2>&1。
  saxon_ver="$(java -cp "$CP" net.sf.saxon.Version 2>&1 | head -1)"
  saxon_num="$(printf '%s' "$saxon_ver" | sed -n 's/.*HE \([0-9][0-9]*\.[0-9][0-9]*\).*/\1/p')"
  if [ -z "$saxon_num" ]; then
    echo "Saxon 版本探测失败（$SAXON_JAR），无法确认支持 XSLT 3.0，业务规则检查跳过。" >&2
    skipped=1
    skip_reason="Saxon 版本无法确认"
  else
    saxon_major="${saxon_num%%.*}"
    saxon_minor="${saxon_num#*.}"
    if [ "$saxon_major" -lt 9 ] || { [ "$saxon_major" -eq 9 ] && [ "$saxon_minor" -lt 8 ]; }; then
      echo "Saxon $saxon_num 不支持 XSLT 3.0（需 ≥ 9.8），业务规则检查跳过：$SAXON_JAR" >&2
      skipped=1
      skip_reason="Saxon $saxon_num 不支持 XSLT 3.0"
    fi
  fi
fi

term_skipped=0
lint_skipped=0
if ! command -v uv >/dev/null 2>&1; then
  echo "找不到 uv，术语扫描跳过（装：scripts/setup-env.sh）。" >&2
  term_skipped=1
fi

fail=0

echo "== 1. 结构校验（RNG）+ 业务规则（R1–R10）=="

# 结构：dita validate 以一个 map 为输入，DITA-OT 自己沿 topicref/mapref 树遍历全部被引 topic，
# 一次调用顶过去一篇篇起 JVM。root.ditamap 是 domain 骨架的根，覆盖 kb/topics 下绝大多数
# topic；但"骨架"和"引用"是两回事——只被 kb/maps/deliverables/*.ditamap 引用、不在任何
# domain 分支下的 topic（当前如 agent-rules-core.dita、dita-authoring-guide.dita）root
# map 走不到，所以 deliverables 下每个 map 也各验一次。
# 覆盖完整性已核过（2026-08-16，独立脚本核验，见迁移报告）：root.ditamap ∪
# kb/maps/deliverables/*.ditamap＝kb/topics 下全部 .dita，0 篇漏网。往后再漏（新 topic
# 哪个 map 都不进）归 `just ia`「不在任何分支下」那行兜底，不在这里重复查。
STRUCT_MAPS="$KB/maps/root.ditamap"
for m in "$KB"/maps/deliverables/*.ditamap; do
  [ -e "$m" ] && STRUCT_MAPS="$STRUCT_MAPS $m"
done
for m in $STRUCT_MAPS; do
  rel="${m#"$KB"/}"
  if ! dita validate --input="$m" >/tmp/kb-rv.log 2>&1; then
    echo "[structure] $rel"; sed 's/^/    /' /tmp/kb-rv.log; fail=1
  fi
done

# 业务规则：一次 Saxon 调用，check-rules.xsl 的 main 模板用 XSLT 3.0 collection()
# 遍历 kb/topics 下全部 .dita 各跑一遍 R1–R10，每条违规行自带 "[rules] <rel>: " 前缀
# （文件名前缀现在是 XSLT 自己出的，不再靠这里 sed 拼）。单篇 XML 解析失败（格式错）
# 用 xsl:try 隔离，只报该篇 R0，不拖垮其余各篇的检查结果——那篇的结构性错误本身，
# 上面的 dita validate 已经带着文件名行号报过了。
#
# 退出码必须看：Saxon 失败时 stdout 常常是空的（编译错、找不到 -it 入口、CP 缺 jar 都
# 是这样），只判 "$out" 非空等于把"没跑成"读成"零违规"。批量化之后这一层是一次调用，
# 一次失败＝R1–R10 整层失效，还会被下面的 "✅ 全过" 盖住——正是本脚本第 4–5 行
# 与 102–103 行明令禁止的假绿。所以非零退出码走 skipped 通道（exit 2），不走 fail。
if [ "$skipped" -eq 0 ]; then
  out="$(java -cp "$CP" net.sf.saxon.Transform -it:main -xsl:"$XSL" "kb-dir=file://$KB" 2>/tmp/kb-rules-err.log)"
  rc=$?
  if [ "$rc" -ne 0 ]; then
    echo "Saxon 执行失败（退出码 $rc），业务规则 R1–R10 未执行：" >&2
    sed 's/^/    /' /tmp/kb-rules-err.log >&2
    skipped=1
    skip_reason="Saxon 执行失败（退出码 $rc）"
  elif [ -n "$out" ]; then
    echo "$out"
    echo "$out" | grep -q 'error' && fail=1
  fi
fi

# 维度覆盖度已归 IA 治理（库的形状，非单篇规则），看 `just ia`；
# 脚本 dimension-coverage.py 于 2026-08-15 走完吸收五关退役。

echo
echo "== 2. 体裁与文体 R12–R15（dita-tools lint；draft 记 warning，晋级门）=="
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
  echo "⚠️  业务规则 R1–R10 未执行（$skip_reason），本次结果不能当作通过依据"
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

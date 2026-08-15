#!/usr/bin/env bash
# 从单一 DITA 源构建各 agent 工具的规则文件（Phase 4 验证的核心主张）。
# 同一 map + 各工具的 DITAVAL → 各工具变体；chunk="combine" 合成单文件。
set -euo pipefail
cd "$(dirname "$0")/.."

MAP=maps/deliverables/agent-rules.ditamap

# 工具列表从 filters/ 派生，不硬编码：新增一个 DITAVAL 就自动出一个变体。
# 与词表 tool 值集的差额由 `dita-tools ia --details` 的「受控值使用情况」盯着
# （定义了却无 DITAVAL 的工具会显示为"未用"）——两处不必互相硬编码。
tools=$(ls filters/tool-*.ditaval | sed 's|filters/tool-||; s|\.ditaval$||')
[ -n "$tools" ] || { echo "filters/ 下没有 tool-*.ditaval，无变体可建" >&2; exit 1; }

for tool in $tools; do
  dita -f markdown_github --input="$MAP" --filter="filters/tool-$tool.ditaval" -o "out/$tool" >/dev/null
  cp "out/$tool/agent-rules.md" "out/$tool.md"   # out/<tool>.md 即该工具的规则文件雏形
  echo "built out/$tool.md"
done
echo "改一处源重跑本脚本，所有工具变体同步更新——这是单源多变体的兑现。"

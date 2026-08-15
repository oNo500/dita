#!/usr/bin/env bash
# 从单一 DITA 源构建各 agent 工具的规则文件（Phase 4 验证的核心主张）。
# 同一 map + 各工具的 DITAVAL → 各工具变体；chunk="combine" 合成单文件。
set -euo pipefail
cd "$(dirname "$0")/.."

MAP=maps/deliverables/agent-rules.ditamap
for tool in claude-code codex; do
  dita -f markdown_github --input="$MAP" --filter="filters/tool-$tool.ditaval" -o "out/$tool" >/dev/null
  cp "out/$tool/agent-rules.md" "out/$tool.md"   # out/<tool>.md 即该工具的规则文件雏形
  echo "built out/$tool.md"
done
echo "改一处源重跑本脚本，所有工具变体同步更新——这是单源多变体的兑现。"

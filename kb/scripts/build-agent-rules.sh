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
  # 先清干净：DITA-OT 的输出布局会随 topic 所在目录、以及 xref 是否拉入 map 外的 topic 而变。
  # 残留的旧产物会让下面的定位撞上过期文件——2026-08-15 就发生过：topic 换目录后合并产物
  # 移到了 out/<tool>/maps/deliverables/ 下，脚本仍 cp 老路径，连续多次"构建成功"发的都是陈旧内容。
  rm -rf "out/$tool"
  dita -f markdown_github --input="$MAP" --filter="filters/tool-$tool.ditaval" -o "out/$tool" >/dev/null

  # 不硬编码产物路径：找出本次生成的合并文件，找不到就失败，绝不 cp 一个不知来历的文件
  built=$(find "out/$tool" -type f -name 'agent-rules.md' | head -1)
  [ -n "$built" ] || { echo "构建未产出 agent-rules.md（输出布局可能又变了），请检查 out/$tool" >&2; exit 1; }
  [ -s "$built" ] || { echo "产出的 $built 是空文件" >&2; exit 1; }
  cp "$built" "out/$tool.md"   # out/<tool>.md 即该工具的规则文件雏形
  echo "built out/$tool.md（源：${built#out/$tool/}）"
done
echo "改一处源重跑本脚本，所有工具变体同步更新——这是单源多变体的兑现。"

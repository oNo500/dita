#!/usr/bin/env bash
# 从单一 DITA 源构建各 agent 工具的规则文件（Phase 4 验证的核心主张）。
# 同一 map + 各工具的 DITAVAL → 各工具变体；chunk="combine" 合成单文件。
set -euo pipefail
cd "$(dirname "$0")/.."

MAP=maps/deliverables/agent-rules.ditamap

# 已知可忽略的一条 DITA-OT 错误，白名单在此、理由在此，不在别处静默吞掉。
#
# 症状：`[DOTX031E] The 'dita-authoring-guide.dita' resource is not available to
# resolve link information.`，出自 writing-style.dita 与 terminology-rules.dita。
#
# 成因：`chunk="combine"` 把 dita-authoring-guide 并进合成文件，它作为独立产物的
# URI 随之消失。而该篇是路由总纲，xref 出去的十篇同簇正本被 DITA-OT 拉进同一个
# 作业，其中两篇又 xref 回总纲——topicpull 这一步就找不到目标了。
#
# 为什么判定可忽略（三条都成立才放行）：
#   1. 不是源缺陷：`dita validate` 全过，库内这些 xref 本身正确、目标确实存在；
#   2. 不丢信息：两处 xref 都写了显式链接文字，链接文字不靠 topicpull 取；
#   3. 不影响交付物：受影响的只有 topics/ 下的**副产物** md，交付的是合成文件本身。
# 任何一条不再成立（比如换了不写链接文字的 xref），这条白名单就该撤掉。
IGNORABLE="DOTX031E.*'dita-authoring-guide\.dita' resource is not available"

# 工具列表从 filters/ 派生，不硬编码：新增一个 DITAVAL 就自动出一个变体。
# 与词表 tool 值集的差额由 `dita-tools ia --details` 的「受控值使用情况」盯着
# （定义了却无 DITAVAL 的工具会显示为"未用"）——两处不必互相硬编码。
tools=$(ls filters/tool-*.ditaval | sed 's|filters/tool-||; s|\.ditaval$||')
[ -n "$tools" ] || { echo "filters/ 下没有 tool-*.ditaval，无变体可建" >&2; exit 1; }

log=$(mktemp)
trap 'rm -f "$log"' EXIT

for tool in $tools; do
  # 先清干净：DITA-OT 的输出布局会随 topic 所在目录、以及 xref 是否拉入 map 外的 topic 而变。
  # 残留的旧产物会让下面的定位撞上过期文件——2026-08-15 就发生过：topic 换目录后合并产物
  # 移到了 out/<tool>/maps/deliverables/ 下，脚本仍 cp 老路径，连续多次"构建成功"发的都是陈旧内容。
  rm -rf "out/$tool"

  # dita 对 DOTX/DOTJ 这类错误照样返回 0，所以退出码不足以判断构建是否干净：
  # 必须自己读它的输出。2026-08-17 之前这里是 `>/dev/null`，4 条 DOTX031E 被直接丢掉，
  # 脚本照报 built——与上面那次"构建成功却发陈旧内容"的事故是同一类失明。
  if ! dita -f markdown_github --input="$MAP" --filter="filters/tool-$tool.ditaval" -o "out/$tool" >"$log" 2>&1; then
    echo "dita 构建失败（$tool）：" >&2
    cat "$log" >&2
    exit 1
  fi

  unexpected=$(grep -E '\[DOT[A-Z]+[0-9]+E\]' "$log" | grep -Ev "$IGNORABLE" || true)
  if [ -n "$unexpected" ]; then
    echo "dita 报了错误码却返回 0（$tool）——不在白名单内，按失败处理：" >&2
    echo "$unexpected" >&2
    exit 1
  fi
  ignored=$(grep -Ec "$IGNORABLE" "$log" || true)
  [ "$ignored" -eq 0 ] || echo "  （$tool：$ignored 条已知可忽略的 DOTX031E，理由见脚本顶部白名单注释）"

  # 不硬编码产物路径：找出本次生成的合并文件，找不到就失败，绝不 cp 一个不知来历的文件
  built=$(find "out/$tool" -type f -name 'agent-rules.md' | head -1)
  [ -n "$built" ] || { echo "构建未产出 agent-rules.md（输出布局可能又变了），请检查 out/$tool" >&2; exit 1; }
  [ -s "$built" ] || { echo "产出的 $built 是空文件" >&2; exit 1; }

  # 链接改写：交付物是给 agent 单独读的一份 markdown，不在 out/ 的目录结构里。
  # DITA-OT 发出的是相对被引 topic 的 `../../topics/x.md`——那只在原位可解析，
  # 提到 out/<tool>.md 后必然失效（且 kb/topics 下是 .dita，不是 .md）。
  # 改成仓库根相对路径指向 DITA 正本：拿到仓库的人能直接定位，没仓库的人也看得出
  # 指向何处。绝不留会解析失败的 `../`。
  sed -E 's|\]\((\.\./)+topics/([^)]*)\.md\)|](kb/topics/\2.dita)|g' "$built" > "out/$tool.md"

  if grep -q '](\.\./' "out/$tool.md"; then
    echo "out/$tool.md 里仍有相对路径链接，改写规则没覆盖到：" >&2
    grep -o '](\.\.[^)]*)' "out/$tool.md" | sort -u >&2
    exit 1
  fi

  echo "built out/$tool.md（源：${built#out/$tool/}）"
done
echo "改一处源重跑本脚本，所有工具变体同步更新——这是单源多变体的兑现。"

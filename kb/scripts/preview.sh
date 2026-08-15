#!/usr/bin/env bash
# DITA 预览：构建 html5 + 起本地 HTTP server + 开浏览器。
# 用 HTTP（非 file://）避免 CORS/相对路径边缘坑，跟部署一致。
# 输入默认全库；传领域 map 只构建该域（库大时更快，域内 xref 仍可跳）。
#   ./scripts/preview.sh                        # 全库
#   ./scripts/preview.sh maps/domains/ai.ditamap  # 只看 ai 域
# 改了源，重跑本脚本刷新（准实时 live-reload 见 README，需额外 watch 工具）。
set -euo pipefail
cd "$(dirname "$0")/.."

INPUT="${1:-maps/root.ditamap}"
OUT=/tmp/kb-preview
PORT=8080

echo "构建 $INPUT → html5 ..."
dita -f html5 -i "$INPUT" -o "$OUT" \
  --nav-toc=full \
  --args.css=preview.css \
  --args.cssroot="$(pwd)/scripts" \
  --args.copycss=yes \
  >/dev/null
echo "  完成。"

cd "$OUT"
python3 -m http.server "$PORT" >/dev/null 2>&1 &
SRV=$!
trap 'kill $SRV 2>/dev/null' EXIT
sleep 1

URL="http://localhost:$PORT/index.html"
echo "预览：$URL"
echo "（改了源重跑本脚本刷新；Ctrl-C 停止 server）"
open "$URL" 2>/dev/null || echo "手动在浏览器打开：$URL"
wait $SRV

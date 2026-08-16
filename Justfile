# dita monorepo 统一入口。能力地图与工具生态终态见 docs/architecture.md §四。
# 过渡期里有的命令是脚本、有的是 dita-tools——用哪条不必关心，都从这里走。

default:
    @just --list

# kb 治理全套：结构校验 + 业务规则 R1–R10 + 术语扫描（入库前跑；维度覆盖看 just ia）
review:
    cd kb && sh scripts/review.sh

# IA 治理骨架：哪空、在做、算完（--details 展开细表，--depth N 限层）
# 先现编再跑——保证看到的永远是当前代码的结果，不是上次 install 的旧二进制
ia *args:
    cd dita-tools && cargo build -q
    cd kb && ../dita-tools/target/debug/dita-tools ia {{args}}

# 单源 → 各工具变体（CLAUDE.md / AGENTS.md 雏形）
build:
    cd kb && sh scripts/build-agent-rules.sh

# html5 预览：构建 + 起本地 server + 开浏览器（默认全库，传 map 只构建该域）
# just preview                              # 全库
# just preview maps/domains/ai.ditamap     # 只看 ai 域
preview map="maps/root.ditamap":
    cd kb && sh scripts/preview.sh {{map}}

# 外链活性（联网、慢；定期跑，不并入 review）
links:
    uv run --script kb/scripts/link-check.py
    uv run --script kb/scripts/link-check.py research

# Rust 平台：测试 / lint / 重装二进制（改了 Rust 代码后必须 install，否则 PATH 上是旧的）
test:
    cd dita-tools && cargo test --workspace

clippy:
    cd dita-tools && cargo clippy --workspace --all-targets

fmt:
    cd dita-tools && cargo fmt --all -- --check

install:
    cd dita-tools && cargo install --path apps/dita_cli

# 从零装齐全部依赖（幂等；版本 SSOT 在脚本顶部）
setup:
    sh scripts/setup-env.sh

# 提交前全套（不含 links——它依赖网络）
check: review test clippy fmt

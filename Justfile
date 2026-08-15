# dita monorepo 统一入口。能力地图与工具生态终态见 docs/architecture.md §四。
# 过渡期里有的命令是脚本、有的是 dita-tools——用哪条不必关心，都从这里走。

default:
    @just --list

# kb 治理全套：结构校验 + 业务规则 R1–R10 + 覆盖度 + 术语扫描（入库前跑）
review:
    cd kb && sh scripts/review.sh

# IA 治理骨架：哪空、在做、算完（--details 展开细表，--depth N 限层）
ia *args:
    cd kb && dita-tools ia {{args}}

# 单源 → 各工具变体（CLAUDE.md / AGENTS.md 雏形）
build:
    cd kb && sh scripts/build-agent-rules.sh

# 外链活性（联网、慢；定期跑，不并入 review）
links:
    python3 kb/scripts/link-check.py
    python3 kb/scripts/link-check.py research

# Rust 平台：测试 / lint / 重装二进制（改了 Rust 代码后必须 install，否则 PATH 上是旧的）
test:
    cd dita-tools && cargo test --workspace

clippy:
    cd dita-tools && cargo clippy --workspace --all-targets

install:
    cd dita-tools && cargo install --path apps/dita_cli

# 提交前全套（不含 links——它依赖网络）
check: review test clippy

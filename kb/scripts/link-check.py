#!/usr/bin/env python3
"""链接活性检查：提取外链，批量查活性，报失效。

来源纪律的机器化配套（见 dita2 cases/知识体系重塑/来源与时效-设计.md）：
R8 只保证"有来源"，本脚本保证"来源还在"。源没了是内容过时的硬信号之一。

**不并入 review.sh**：外链查活性要联网、慢、且受网络波动影响，卡在每次入库前不合适
（来源与时效-设计 待定项 3 的建议）。定期单独跑，或接 CI 的周期任务。

扫描对象：`scope="external"` 的 href（正文 xref 与 prolog source 统一用它标，
见来源与时效-设计"所有外部来源统一用 scope=external"），以及 .md 文件里的裸 URL。

用法：
    python3 scripts/link-check.py                # 默认扫 kb 的 topics/ maps/ vocab/
    python3 scripts/link-check.py ../docs        # 也可扫别的目录（.md 一并扫）
退出码：有失效链接则非零。
"""
import concurrent.futures
import os
import re
import sys
import urllib.error
import urllib.request

TIMEOUT = 20
WORKERS = 8
# HTTP 头只能 latin-1 编码，UA 里放中文会让每个请求都抛 UnicodeEncodeError
UA = "dita-tools-link-check/1.0 (+kb source liveness check)"

# scope="external" 与 href 的相对顺序不固定，两种写法都要认
HREF_IN_DITA = re.compile(
    r'<[^>]*?(?:href="(?P<a>https?://[^"]+)"[^>]*?scope="external"'
    r'|scope="external"[^>]*?href="(?P<b>https?://[^"]+)")',
    re.S,
)
URL_IN_MD = re.compile(r'https?://[^\s<>()\[\]"\']+')
# markdown 里的代码块与行内代码不是引用，是示例：命名空间 URI（本就不必可解析）、
# 命令行示例里的占位 URL 都在其中。把它们当外链查，报出来的全是误报。
FENCED_CODE = re.compile(r'^```.*?^```', re.M | re.S)
INLINE_CODE = re.compile(r'`[^`\n]*`')
# 形如 https://.../x 的占位符：省略号（… 或 ...）或尖括号即非真实地址
PLACEHOLDER = re.compile(r'(\.\.\.|…|<|>)')


def collect(paths):
    """→ {url: [(相对路径, 行号), ...]}，同一 URL 多处引用只查一次。"""
    found = {}
    for root_dir in paths:
        for dirpath, _, filenames in os.walk(root_dir):
            for name in sorted(filenames):
                if not name.endswith(('.dita', '.ditamap', '.md')):
                    continue
                path = os.path.join(dirpath, name)
                try:
                    text = open(path, encoding='utf-8').read()
                except OSError:
                    continue
                if name.endswith('.md'):
                    # 代码块整体挖空但保留换行，行号才不会错位
                    scanned = FENCED_CODE.sub(lambda m: '\n' * m.group(0).count('\n'), text)
                    scanned = INLINE_CODE.sub(lambda m: ' ' * len(m.group(0)), scanned)
                    urls = [
                        (m.group(0).rstrip('.,;:'), m.start())
                        for m in URL_IN_MD.finditer(scanned)
                        if not PLACEHOLDER.search(m.group(0))
                    ]
                else:
                    urls = [((m.group('a') or m.group('b')), m.start()) for m in HREF_IN_DITA.finditer(text)]
                for url, offset in urls:
                    line = text.count('\n', 0, offset) + 1
                    found.setdefault(url, []).append((path, line))
    return found


def check(url):
    """→ (url, 状态码或错误文本)。HEAD 被拒时退回 GET——不少站点不接受 HEAD。"""
    for method in ('HEAD', 'GET'):
        req = urllib.request.Request(url, method=method, headers={'User-Agent': UA})
        try:
            with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
                return url, resp.status
        except urllib.error.HTTPError as e:
            if method == 'HEAD' and e.code in (403, 405, 501):
                continue          # 该站不接受 HEAD，换 GET 再试
            return url, e.code
        except Exception as e:     # noqa: BLE001 —— 超时/DNS/证书都归"查不到"
            if method == 'GET':
                return url, type(e).__name__
    return url, 'unknown'


def main():
    kb = os.path.join(os.path.dirname(__file__), '..')
    paths = sys.argv[1:] or [os.path.join(kb, d) for d in ('topics', 'maps', 'vocab')]
    links = collect(paths)
    if not links:
        print('没有找到外链。')
        return 0

    print(f'检查 {len(links)} 个外链（{WORKERS} 并发，超时 {TIMEOUT}s）…\n')
    dead = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as pool:
        for url, status in pool.map(check, links):
            if status != 200:
                dead.append((url, status))

    for url, status in sorted(dead, key=lambda x: str(x[1])):
        print(f'❌ {status}  {url}')
        for path, line in links[url]:
            print(f'      {os.path.relpath(path, kb)}:{line}')

    ok = len(links) - len(dead)
    print(f'\n{ok}/{len(links)} 可达', end='')
    if dead:
        print(f'，{len(dead)} 个失效——来源没了通常意味着该复核这篇内容')
        return 1
    print('，全部外链存活')
    return 0


if __name__ == '__main__':
    sys.exit(main())

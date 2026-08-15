#!/usr/bin/env python3
"""术语规整（报告版）：扫正文里术语库已收录术语的"字面出现"（未用 keyref），
报告建议改成 <term keyref="...">。

起步只报告、不自动改（机器兜底-设计 决策2：先报告、信得过再开自动替换）。
读 topics/glossary/term-*.dita 的 glossentry，建"叫法→term key"表（含中文首选、
中英别名 glossSynonym、缩写 glossAcronym）；再扫其余 topic 的正文文本（跳过已在
<term> 内的），命中即报。

缩写等短 ASCII 词易误报，报告里标注（缩写｜注意误报），交人判断。
用法：python3 scripts/term-normalize.py [topics目录]
"""
import sys, glob, os, re
import xml.etree.ElementTree as ET


def build_alias_map(gloss_dir):
    aliases = {}   # 叫法原文 -> term key
    for path in sorted(glob.glob(os.path.join(gloss_dir, 'term-*.dita'))):
        try:
            root = ET.parse(path).getroot()
        except ET.ParseError:
            continue
        if root.tag != 'glossentry':
            continue
        key = root.get('id')
        names = []
        gt = root.find('glossterm')
        if gt is not None and gt.text:
            names.append(gt.text.strip())
        for tag in ('glossSynonym', 'glossAcronym', 'glossSymbol'):
            for e in root.iter(tag):
                if e.text and e.text.strip():
                    names.append(e.text.strip())
        for n in names:
            if n and n not in aliases:
                aliases[n] = key
    return aliases


# 这些元素内的文本是"标记语境"而非行文：路径、代码、元素/属性名、命令、链接文字。
# 术语 keyref 属于行文，塞进这些地方是错的（<filepath>.codex/</filepath> 不该改成术语引用，
# 引文标题 <xref>Claude Code, ...</xref> 也不该）。扫描时整体跳过。
SKIP_TAGS = {
    'term', 'keyword',
    # 标题是标签不是行文：keyref 会改变导航/TOC 的渲染，而"这篇讲的就是它"本身已由
    # 标题表达，加引用的查询价值为零。
    'title', 'navtitle',
    'filepath', 'codeph', 'codeblock', 'cmdname', 'varname', 'userinput',
    'systemoutput', 'msgph', 'msgnum', 'apiname', 'option', 'parmname', 'synph',
    'xmlelement', 'xmlatt', 'xmlnsname', 'xmlpi', 'markupname',
    'uicontrol', 'wintitle', 'menucascade', 'shortcut',
    'xref', 'linktext', 'draft-comment',
}


def text_outside_terms(root):
    """收集行文文本片段——跳过 SKIP_TAGS 及其子树（标记语境不是行文）。"""
    chunks = []

    def walk(el, skipping):
        sk = skipping or (el.tag in SKIP_TAGS)
        if el.text and not sk:
            chunks.append(el.text)
        for c in el:
            walk(c, sk)
            # tail 属于父的行文，跳过与否取决于父（即当前 skipping），不取决于 c
            if c.tail and not skipping:
                chunks.append(c.tail)
    walk(root, False)
    return chunks


def is_ascii(s):
    return all(ord(c) < 128 for c in s)


def hits_in(chunk, alias):
    if is_ascii(alias):
        # 含大写字母的 ASCII 叫法按专有名词处理，区分大小写——否则 "Codex" 会命中
        # 路径/包名里的 codex，"Go" 会命中 go 等。全小写叫法（kebab-case）才忽略大小写。
        flags = 0 if any(c.isupper() for c in alias) else re.I
        return re.search(r'\b' + re.escape(alias) + r'\b', chunk, flags) is not None
    return alias in chunk


def main():
    kb = os.path.join(os.path.dirname(__file__), '..')
    topics = sys.argv[1] if len(sys.argv) > 1 else os.path.join(kb, 'topics')
    gloss_dir = os.path.join(kb, 'topics', 'glossary')
    aliases = build_alias_map(gloss_dir)
    if not aliases:
        print("术语库为空，无可匹配。")
        return

    found = 0
    for path in sorted(glob.glob(os.path.join(topics, '**', '*.dita'), recursive=True)):
        if os.path.normpath(gloss_dir) in os.path.normpath(path):
            continue  # 跳过术语库自身
        try:
            root = ET.parse(path).getroot()
        except ET.ParseError:
            continue
        chunks = text_outside_terms(root)
        rel = os.path.relpath(path, kb)
        # 每篇首现标注即可：本篇已 keyref 过的术语，后文字面提及不再报——
        # 否则一段 20 行的表格里每次提及都要包一层，报告很快沦为噪音而无人读。
        already = {t.get('keyref') for t in root.iter('term') if t.get('keyref')}
        seen = set()
        for alias, key in aliases.items():
            if alias in seen or key in already:
                continue
            for ch in chunks:
                if hits_in(ch, alias):
                    note = '｜缩写,注意误报' if (is_ascii(alias) and len(alias) <= 4 and alias.isupper()) else ''
                    print(f'{rel}: "{alias}" → 建议 <term keyref="{key}">{note}')
                    seen.add(alias)
                    found += 1
                    break
    if found == 0:
        print("没有发现术语的裸字面出现（正文要么没用这些术语，要么已 keyref）。")
    else:
        print(f"\n共 {found} 处建议（报告版，未自动改；确认后可人工或开自动替换）。")


if __name__ == '__main__':
    main()

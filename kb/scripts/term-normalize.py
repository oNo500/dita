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


def text_outside_terms(root):
    """收集不在 <term> 元素内的文本片段。"""
    chunks = []

    def walk(el, in_term):
        it = in_term or (el.tag == 'term')
        if el.text and not it:
            chunks.append(el.text)
        for c in el:
            walk(c, it)
            if c.tail:            # tail 属于父，不在 c 内部
                chunks.append(c.tail)
    walk(root, False)
    return chunks


def is_ascii(s):
    return all(ord(c) < 128 for c in s)


def hits_in(chunk, alias):
    if is_ascii(alias):
        return re.search(r'\b' + re.escape(alias) + r'\b', chunk, re.I) is not None
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
        seen = set()
        for alias, key in aliases.items():
            if alias in seen:
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

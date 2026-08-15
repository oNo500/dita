#!/usr/bin/env python3
"""维度覆盖度报告：扫 kb/topics 下的 DITA 文件，按域算维度覆盖度、列盲区。

机制（见 vocab/subjectScheme.ditamap 维度值集注释、dita2 维度对标报告）：
- 每个 topic 用 <data name="domain" value="..."/> 标所属域。
- 领域全景 topic 用 <data name="planned-dimension" value="dim-x"/> 声明本域规划的完整维度清单。
- 内容 topic 用根属性 @dimension="dim-a dim-b"（专门化自 @props，可多值）声明覆盖的维度。

覆盖度 = |已覆盖 ∩ 规划| / |规划|；盲区 = 规划 - 已覆盖。
只统计"规划内"的覆盖：内容标了规划外的维度会单列提示（可能该补进全景，或标错）。
"""
import sys, glob, os
import xml.etree.ElementTree as ET


def scan(topics_dir):
    planned = {}   # domain -> set(dim)  规划维度（来自全景 planned-dimension）
    covered = {}   # domain -> set(dim)  实覆盖维度（来自内容 topic @dimension）
    for path in sorted(glob.glob(os.path.join(topics_dir, '**', '*.dita'), recursive=True)):
        try:
            root = ET.parse(path).getroot()
        except ET.ParseError as e:
            print(f"跳过（解析失败）{path}: {e}", file=sys.stderr)
            continue
        domain = None
        planned_dims = set()
        for data in root.iter('data'):
            name = data.get('name')
            if name == 'domain':
                domain = data.get('value')
            elif name == 'planned-dimension':
                planned_dims.add(data.get('value'))
        if domain is None:
            continue  # 未标域，不参与覆盖度统计
        dim_attr = root.get('dimension')
        covered_dims = set(dim_attr.split()) if dim_attr else set()
        if planned_dims:
            planned.setdefault(domain, set()).update(planned_dims)
        if covered_dims:
            covered.setdefault(domain, set()).update(covered_dims)
    return planned, covered


def report(planned, covered):
    domains = sorted(planned)
    if not domains:
        print("没有找到声明了 planned-dimension 的全景 topic。")
        return
    for d in domains:
        p = planned[d]
        cov_all = covered.get(d, set())
        cov_in = cov_all & p            # 规划内的覆盖
        blind = sorted(p - cov_in)      # 盲区
        extra = sorted(cov_all - p)     # 覆盖了规划外的维度
        pct = len(cov_in) * 100 // len(p) if p else 0
        print(f"\n域 {d}：覆盖度 {len(cov_in)}/{len(p)}（{pct}%）")
        if cov_in:
            print(f"  已覆盖：{' '.join(sorted(cov_in))}")
        if blind:
            print(f"  盲区（{len(blind)}）：{' '.join(blind)}")
        if extra:
            print(f"  ⚠ 规划外的覆盖（该补进全景或标错了）：{' '.join(extra)}")


if __name__ == '__main__':
    base = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.path.dirname(__file__), '..', 'topics')
    planned, covered = scan(base)
    report(planned, covered)

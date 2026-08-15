<?xml version="1.0" encoding="UTF-8"?>
<!-- KB 业务规则（Schematron，ISO/IEC 19757-3）R1–R10。
     分工：RNG 管结构、subjectScheme enumerationdef 构建期管受控值；
     本文件管 RNG 表达不了的跨属性/语义/业务规则（编辑期即时 + 审查脚本批量）。
     DITA topic 无命名空间，context 直接用元素名。写法用 XPath1 子集，便于自带处理器执行。
     决策依据：dita2 cases/知识体系重塑/schematron-设计.md（R1–R10 已定案）。 -->
<schema xmlns="http://purl.oclc.org/dsdl/schematron">
  <title>KB 业务规则 R1-R10</title>

  <pattern id="R1-shortdesc">
    <rule context="concept | reference | task | troubleshooting">
      <assert test="shortdesc and normalize-space(shortdesc) != ''"
        >R1（error）：topic 必须有非空 shortdesc（检索摘要与 LLM 友好都依赖它）。</assert>
    </rule>
  </pattern>

  <pattern id="R2-volatility-required">
    <rule context="concept | reference | task | troubleshooting | glossentry">
      <assert test="@volatility"
        >R2（error）：topic 必须显式标 @volatility（词表故意不设默认，漏标即报错）。</assert>
    </rule>
  </pattern>

  <pattern id="R3-verified-needs-reviewed">
    <rule context="*[@volatility='volatile'][@maturity='verified']">
      <assert test="prolog/data[@name='reviewed'] or .//data[@name='reviewed']"
        >R3（error）：volatile 且 verified 的内容必须有核对日期（prolog data name="reviewed"）。</assert>
    </rule>
  </pattern>

  <pattern id="R4-maturity-values">
    <rule context="*[@maturity]">
      <assert test="@maturity='draft' or @maturity='curated' or @maturity='verified'"
        >R4（error）：@maturity 只能是 draft/curated/verified。</assert>
    </rule>
  </pattern>

  <pattern id="R5-volatility-values">
    <rule context="*[@volatility]">
      <assert test="@volatility='stable' or @volatility='volatile'"
        >R5（error）：@volatility 只能是 stable/volatile。</assert>
    </rule>
  </pattern>

  <pattern id="R6-tool-values">
    <rule context="*[@tool]">
      <assert test="@tool='tool-claude-code' or @tool='tool-codex' or @tool='tool-antigravity'"
        >R6（error）：@tool 只能是 tool-claude-code/tool-codex/tool-antigravity（多值场景由审查脚本的 XSLT2 版补校）。</assert>
    </rule>
  </pattern>

  <pattern id="R7-bare-term">
    <rule context="term[not(@keyref)]">
      <report test="true()" role="warning"
        >R7（warning）：裸 term 未用 keyref。术语库建成后应引 keyref，输出才统一（术语首现判定由 term-normalize 脚本承担，此处仅提示）。</report>
    </rule>
  </pattern>

  <pattern id="R8-source-required">
    <rule context="concept | reference">
      <assert test="prolog/source or .//xref[@scope='external'] or .//data[@name='source']"
        >R8（error）：concept/reference 必须有至少一个来源（prolog source 或来源节外链）。内容本就该有出处。</assert>
    </rule>
  </pattern>

  <!-- R9（领域 map 必须挂一个 tech-landscape 全景 topic）是 map 层 + 跨文档规则，
       单文档 Schematron 判不了（要解析 topicref 指向的 topic 的 outputclass）。
       归构建脚本承担（dimension-coverage.py 可扩展：某 domain 有内容却无全景即报）。此处不实现。 -->

  <pattern id="R10-quickstart-xref">
    <rule context="*[@outputclass='quickstart']">
      <assert test=".//xref"
        >R10（error）：quickstart 必须 xref 到所属领域全景（并声明覆盖/略过哪些维度）。</assert>
    </rule>
  </pattern>

</schema>

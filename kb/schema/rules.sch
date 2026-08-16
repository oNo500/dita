<?xml version="1.0" encoding="UTF-8"?>
<!-- KB 业务规则（Schematron，ISO/IEC 19757-3）R1–R10。
     分工：RNG 管结构、subjectScheme enumerationdef 构建期管受控值；
     本文件管 RNG 表达不了的跨属性/语义/业务规则（编辑期即时 + 审查脚本批量）。
     R12–R15（2026-08-16 加，随 writing-style 落地）由 dita-tools lint 实现（新能力直接进平台，
     不再扩 check-rules.xsl——它在吸收退役通道上）；严重度按 maturity 分级：
     draft 记 warning（草稿免罚），curated/verified 记 error（晋级门）。
     DITA topic 无命名空间，context 直接用元素名。写法用 XPath1 子集，便于自带处理器执行。
     决策依据：dita2 cases/知识体系重塑/schematron-设计.md（R1–R10 已定案）。 -->
<schema xmlns="http://purl.oclc.org/dsdl/schematron">
  <title>KB 业务规则 R1-R16</title>

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


  <!-- ── R12–R15：题材与文体（实现在 dita-tools lint，此处为规格正本）── -->

  <pattern id="R12-genre">
    <rule context="concept | task">
      <assert test="@outputclass"
        >R12：concept/task 必须标题材 @outputclass（固定结构靠它；词表 genre-values 为受控值）。</assert>
    </rule>
    <rule context="*[@outputclass]">
      <assert test="true()"
        >R12：@outputclass 值必须在词表 genre-values 内，且其 dita-type 须与根元素匹配
        （cheatsheet 只能标在 reference 上，best-practice 只能标在 concept 上）。值集校验由 lint 读词表执行。</assert>
    </rule>
  </pattern>

  <pattern id="R13-genre-structure">
    <rule context="*[@outputclass]">
      <assert test="true()"
        >R13：题材声明了 required-section 的（best-practice、quickstart），正文各节标题须按前缀
        覆盖全部必需节（"做法：四条"匹配"做法"）。骨架缺节是缺陷；节内朴素不是。由 lint 执行。</assert>
    </rule>
  </pattern>

  <pattern id="R14-source-section">
    <rule context="section[title='来源']">
      <assert test=".//b[normalize-space()='事实'] and .//b[normalize-space()='判断']"
        >R14：来源节固定两段标签「事实」「判断」（可空不可省）；旧标签「已核对」应改；
        正文不写核对日期（日期唯一存放处是 prolog data name="reviewed"）。</assert>
    </rule>
  </pattern>

  <pattern id="R15-plain-register">
    <rule context="conbody | refbody | taskbody">
      <assert test="true()"
        >R15（代理指标，恒 warning）：粗体每节至多 2 处；破折号插入语每段至多 1 处；
        程度词（特别/极其/恰恰/真正的/最危险）与口语词（凑合/挂个/塞进/出事/拦住/就该）不出现；
        标题不含问号、"，不是"论断句式、悬念破折号（writing-style 规则七的机器面）。
        只是代理——格言句、场景化开头、定位词与类目对齐仍靠人审。由 lint 执行。</assert>
    </rule>
  </pattern>


  <pattern id="R16-split-threshold">
    <rule context="concept">
      <assert test="true()"
        >R16（2026-08-16 定，硬计数）：concept 正文里实现层行内标记
        （codeph/cmdname/apiname/parmname/varname/option/userinput/systemoutput/synph/codeblock）
        合计不得超过 8 处——超过即说明判据与配置并存，按 phase3-review 约定 3 拆成
        concept（判据）＋ reference（配置）。xmlelement/xmlatt/filepath 不计
        （前两者是本库讲 DITA 写作时的主题词汇，后者多为示意）。由 dita-tools lint 执行。</assert>
    </rule>
  </pattern>

</schema>

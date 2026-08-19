<?xml version="1.0" encoding="UTF-8"?>
<!-- KB 业务规则（Schematron，ISO/IEC 19757-3）R1–R20。
     分工：RNG 管结构、subjectScheme enumerationdef 构建期管受控值；
     本文件管 RNG 表达不了的跨属性/语义/业务规则（编辑期即时 + 审查脚本批量）。
     R12–R16（2026-08-16 加，随 writing-style 落地）与 R18–R20 由 dita-tools lint 实现
     （新能力直接进平台，不再扩 check-rules.xsl——它正在被逐步取代）；严重度按 maturity 分级：
     draft 记 warning（草稿不阻断），curated/verified 记 error（晋级门）。R18 是例外，恒 error
     （被检查的正是分级所依赖的属性）；R19 随分级，但索引读不到时走"未执行"而非通过。
     DITA topic 无命名空间，context 直接用元素名。写法用 XPath1 子集，便于自带处理器执行。
     决策依据：dita2 cases/知识体系重塑/schematron-设计.md（R1–R10 已定案）。
     归属标注（2026-08-16，Task 13b 规则归并）：每条 R 前一行注释写明它是哪个学科正本的
     机器面。规则的人读正本在 topics/content-engineering/，本文件只管机器执行——
     两处内容不重复，改规则先改正本，再看这条 R 是否要跟着调。 -->
<schema xmlns="http://purl.oclc.org/dsdl/schematron">
  <title>KB 业务规则 R1-R20</title>

  <!-- 归属：LLM 友好与检索 / writing-llm-friendly -->
  <pattern id="R1-shortdesc">
    <rule context="concept | reference | task | troubleshooting">
      <assert test="shortdesc and normalize-space(shortdesc) != ''"
        >R1（error）：topic 必须有非空 shortdesc（检索摘要与 LLM 友好都依赖它）。</assert>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <pattern id="R2-volatility-required">
    <rule context="concept | reference | task | troubleshooting | glossentry">
      <assert test="@volatility"
        >R2（error）：topic 必须显式标 @volatility（词表故意不设默认，漏标即报错）。</assert>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <pattern id="R3-verified-needs-reviewed">
    <rule context="*[@volatility='volatile'][@maturity='verified']">
      <assert test="prolog/data[@name='reviewed'] or .//data[@name='reviewed']"
        >R3（error）：volatile 且 verified 的内容必须有核对日期（prolog data name="reviewed"）。</assert>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <pattern id="R4-maturity-values">
    <rule context="*[@maturity]">
      <assert test="@maturity='draft' or @maturity='curated' or @maturity='verified'"
        >R4（error）：@maturity 只能是 draft/curated/verified。</assert>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <pattern id="R5-volatility-values">
    <rule context="*[@volatility]">
      <assert test="@volatility='stable' or @volatility='volatile'"
        >R5（error）：@volatility 只能是 stable/volatile。</assert>
    </rule>
  </pattern>

  <!-- 归属：交付物变体过滤 / 词表 tool-values（无人读正本） -->
  <pattern id="R6-tool-values">
    <rule context="*[@tool]">
      <assert test="@tool='tool-claude-code' or @tool='tool-codex' or @tool='tool-antigravity'"
        >R6（error）：@tool 只能是 tool-claude-code/tool-codex/tool-antigravity（多值场景由审查脚本的 XSLT2 版补校）。</assert>
    </rule>
  </pattern>

  <!-- 归属：术语治理 / terminology-rules -->
  <pattern id="R7-bare-term">
    <rule context="term[not(@keyref)]">
      <report test="true()" role="warning"
        >R7（warning）：裸 term 未用 keyref。术语库建成后应引 keyref，输出才统一（术语首现判定由 term-normalize 脚本承担，此处仅提示）。</report>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <pattern id="R8-source-required">
    <rule context="concept | reference">
      <assert test="prolog/source or .//xref[@scope='external'] or .//data[@name='source']"
        >R8（error）：concept/reference 必须有至少一个来源（prolog source 或来源节外链）。内容应有出处。</assert>
    </rule>
  </pattern>

  <!-- 归属：领域维度框架 / domain-dimension-method -->
  <!-- R9（领域 map 必须挂一个 tech-landscape 概览 topic）是 map 层 + 跨文档规则，
       单文档 Schematron 判不了（要解析 topicref 指向的 topic 的 outputclass）。
       归构建脚本承担（dimension-coverage.py 可扩展：某 domain 有内容却无概览即报）。此处不实现。 -->

  <!-- 归属：领域维度框架 / domain-dimension-method -->
  <!-- R10（quickstart 必须 xref 到所属领域概览并声明覆盖/略过哪些维度）已被 R20 吸收，
       此处不再实现，check-rules.xsl 里的那一段同步删除（吸收纪律：一项能力只落一处，
       两处都报会让同一处缺失出现两条消息）。原因是单文档 Schematron 只判得了「有没有
       xref」——任意一条链接都能满足它，而规则真正要的是「那个 xref 指到的是不是本域的
       概览」，这要打开被引文件读它的 @outputclass 与 domain，跨文档，XPath1 子集表达不了。
       差分对账：R10 的判定是 R20 挂靠一面的严格弱化（R20 通过必然 R10 通过，反之不然），
       故吸收不放宽任何一处，严重度也照 R10 保持恒 error。 -->


  <!-- ── R12–R15：体裁与文体（实现在 dita-tools lint，此处为规格正本）── -->

  <!-- 归属：内容类型化 / writing-typing -->
  <pattern id="R12-genre">
    <rule context="concept | task">
      <assert test="@outputclass"
        >R12：concept/task 必须标体裁 @outputclass（固定结构靠它；词表 genre-values 为受控值）。</assert>
    </rule>
    <rule context="*[@outputclass]">
      <assert test="true()"
        >R12：@outputclass 值必须在词表 genre-values 内，且其 dita-type 须与根元素匹配
        （cheatsheet 只能标在 reference 上，best-practice 只能标在 concept 上）。值集校验由 lint 读词表执行。</assert>
    </rule>
  </pattern>

  <!-- 归属：内容类型化 / writing-typing（必需节清单正本在词表 genre-values） -->
  <pattern id="R13-genre-structure">
    <rule context="*[@outputclass]">
      <assert test="true()"
        >R13：体裁声明了 required-section 的（best-practice、quickstart），正文各节标题须按前缀
        覆盖全部必需节（"做法：四条"匹配"做法"）。骨架缺节是缺陷；节内朴素不是。由 lint 执行。</assert>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <!-- 形式一项刻意不加检查（2026-08-19，随全库来源节条目化）：事实段现在写成 ul、
       一条一行「支撑正文哪部分——地址」，正本在 writing-sourcing。能想到的机器代理只有
       「来源节里有 external xref 就必须有 ul」，而它对事实段为「无」的篇必然误报——
       那几篇正文全是本库判断，来源节里的外链是「该源不承担本篇任何断言」的反向声明
       （naming-rules、rot-detection、writing-style、RAG 三篇、engineering-ci）。
       规则真正要的「每条说明它支撑正文哪一部分」更判不了：条目怎么切分是编辑判断。
       与 R14 穷尽性一项同一处置——判不准的代理会让整条规则被忽略，靠人审接住。 -->
  <pattern id="R14-source-section">
    <rule context="section[title='来源']">
      <assert test=".//b[normalize-space()='事实'] and .//b[normalize-space()='判断']"
        >R14：来源节固定两段标签「事实」「判断」（可空不可省）；旧标签「已核对」应改；
        正文不写核对日期（日期唯一存放处是 prolog data name="reviewed"）。</assert>
    </rule>
  </pattern>

  <!-- 归属：写作文体 / writing-style；其中标题一项归 naming-rules -->
  <pattern id="R15-plain-register">
    <rule context="conbody | refbody | taskbody | shortdesc">
      <assert test="true()"
        >R15（代理指标，严重度随 maturity 分级）：粗体每节至多 2 处；破折号插入语每段至多 1 处；
        程度词（特别/极其/恰恰/真正的/最危险）与口语词不出现。口语词的机器面覆盖 writing-style
        规则四的全部禁词，取法分两种：跑 按单字扫（技术文里无合法复合词，跑道/奔跑一类穷举得尽，
        作排除表）；拦/装/挂/塞 的合法复合词开放（安装、挂载、阻塞、拦截，随时可能新增），
        故只取可判定的口语搭配（拦住/拦下、装上/装进/装个、挂个/挂上、塞进/塞满），
        另加 凑合/就该/来点/出事。每个词各带一份合法复合词排除表，命中落在表内不计
        （安装上游、封装进、产出事实、带来点滴）。这一取法宁可漏报不误报：未列出的口语搭配
        由人审接住。单字「个」不进机器面——它是普通量词，机器判不了。
        标题不含问号、"，不是"论断句式、破折号与全角冒号：这一项是 naming-rules 标题规则的机器面，
        原 writing-style 规则七已整条迁入该篇。冒号一项直接禁、不设启发式——"冒号前是否已能
        独立指认该节点"这半句机器判不了，判不准的启发式一旦误报，整条规则会被忽略；
        冒号后的枚举与说明写进 shortdesc 与正文首句。只扫全角「：」：半角冒号出现在标识符
        内部（xml:lang、URL scheme），不是副标题串接，扫它会误伤命名规则要求保留机制名的那类
        标题；半角写法的副标题由人审接住，与口语词同一取法（宁可漏报不误报）。
        检查范围含 shortdesc：它会被抽出去复用（链接预览、目录条目说明、检索摘要），
        一篇被十处链接则其 shortdesc 在十处露面，口语跟着链接扩散全库，故不能只扫正文。
        落到 shortdesc 上的子检查逐项定：口语词与程度词照搬（提及排除同样生效）；
        破折号照搬「一段至多 1」——shortdesc 恰是单段；粗体在 shortdesc 上为 0，
        因为「每节至多 2」按节量化而 shortdesc 不是节，搬得过来的是「只标判据与警示」，
        摘要无判据与警示可标；标题模式一项查的是 title，不落到 shortdesc。
        只是代理——格言句、场景化开头、定位词与类目对齐仍靠人审。由 lint 执行。</assert>
    </rule>
  </pattern>


  <!-- 归属：切分与准入 / writing-atomicity -->
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

  <!-- 归属：命名与归属 / naming-rules -->
  <pattern id="R17-domain-registered">
    <rule context="*[prolog/data[@name='domain']]">
      <assert test="true()"
        >R17（2026-08-16 定）：prolog data name="domain" 的 value 必须是 subjectScheme
        主题树里已注册的 subject key——domain 是全库唯一未受控的元数据字段（enumerationdef
        绑不了 data 元素，值域没有别处能管），已实际碎化出一篇一域的孤儿值。未注册的值报
        error，并提示在 subjectScheme 注册该键或改用已注册值；顺带反向报表：已注册但零
        topic 挂靠的 subject key（树的空叶子），归入 ia「需要处理」段。由 dita-tools ia 执行。</assert>
    </rule>
  </pattern>

  <!-- 归属：来源与成熟度 / writing-sourcing -->
  <pattern id="R18-maturity-required">
    <rule context="concept | reference | task | troubleshooting | glossentry">
      <assert test="@maturity"
        >R18（error，与 R2 对称，不按 maturity 分级）：内容 topic（含 glossentry）必须显式标
        @maturity。交付物成熟度门只是条件过滤（DITAVAL 排除 @maturity="draft"），没有「属性必须
        出现」这一档——不标 @maturity 的 topic 不匹配排除规则，未审内容会直接进交付物。词表里
        「未标注即视为 draft」只是校验与语义上的默认值，DITAVAL 看不到、不会当成排除条件。本条
        补的正是这一格：漏标本身即错误，不论该 topic 其余各项是否合规，因此不随 draft/curated
        分级——被检查的正是分级所依赖的那个属性缺席。由 dita-tools lint 执行（覆盖面含
        glossentry，与本条 Schematron 规格的 context 一致；R12–R16 的体裁/结构/文体检查不含
        glossentry，两者覆盖面不同、互不影响）。</assert>
    </rule>
  </pattern>

  <!-- 归属：命名与归属 / naming-rules -->
  <pattern id="R19-upstream-node">
    <rule context="*[prolog/data[@name='domain'][@value='dita']]">
      <assert test="prolog/data[@name='upstream-node']"
        >R19（2026-08-18 定，严重度随 maturity 分级：draft warning / curated 以上 error）：
        声明式溯源。每篇在 prolog 写 data name="upstream-node"，value 为该篇标题所依据的
        上游节点标题原文（英文逐字）；组合篇声明多条，逐条校验；上游确无对应节点的写
        value="coined"，并在文件头注释写明三道关（穷尽查证 / 先怀疑切分 / 只组合不发明）。
        不校验标题像不像上游节点名——本库标题是中文且常为组合，逐字匹配全是噪音（设计稿二）；
        校验的是三件确定的事：声明的节点是否真实存在、自造是否带说明、以及**上游改名后哪些
        声明失联**（第三件才是这套东西的目的）。
        比对前双方都归一化：大小写不敏感、首尾空白去除、内部连续空白折叠为一个，归一化后
        **精确**匹配，不做模糊或子串匹配——模糊会把 Specialization 匹到 Overview of
        specialization，制造比误报更隐蔽的假通过。
        解析不到时消息必须列三种可能而不是断言拼错：①拼写有误；②上游已改名或删除；
        ③索引未收录该节点（resource-only 子树、未随发行版发布的页面、conref 素材片段刻意
        排除在外），并提示核对索引头的生成日期与来源版本。把索引的空缺报成作者的错误，
        是最伤可信度的一类误报。
        索引（kb/vocab/upstream-nodes.tsv）缺失或读不了时本条**未执行**，走 skip 通道
        （lint 退出码 2、review.sh 报"未执行"），不得静默通过。
        覆盖面：目前只有 domain="dita"（其上游可本地解析，成本最低）。规则是全库通用的，
        推广到其他分支要补的只有抓取器与 benchmark-registry 的 index-source / index-generated
        两个字段，不改本条的判定逻辑（设计稿七之二）。由 dita-tools lint 执行。</assert>
    </rule>
  </pattern>

  <!-- 归属：领域维度框架 / domain-dimension-method -->
  <pattern id="R20-genre-hangoff">
    <rule context="*[@outputclass]">
      <assert test="true()"
        >R20（2026-08-18 定，恒 error，不按 maturity 分级）：体裁声明的挂靠与取舍。
        词表 genre-values 里带 data name="hangs-off-genre" 的体裁（目前只有 quickstart，
        值为 tech-landscape），其 topic 必须满足两件事，缺任一报错。
        其一，挂靠：正文里至少有一个 xref 指向的库内 .dita 文件，其根元素 @outputclass
        等于所声明的那个体裁，且该文件 prolog 的 domain 与本篇相同。判定要打开被引文件看，
        不是「有没有 xref」——那是被本条吸收的 R10 的判法，任意一条链接都能满足它。
        挂到别域的概览同样报错：取舍声明若对着一份无关的维度清单做，等于没做。
        其二，取舍声明：根元素必须标 @dimension，声明本篇覆盖了概览规划清单里的哪几个维度；
        所声明的值须全部落在该概览的 planned-dimension 内（落在外面说明要么本篇标错、
        要么概览漏登记，两种都会让覆盖度算不准）；且必须是**真子集**——覆盖了规划的全部维度
        就不是取舍，那篇是概览而不是概览上的一条路径。
        **略过的维度刻意不要求逐条声明**：它是规划清单与覆盖声明的差集，可由机器求出
        （dita-tools ia 已在算），手抄一份可推导的集合正是本库在别处逐条消灭的缺陷。
        作者要交的是选择与理由——选择落在 @dimension 上，理由落在「取舍」一节的散文里，
        而该节的**存在**由 R13 管（节名属体裁结构，正本在词表的 required-section，
        工具不得内联该字面），本条只管挂靠解析得到与覆盖声明成立与否，两条不重复报同一处缺失。
        恒 error 的两条理由：其一，被查的不是完成度而是体裁的定义性条件——不挂靠任何框架的
        quickstart 不是未写完的 quickstart，是标错体裁的 how-to；其二，被吸收的 R10 自首版
        起即为恒 error，吸收不得顺带放宽。由 dita-tools lint 执行。</assert>
    </rule>
  </pattern>

</schema>

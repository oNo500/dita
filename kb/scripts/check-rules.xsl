<?xml version="1.0" encoding="UTF-8"?>
<!-- KB 业务规则检查（可执行版，Saxon 跑）。实现 schema/rules.sch 的 R1–R8 与 R21
     （R10 于 2026-08-18 被 R20 吸收进 dita-tools lint，此处删除，见 rules.sch 该条注释）。
     R21（2026-08-19 加）本该按「新能力直接进平台」的惯例落 dita-tools lint，此处是刻意的例外：
     它的判定就是 R8 无来源态的否定，两处各实现一份必然漂移，故与 R8 同处共用一组变量。
     用 DITA-OT 自带 Saxon-HE 执行，不引第三方 Schematron 编译器。
     ⚠ 须与 schema/rules.sch 保持同步（那份是人读规格，本份是可执行实现）；
        将来接入 SchXslt 后可直接编译 rules.sch，删除本份消除重复。
     输出：每行一条违规 "Rx(error|warning): <msg>"；无输出＝该文件全过。
     R9（map 层跨文档）归 dimension-coverage.py。

     两个入口：
     - 命名模板 main（review.sh 用）：kb-dir 参数指到 kb 根，一次 Saxon 调用用
       uri-collection() 遍历 kb/topics 下全部 .dita，逐篇跑规则、逐行标 "[rules] <rel>: "
       前缀——文件名前缀不再由 review.sh 的 sed 拼，改这份自己出。每篇用 xsl:try 隔离，
       一篇解析失败（格式错的 XML）不拖垮整批：报 R0，结构性错误本身留给 review.sh 第 1
       节的 dita validate 去定位。
     - match="/"：单文件调试用（-s:<file> 直接跑），保留向后兼容，前缀留空。两条入口
       共用同一份规则实现（check-doc 命名模板），规则文本只写一处。 -->
<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
                xmlns:xs="http://www.w3.org/2001/XMLSchema"
                xmlns:err="http://www.w3.org/2005/xqt-errors">
  <xsl:output method="text"/>

  <xsl:variable name="maturities" select="('draft','curated','verified')"/>
  <xsl:variable name="volatilities" select="('stable','volatile')"/>
  <xsl:variable name="tools" select="('tool-claude-code','tool-codex','tool-antigravity')"/>

  <!-- 全局参数（非模板局部参数）：-it 初始模板调用时，Saxon 命令行的 name=value
       只绑定全局 xsl:param，绑不到命名模板自己的局部 xsl:param，踩过坑记在这。 -->
  <xsl:param name="kb-dir" as="xs:string?"/>

  <!-- 规则实现本体：对一份已解析文档执行 R1–R8 与 R21（R9/R10 除外，见上），每条违规一行，
       行首按调用方给的 $prefix 定位到具体文件（批量入口传 "[rules] <rel>: "，
       单文件调试入口传空串）。 -->
  <xsl:template name="check-doc">
    <xsl:param name="doc" as="document-node()"/>
    <xsl:param name="prefix" as="xs:string" select="''"/>
    <xsl:variable name="r" select="$doc/*"/>
    <xsl:variable name="hasShortdescType" select="boolean($r[self::concept or self::reference or self::task or self::troubleshooting])"/>
    <xsl:variable name="topicType" select="boolean($r[self::concept or self::reference or self::task or self::troubleshooting or self::glossentry])"/>

    <xsl:if test="$hasShortdescType and not($r/shortdesc[normalize-space(.)!=''])">
      <xsl:value-of select="concat($prefix,'R1(error): topic 必须有非空 shortdesc','&#10;')"/>
    </xsl:if>
    <xsl:if test="$topicType and not($r/@volatility)">
      <xsl:value-of select="concat($prefix,'R2(error): topic 必须显式标 @volatility','&#10;')"/>
    </xsl:if>
    <xsl:if test="$r[@volatility='volatile'][@maturity='verified'] and not($r//data[@name='reviewed'])">
      <xsl:value-of select="concat($prefix,'R3(error): volatile+verified 必须有核对日期 data name=reviewed','&#10;')"/>
    </xsl:if>
    <xsl:for-each select="$doc//*[@maturity][not(@maturity=$maturities)]">
      <xsl:value-of select="concat($prefix,'R4(error): @maturity 非法值 &quot;',@maturity,'&quot;','&#10;')"/>
    </xsl:for-each>
    <xsl:for-each select="$doc//*[@volatility][not(@volatility=$volatilities)]">
      <xsl:value-of select="concat($prefix,'R5(error): @volatility 非法值 &quot;',@volatility,'&quot;','&#10;')"/>
    </xsl:for-each>
    <xsl:for-each select="$doc//*[@tool]">
      <xsl:variable name="bad" select="tokenize(normalize-space(@tool),'\s+')[not(.=$tools)]"/>
      <xsl:if test="exists($bad)">
        <xsl:value-of select="concat($prefix,'R6(error): @tool 非法值 &quot;',string-join($bad,' '),'&quot;','&#10;')"/>
      </xsl:if>
    </xsl:for-each>
    <xsl:for-each select="$doc//term[not(@keyref)]">
      <xsl:value-of select="concat($prefix,'R7(warning): 裸 term 未用 keyref（术语库建成后应引 keyref）','&#10;')"/>
    </xsl:for-each>
    <!-- R8（2026-08-19 改判据）与 R21（同日新增）共用同一组变量：两条问的是同一件事的
         两面（来源状况是什么 / 该状况配不配 verified），判定若各写一份必然漂移。
         两态靠**结构**互斥，不靠措辞：节内有 ul＝列条目态，无 ul＝声明态。
         这一点是刻意的——早先试过「节内有外链即算有来源」，它会被无来源篇里那句
         「某某源不承担本篇任何断言」的反向声明满足，声明句就此变成可有可无。 -->
    <xsl:variable name="srcSections" select="$doc//section[normalize-space(title)='来源']"/>
    <xsl:variable name="srcSection" select="$srcSections[1]"/>
    <xsl:variable name="srcList" select="$srcSection//ul"/>
    <!-- 无来源态的判据是一个固定字面，不是"有文字即可"：机器认的是本节第一段以
         「本篇无外部来源」开头，作者写下这句时那份心理成本仍在。 -->
    <xsl:variable name="declaresNone"
                  select="boolean($srcSection/p[1][starts-with(normalize-space(.),'本篇无外部来源')])"/>
    <xsl:if test="count($srcSections) &gt; 1">
      <xsl:value-of select="concat($prefix,'R8(error): 有 ',count($srcSections),' 个「来源」节——来源状况只声明一次，多个节让判定失去唯一解','&#10;')"/>
    </xsl:if>
    <xsl:if test="$r[self::concept or self::reference]">
      <xsl:choose>
        <!-- 交付物源（体裁 artifact，构建成 CLAUDE.md / AGENTS.md）不设来源节：
             整节会进常驻上下文。这类篇把来源落 prolog/source。
             门开得这么窄有实据：全库 82 篇带 prolog/source，其中 8 篇的来源节明写
             「该源作为元数据来源，不承担本篇任何断言」——prolog/source 在本库是元数据
             出处字段，不是断言出处，无条件认它等于发一张覆盖全库的免检通行证。
             'artifact' 这个字面内联在此，与本文件顶部三个值集变量同一处置：本文件不读词表，
             读词表的是 dita-tools lint，而 R8 要与 R21 同处（判定单源）。 -->
        <xsl:when test="empty($srcSection) and $r/@outputclass='artifact' and $r/prolog/source"/>
        <xsl:when test="empty($srcSection)">
          <xsl:value-of select="concat($prefix,'R8(error): 缺来源节——必须显式声明来源状况：有来源就在末尾「来源」一节用 ul 逐条列出（至少一条附 scope=&quot;external&quot; 的地址），无来源就让该节第一段以「本篇无外部来源，属本库方法论。」开头；只有体裁 artifact 的交付物源可免来源节，改在 prolog/source 声明','&#10;')"/>
        </xsl:when>
        <xsl:when test="exists($srcList) and $declaresNone">
          <xsl:value-of select="concat($prefix,'R8(error): 来源节自相矛盾——列了条目又声明「本篇无外部来源」。两态二选一：列条目就删掉声明句，没有来源就删掉列表','&#10;')"/>
        </xsl:when>
        <xsl:when test="exists($srcList) and empty($srcList//xref[@scope='external'])">
          <xsl:value-of select="concat($prefix,'R8(error): 来源节列了条目，却没有一条带外部地址——地址要机器看得见（scope=&quot;external&quot; 的 xref）；确实无外部来源就删掉列表，改写「本篇无外部来源，属本库方法论。」','&#10;')"/>
        </xsl:when>
        <xsl:when test="exists($srcList)"/>
        <xsl:when test="$declaresNone"/>
        <xsl:otherwise>
          <xsl:value-of select="concat($prefix,'R8(error): 来源节既不是列条目态、也不是声明态——有来源就写成 ul 逐条列出（至少一条带 scope=&quot;external&quot; 的地址），无来源就让本节第一段以「本篇无外部来源」开头','&#10;')"/>
        </xsl:otherwise>
      </xsl:choose>
    </xsl:if>
    <xsl:if test="$r/@maturity='verified' and $declaresNone">
      <xsl:value-of select="concat($prefix,'R21(error): 声明「本篇无外部来源」的篇不得标 maturity=&quot;verified&quot;——verified 的定义是来源已逐条核对，无来源对不上该定义，成熟度封顶 curated','&#10;')"/>
    </xsl:if>
  </xsl:template>

  <!-- 单文件调试入口：java ... net.sf.saxon.Transform -s:<file> -xsl:check-rules.xsl
       review.sh 不再用这条（改走 main），留着给人手动核对单篇。 -->
  <xsl:template match="/">
    <xsl:call-template name="check-doc">
      <xsl:with-param name="doc" select="."/>
    </xsl:call-template>
  </xsl:template>

  <!-- 批量入口：java ... net.sf.saxon.Transform -it:main -xsl:check-rules.xsl "kb-dir=file://<KB 绝对路径>"
       一次 JVM 执行完 kb/topics 下全部 .dita 的 R1–R8 与 R21。 -->
  <xsl:template name="main">
    <xsl:if test="not($kb-dir)">
      <xsl:message terminate="yes">缺 kb-dir 参数：java ... -it:main -xsl:check-rules.xsl "kb-dir=file://&lt;kb 绝对路径&gt;"</xsl:message>
    </xsl:if>
    <xsl:for-each select="uri-collection(concat($kb-dir, '/topics?select=*.dita;recurse=yes'))">
      <xsl:sort select="."/>
      <xsl:variable name="uri" select="."/>
      <xsl:variable name="rel" select="concat('topics/', substring-after($uri, '/topics/'))"/>
      <xsl:try>
        <xsl:call-template name="check-doc">
          <xsl:with-param name="doc" select="doc($uri)"/>
          <xsl:with-param name="prefix" select="concat('[rules] ', $rel, ': ')"/>
        </xsl:call-template>
        <xsl:catch>
          <!-- 格式错的 XML 单篇隔离，不拖垮整批；具体结构错误由 review.sh 第 1 节
               （dita validate）定位，这里只报"这篇没法跑规则"，附 Saxon 的解析错误信息。 -->
          <xsl:value-of select="concat('[rules] ', $rel, ': R0(error): XML 解析失败，跳过本篇业务规则检查——', $err:description, '&#10;')"/>
        </xsl:catch>
      </xsl:try>
    </xsl:for-each>
  </xsl:template>
</xsl:stylesheet>

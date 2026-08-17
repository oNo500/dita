<?xml version="1.0" encoding="UTF-8"?>
<!-- KB 业务规则检查（可执行版，Saxon 跑）。实现 schema/rules.sch 的 R1–R10。
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

  <!-- 规则实现本体：对一份已解析文档跑 R1–R10（R9 除外，见上），每条违规一行，
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
    <xsl:if test="$r[self::concept or self::reference] and not($r/prolog/source or $r//xref[@scope='external'] or $r//data[@name='source'])">
      <xsl:value-of select="concat($prefix,'R8(error): concept/reference 必须有至少一个来源','&#10;')"/>
    </xsl:if>
    <xsl:if test="$r[@outputclass='quickstart'] and not($r//xref)">
      <xsl:value-of select="concat($prefix,'R10(error): quickstart 必须 xref 到所属领域概览','&#10;')"/>
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
       一次 JVM 跑完 kb/topics 下全部 .dita 的 R1–R10。 -->
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

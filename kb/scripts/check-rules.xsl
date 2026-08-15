<?xml version="1.0" encoding="UTF-8"?>
<!-- KB 业务规则检查（可执行版，Saxon 跑）。实现 schema/rules.sch 的 R1–R10。
     用 DITA-OT 自带 Saxon-HE 执行，不引第三方 Schematron 编译器。
     ⚠ 须与 schema/rules.sch 保持同步（那份是人读规格，本份是可执行实现）；
        将来接入 SchXslt 后可直接编译 rules.sch，删除本份消除重复。
     输出：每行一条违规 "Rx(error|warning): <msg>"；无输出＝该文件全过。
     文件名前缀由 review.sh 负责。R9（map 层跨文档）归 dimension-coverage.py。 -->
<xsl:stylesheet version="3.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:output method="text"/>

  <xsl:variable name="maturities" select="('draft','curated','verified')"/>
  <xsl:variable name="volatilities" select="('stable','volatile')"/>
  <xsl:variable name="tools" select="('tool-claude-code','tool-codex','tool-antigravity')"/>

  <xsl:template match="/">
    <xsl:variable name="r" select="*"/>
    <xsl:variable name="hasShortdescType" select="boolean($r[self::concept or self::reference or self::task or self::troubleshooting])"/>
    <xsl:variable name="topicType" select="boolean($r[self::concept or self::reference or self::task or self::troubleshooting or self::glossentry])"/>

    <xsl:if test="$hasShortdescType and not($r/shortdesc[normalize-space(.)!=''])">
      <xsl:text>R1(error): topic 必须有非空 shortdesc&#10;</xsl:text>
    </xsl:if>
    <xsl:if test="$topicType and not($r/@volatility)">
      <xsl:text>R2(error): topic 必须显式标 @volatility&#10;</xsl:text>
    </xsl:if>
    <xsl:if test="$r[@volatility='volatile'][@maturity='verified'] and not($r//data[@name='reviewed'])">
      <xsl:text>R3(error): volatile+verified 必须有核对日期 data name=reviewed&#10;</xsl:text>
    </xsl:if>
    <xsl:for-each select="//*[@maturity][not(@maturity=$maturities)]">
      <xsl:value-of select="concat('R4(error): @maturity 非法值 &quot;',@maturity,'&quot;&#10;')"/>
    </xsl:for-each>
    <xsl:for-each select="//*[@volatility][not(@volatility=$volatilities)]">
      <xsl:value-of select="concat('R5(error): @volatility 非法值 &quot;',@volatility,'&quot;&#10;')"/>
    </xsl:for-each>
    <xsl:for-each select="//*[@tool]">
      <xsl:variable name="bad" select="tokenize(normalize-space(@tool),'\s+')[not(.=$tools)]"/>
      <xsl:if test="exists($bad)">
        <xsl:value-of select="concat('R6(error): @tool 非法值 &quot;',string-join($bad,' '),'&quot;&#10;')"/>
      </xsl:if>
    </xsl:for-each>
    <xsl:for-each select="//term[not(@keyref)]">
      <xsl:text>R7(warning): 裸 term 未用 keyref（术语库建成后应引 keyref）&#10;</xsl:text>
    </xsl:for-each>
    <xsl:if test="$r[self::concept or self::reference] and not($r/prolog/source or $r//xref[@scope='external'] or $r//data[@name='source'])">
      <xsl:text>R8(error): concept/reference 必须有至少一个来源&#10;</xsl:text>
    </xsl:if>
    <xsl:if test="$r[@outputclass='quickstart'] and not($r//xref)">
      <xsl:text>R10(error): quickstart 必须 xref 到所属领域全景&#10;</xsl:text>
    </xsl:if>
  </xsl:template>
</xsl:stylesheet>

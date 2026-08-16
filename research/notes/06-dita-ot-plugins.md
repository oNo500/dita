# 06 · DITA-OT 插件开发

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/toolchain/plugin-extension-points.dita` ← 插件是什么、plugin.xml 结构、主要扩展点分类、现成插件参考；`kb/topics/dita/toolchain/xslt-override.dita` ← 完整示例、覆盖机制的关键、处理 2.0 特有元素；`kb/topics/dita/toolchain/custom-transtype.dita` ← 写全新的 transtype、挂 Ant target、界面字符串本地化；`kb/topics/dita/toolchain/plugin-debugging.dita` ← 调试插件的工作流；不迁小节：无），本文冻结为调研档案，不再更新。

> 二次开发的第二条路：**改变输出**。

## 插件是什么

一个目录，里面必须有 `plugin.xml`。装进 DITA-OT 后，它声明的"特性（feature）"会被合并进工具箱的构建配置。

```
com.example.myplugin/
├── plugin.xml           ← 必需
├── build.xml            ← 可选：自定义 Ant target
├── xsl/
│   └── custom.xsl       ← XSLT 覆盖
├── cfg/common/vars/
│   └── zh-CN.xml        ← 界面字符串本地化
├── resource/
│   └── custom.css
└── catalog-dita.xml     ← 如果带 schema
```

安装 / 卸载：

```bash
dita install ./com.example.myplugin       # 目录
dita install ./myplugin.zip               # zip
dita install https://.../plugin.zip       # URL
dita install org.lwdita                   # 从官方注册表按 ID 装
dita plugins                              # 查看已装
dita uninstall com.example.myplugin
dita install                              # 无参数 = 重建集成配置（改了插件后必跑！）
```

> **最高频的坑**：改了插件文件后忘了跑 `dita install`，然后困惑"为什么没生效"。DITA-OT 在安装时把各插件的配置**编译**进 `config/` 和生成的 XSL 导入链里，不是运行时读取的。

---

## plugin.xml 结构

```xml
<?xml version="1.0" encoding="UTF-8"?>
<plugin id="com.example.myhtml">
  <!-- 版本与依赖 -->
  <feature extension="package.version" value="1.0.0"/>
  <require plugin="org.dita.html5"/>

  <!-- 注册新 transtype，继承 html5 -->
  <transtype name="myhtml" extends="html5" desc="带公司品牌的 HTML5 输出"/>

  <!-- XSLT 覆盖：追加到 html5 的导入链末尾（后导入者优先） -->
  <feature extension="dita.xsl.html5" file="xsl/custom.xsl"/>

  <!-- 传参数给 XSLT -->
  <feature extension="dita.conductor.html5.param" file="params.xml"/>

  <!-- 挂 Ant target -->
  <feature extension="dita.conductor.target.relative" file="build.xml"/>

  <!-- 前后置钩子 -->
  <feature extension="depend.preprocess.pre"  value="my.before.preprocess"/>
  <feature extension="depend.preprocess.post" value="my.after.preprocess"/>

  <!-- 注册 schema catalog -->
  <feature extension="dita.specialization.catalog.relative" file="catalog-dita.xml"/>

  <!-- 追加 Java 库到 classpath -->
  <feature extension="dita.conductor.lib.import" file="lib/my-processor.jar"/>

  <!-- 自定义诊断消息 -->
  <feature extension="dita.xsl.messages" file="resource/messages.xml"/>

  <!-- 打包时包含的资源 -->
  <feature extension="dita.resource.copy" file="resource/custom.css"/>
</plugin>
```

### 主要扩展点分类

| 类别 | 代表扩展点 | 用途 |
|---|---|---|
| **XSLT 导入** | `dita.xsl.html5`、`dita.xsl.xslfo`、`dita.xsl.markdown` | 覆盖模板 —— **最常用** |
| **XSLT 参数** | `dita.conductor.html5.param`、`dita.conductor.pdf2.param` | 向 XSLT 传全局 `<xsl:param>` |
| **Ant 集成** | `dita.conductor.target.relative`、`dita.conductor.transtype.check` | 加 target、注册 transtype 校验 |
| **前后置** | `depend.preprocess.pre` / `.post`、`depend.preprocess.<阶段>.pre` | 在流水线特定位置插入自己的 target |
| **Schema** | `dita.specialization.catalog.relative` | 注册专门化 RNG/DTD |
| **资源** | `dita.conductor.lib.import`、`dita.resource.copy` | jar 包、静态资源 |
| **元信息** | `package.version`、`package.support.name` | 版本与支持信息 |

完整清单：<https://www.dita-ot.org/dev/extension-points/plugin-extension-points>

---

## 完整示例：给 `<note>` 换个渲染

```xml
<!-- xsl/custom.xsl -->
<xsl:stylesheet version="3.0"
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns="http://www.w3.org/1999/xhtml"
    exclude-result-prefixes="xsl">

  <!-- ★ 按 @class 匹配，不按元素名 -->
  <xsl:template match="*[contains(@class, ' topic/note ')]">
    <div class="callout callout-{(@type, 'note')[1]}">
      <div class="callout-icon" aria-hidden="true">
        <xsl:value-of select="if (@type = 'warning') then '⚠️'
                              else if (@type = 'tip')  then '💡'
                              else 'ℹ️'"/>
      </div>
      <div class="callout-body">
        <xsl:apply-templates/>
      </div>
    </div>
  </xsl:template>

  <!-- 挂自己的 CSS -->
  <xsl:template name="gen-user-head">
    <link rel="stylesheet" type="text/css" href="{$PATH2PROJ}css/custom.css"/>
  </xsl:template>

</xsl:stylesheet>
```

装上后：

```bash
dita install ./com.example.myhtml
dita -i root.ditamap -f myhtml -o out
```

### 覆盖机制的关键

DITA-OT 在 `dita install` 时把所有声明了 `dita.xsl.html5` 的插件文件**按依赖顺序追加 `<xsl:import>`** 到生成的入口 XSL 里。XSLT 规则：**后导入的优先级高**，所以你的模板自然覆盖内置模板。

多个插件覆盖同一模板时，用 `priority` 抢优先级，或用 `<xsl:next-match/>` 调用被覆盖的那个：

```xml
<xsl:template match="*[contains(@class, ' topic/note ')]" priority="10">
  <xsl:next-match/>          <!-- 先跑原本的渲染 -->
  <p class="note-footer">补充说明</p>
</xsl:template>
```

---

## 处理 2.0 特有元素

新元素同样按 `@class` 匹配。几个常用的：

```xml
<!-- <div>：通用块容器，默认可能只是透传，值得自己控制 -->
<xsl:template match="*[contains(@class, ' topic/div ')]">
  <div class="{(@outputclass, 'dita-div')[1]}">
    <xsl:apply-templates/>
  </div>
</xsl:template>

<!-- <titlealt>：按 role 分流 -->
<xsl:template match="*[contains(@class, ' topic/titlealt ')]
                       [contains(@title-role, 'search')]" mode="search-title">
  <xsl:value-of select="normalize-space(.)"/>
</xsl:template>

<!-- <video>：加自定义播放器包装 -->
<xsl:template match="*[contains(@class, ' multimedia-d/video ')]">
  <figure class="video-wrap">
    <xsl:next-match/>
  </figure>
</xsl:template>
```

> `@class` 里的模块名（斜杠前那部分）在草案期间可能变动。写模板前先用 `-f dita` 出一份中间结果，**亲眼确认实际的 `@class` 值**，别照着记忆写。

---

## 挂 Ant target

```xml
<!-- build.xml -->
<project name="com.example.myhtml">
  <target name="my.after.preprocess">
    <echo message="preprocess 完成，临时目录：${dita.temp.dir}"/>
    <!-- 只对特定 transtype 生效：用 Ant 原生的 condition + target @if。
         （<if>/<then> 是 ant-contrib 的任务，不是 Ant 原生的，
          是否随 DITA-OT 分发因版本而异，别依赖它） -->
    <condition property="run.my.analyzer">
      <equals arg1="${transtype}" arg2="myhtml"/>
    </condition>
    <antcall target="my.analyzer"/>
  </target>

  <target name="my.analyzer" if="run.my.analyzer">
    <java classname="com.example.Analyzer" fork="false">
      <arg value="${dita.temp.dir}"/>
    </java>
  </target>
</project>
```

配合 plugin.xml 里的 `<feature extension="depend.preprocess.post" value="my.after.preprocess"/>`。

有用的内置属性：

| 属性 | 含义 |
|---|---|
| `${dita.temp.dir}` | 临时目录（中间产物在这） |
| `${output.dir}` | 最终输出目录 |
| `${args.input}` | 输入 map 路径 |
| `${transtype}` | 当前 transtype |
| `${dita.dir}` | DITA-OT 安装目录 |
| `${dita.plugin.<插件ID>.dir}` | 某插件的目录 |

---

## 写全新的 transtype

不继承已有格式，从 preprocess 结果自己生成（例如出 JSON 给前端搜索引擎用）：

```xml
<plugin id="com.example.searchindex">
  <transtype name="searchindex" desc="生成全文检索 JSON 索引"/>
  <feature extension="dita.conductor.transtype.check" value="searchindex"/>
  <feature extension="dita.conductor.target.relative" file="build.xml"/>
</plugin>
```

```xml
<!-- build.xml -->
<project name="com.example.searchindex">
  <target name="dita2searchindex" depends="build-init, preprocess, searchindex.generate"/>

  <target name="searchindex.generate">
    <xslt basedir="${dita.temp.dir}" destdir="${output.dir}"
          includes="**/*.dita" extension=".json"
          style="${dita.plugin.com.example.searchindex.dir}/xsl/to-json.xsl">
      <param name="OUTPUTDIR" expression="${output.dir}"/>
    </xslt>
  </target>
</project>
```

**约定**：transtype 名为 `X` 时，Ant target 必须叫 `dita2X`。

---

## 界面字符串本地化

DITA-OT 输出里的固定文本（"Related concepts"、"Parent topic:" 等）来自 `strings.xml` 体系：

```xml
<!-- cfg/common/vars/zh-CN.xml -->
<vars xmlns="http://www.oasis-open.org/architecture/2005/">
  <variable id="Related concepts">相关概念</variable>
  <variable id="Parent topic">上级主题</variable>
  <variable id="Note">注意</variable>
</vars>
```

配 `<feature extension="dita.xsl.strings" file="cfg/common/vars/strings.xml"/>`。

---

## 现成插件参考

学插件写法最快的方式是读源码：

| 插件 | 看点 |
|---|---|
| **org.lwdita**（Jarno Elovirta） | Markdown ↔ DITA 双向，自定义 transtype + Java 解析器的完整范例 |
| **org.dita.html5** | 官方 HTML5 输出，标准 XSLT 组织方式 |
| **com.elovirta.pdf** / **dita-ot-pdf-css** | 用 CSS 出 PDF，绕开 XSL-FO |
| **DITA-OT 的 2.0 语法插件** | 看 catalog 怎么把 2.0 的 public ID 映射到语法文件 |

插件注册表：<https://www.dita-ot.org/plugins>

---

## 调试插件的工作流

```bash
# 1. 改插件
vim xsl/custom.xsl

# 2. 重装（★ 必须）
dita install ./com.example.myhtml

# 3. 带调试信息构建
dita -i test/root.ditamap -f myhtml -o /tmp/out --temp=/tmp/tmp --debug -v

# 4. 检查中间产物
ls /tmp/tmp
```

---

→ 下一步：[07-programmatic-processing.md](07-programmatic-processing.md)

---

## 来源

**已逐页核对（2026-08）**

- [插件扩展点](https://www.dita-ot.org/dev/extension-points/plugin-extension-points) — 扩展点分类（通用 / 预处理 / XSLT-import / XSLT-parameter / 版本与支持信息）；扩展点在 `plugin.xml` 的 `<feature extension="...">` 中声明；**XSLT 覆盖靠安装时自动追加 import 语句**；可用 `${transtype}` 限定自定义代码的生效范围；`depend.preprocess.post` 等标识符
- [DITA 2.0 preview 支持](https://www.dita-ot.org/dev/reference/dita-v2-0-support.html) — 2.0 新元素在 DITA-OT 中的可用性与版本下限（`<div>` `<video>` `<titlealt>` `<keytext>` 等），用于"处理 2.0 特有元素"一节的前提
- [扩展 XML catalog 文件](https://www.dita-ot.org/dev/topics/plugin-xmlcatalog) — `dita.specialization.catalog.relative` 与 `org.dita.pdf2.catalog.relative` 扩展点用于注册专门化 schema
- [DITA-OT 发布说明](https://www.dita-ot.org/dev/release-notes/) — 4.4 版本基线

**未逐页核对，来自通用 DITA-OT 实践**

- `plugin.xml` 完整示例中各 feature 的组合写法
- `dita install` / `uninstall` / `plugins` 的命令行为，以及"改插件后必须重跑 `dita install`"
- Ant target 示例、内置属性表（`${dita.temp.dir}` `${output.dir}` `${dita.plugin.<ID>.dir}` 等）
- 自定义 transtype 的 `dita2X` target 命名约定
- 界面字符串本地化（`cfg/common/vars/`、`dita.xsl.strings`）的配置方式
- XSLT 覆盖示例代码与 `<xsl:next-match/>` / `priority` 的用法
- 现成插件推荐表（判断性内容）

> ⚠️ 正文中 `@class` 里模块名的具体取值（如 `multimedia-d/video`）**未经核对**，草案期间可能变动。正文已提示：写模板前先用 `-f dita` 出中间结果亲眼确认。

**插件注册表**：<https://www.dita-ot.org/plugins>

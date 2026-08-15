Subject Scheme（主题方案）

解决的核心问题

  在大型内容体系里，如果每个作者都可以随意在 Topic 上写 audience="高级用户" 或 audience="expert"，久而久之标签会变得混乱，过滤和检索都失效。

  Subject Scheme 的作用就是：把元数据的"合法值"集中定义在一个地方，然后通过 enumerationdef 强制绑定到属性上，让编辑器和构建工具统一校验。
  ──────

  ### 这套机制由哪几个 DITA 特性组成

| 特性         | DITA 规范名称                      | 你的文件里的体现                                  |
| ------------ | ---------------------------------- | ------------------------------------------------- |
| 主题方案 Map | Subject Scheme Map                 | 整个 subjectScheme.ditamap 文件本身               |
| 受控分类树   | Subject Definition                 | 九个领域 + 子节点的 <subjectdef> 树               |
| 属性枚举约束 | Enumeration Definition             | 文件末尾的 <enumerationdef> 绑定块                |
| 属性专门化   | Attribute Specialization           | @dimension 从 @props 派生                         |
| 条件过滤     | Profiling / Conditional Processing | @maturity, @tool, @volatility 配合 DITAVAL 出变体 |
| ──────       |                                    |                                                   |
  ### 在 DITA 生态里的定位

  这是 DITA 里IA 层面最高级别的治理工具，也是大多数 DITA 用户从未触碰过的部分。

  大多数 DITA 用户只用 <map> 和 <topic>，把 Subject Scheme 用起来的，通常是有大型内容体系（上千个 Topic）需要多维检索和多版本发布的组织。Heretto、Paligo 这类 CCMS 在 UI 上提供的"条件属性下拉框"，底层就是在消费  Subject Scheme 的定义。

## 治理缺失

 ### 现在真正缺的东西
  1. SubjectScheme 的枚举值校验没有在 DITA-OT 构建时生效
  你在 check-rules.xsl 里是手写硬编码了合法值（maturity、volatility、tool）。但 subjectScheme.ditamap 里的 <enumerationdef> 理论上应该由 DITA-OT 在构建时自动执行——但实际上 DITA-OT 只在编辑器（Oxygen
    等）做这个，命令行构建不会自动报枚举错误。所以你的 XSL 脚本是对的弥补，但两份定义要手动同步。
  2. @dimension 的枚举值没有被 check-rules.xsl 覆盖
  dimension 有 45 个合法值，目前脚本里没有校验 @dimension 是否合法，只有 dimension-coverage.py 在统计覆盖度，但没有阻断非法值写入。
  3. Map 级别的结构没有治理工具
  review.sh 只扫了 topics/ 下的 .dita 文件。maps/ 目录里 Map 结构是否规范（比如某个 topicref 引用了不存在的文件）没有检查。

  4. 没有可视化的结构全景
  你只能跑 preview.sh 看 HTML5 输出，看到的是"读者视角"的内容页面。但"IA 视角"的结构全景（整个知识树长什么样、哪些域是空的、哪些 Topic 没有归入任何 Map）没有视图。
    ──────
  ### 最值得补的两件事
  补 R11：@dimension 枚举校验
  在 check-rules.xsl 里加一段，把词表里的 45 个 dim-* 值写进去，校验 @dimension 属性里的每个值都合法。
  补一个 Map 结构报告脚本
  写一个 Python 脚本，读取 maps/root.ditamap（递归展开所有 <mapref>），输出：

  • 所有 topicref 引用的文件是否存在
  • 哪些 topics/ 下的文件没有被任何 Map 引用（孤儿 Topic）
  • 各域 Map 的 Topic 数量统计
<div align="center">

<img src="assets/hero.png" width="440" alt="Flowtrace">

# Flowtrace

**让 agent 跑一个任务的方式：透明、可复用、会进化。**

[![GitHub stars](https://img.shields.io/github/stars/AIScientists-Dev/flowtrace?style=flat-square&logo=github)](https://github.com/AIScientists-Dev/flowtrace/stargazers) [![MIT License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](../LICENSE) [![Homepage](https://img.shields.io/badge/Homepage-morphmind.ai-lightgrey?style=flat-square)](https://morphmind.ai) [![Discord](https://img.shields.io/badge/Discord-Join-5865F2?style=flat-square&logo=discord&logoColor=white)](https://discord.gg/x9mtbMEx) [![X](https://img.shields.io/badge/X-Follow-000000?style=flat-square&logo=x&logoColor=white)](https://x.com/morphmind__ai?s=11)

[**它能做什么**](#它能做什么) · [**安装**](#安装) · [**怎么用**](#怎么用一个-trace) · [**自建 trace**](#做你自己的-trace) · [**示例**](EXAMPLES.zh-CN.md) · [**CLI**](trace/CLI.md)

[English](../README.md) · **简体中文**

⭐ **觉得 Flowtrace 有用，就给个 star** *(按钮在右上角)* —— 这是我们判断要不要继续公开打磨它的依据。

</div>

---

Flowtrace 把一类任务变成一个 trace，让你的 agent 一步步走完。配合任何 LLM / agent，包括 Claude Code、Codex、Cursor。

<div align="center">
<table><tr>
<td align="center" valign="top"><img src="assets/examples/nvda-decision.png" height="340" alt="一个 trace 的图：几条采集支线汇入综合论点，再到仓位与风控，最后汇成一份研报"><br><sub>流程</sub></td>
<td align="center" valign="top"><img src="assets/examples/nvda-decision-pdf.png" height="340" alt="交付物：一份固定格式的研报 PDF"><br><sub>交付物</sub></td>
</tr></table>
</div>

<p align="center"><a href="assets/examples/nvda-decision.pdf"><strong>看完整研报 PDF</strong></a></p>

## 它能做什么

**看得见。** 把工作看成一张图，任意一步的产出都能点开。你知道结果是怎么来的，而不只是看到一个结论。

**有据可查。** 每一步都写出真实文件，agent 没法引用磁盘上不存在的东西。东西就在那儿，能打开核对，不靠"你信我"。

**可追溯。** 每次 run 都作为一个独立版本存下来：换个思路再试也不丢掉管用的那一版，还能逐步回看任意一次是怎么跑出来的。

**可复用。** 下一个同类任务从你已经搭好的东西起步，同一套步骤换新输入重跑，不必从零开始。

**会进化。** 一个步骤改好一次，之后每次 run 都带着，越打磨越合手。

## 安装

```bash
git clone https://github.com/AIScientists-Dev/flowtrace.git
cd flowtrace
./scripts/install.sh                      # 编译，并将 `flowtrace` 软链接至 ~/.local/bin/
```

更新：`git pull && ./scripts/install.sh`。如需更改软链接位置，使用 `INSTALL_DIR=…`。

Flowtrace 还带了一个 **`make-trace` skill**，放在 `skills/make-trace/`。把它复制或软链到你 coding agent 的 skills 目录（Claude Code、Codex、Cursor）里，就能用 `/make-trace` 生成 trace。见 [自建 trace](#做你自己的-trace)。

想改工具本身就手动编译：`cd frontend && npm install && npm run build && cd .. && cargo build --release`。前端要先编译，因为 UI 会在编译时通过 `rust-embed` 一起打包进那个 Rust 二进制里。

## 怎么用一个 trace

一个 trace 就是一张可视化 DAG，你和 AI 一起看着它跑。端到端驱动一遍：

1. **启动服务。** `flowtrace serve` 在 `http://localhost:3000` 打开这张图。git 历史让每一步都能回溯。

2. **跑起来。** 在 trace 文件夹里，对你的 agent 说 _"运行这个 trace"_。它读 `trace.json` 和 `docs/trace/CLI.md`，然后逐节点推进。每个节点发四条命令：说一句要做什么、动手做、带 asset 标记完成、给 UI 发一条结构化 reply：

   ```bash
   RUN=$(flowtrace run new --name "first pass" | tail -1)

   # 每个 step，按依赖顺序：
   flowtrace step <id> running --message "我接下来要做什么"
   #   … agent 干活并写出这一步的 asset …
   flowtrace step <id> done --asset <file>
   flowtrace reply < reply.json          # 给 UI 的结构化输出

   # 最后一步之后：
   flowtrace deliverable done --asset <final-output>
   ```

   每次 CLI 写操作产生一个 git commit。每步一完成，UI 里对应节点就亮起。

你可以在任意节点停下，改掉 AI 在那一步的判断。所有依赖它的下游节点都会失效并重跑，所以你在单个节点上介入，而不必重做整个任务。

要跑任何东西，先得在 `~/traces/` 下有一个 trace。两种办法拿到它：

- **[自建](#做你自己的-trace)：** 把任意素材变成一个新的 trace。
- **[试一个参考 trace](#试一个参考-trace)：** 跑一个我们带的，几秒就能看到一张装满内容的图。

## 做你自己的 trace

**`make-trace` skill** 会把任意**素材**提炼成一条 trace：一份 `SKILL.md`、一份操作手册、一段对话记录、一句话需求、一套你脑子里的流程，或者你刚和 agent 做完的一件事。把素材交给它，运行 `/make-trace`。

什么时候做一条：

- **这类任务你以后还会再做。** 你刚跑的这一次已经把步骤和顺序理清了，trace 把它记下来，下次直接复用形状，不用重新推导。
- **它得每次都跑出同一种结果。** 步骤和输出格式是固定的，两次之间变的只有输入。
- **你想核对中间，而不只是答案。** 每一步都写出一个你能打开的文件，而且改好一步，之后每次运行都继承。

下面六步把示例 `nvda-decision` 从一份 skill 提炼出来，换成任何素材都一样。

这个示例随 repo 一起提供。运行 `bash scripts/examples/nvda-decision/build.sh` 会在 `~/traces/nvda-decision/` 重新生成它：一个 16 节点的 _"该不该买 NVDA？"_ trace —— 四条采集支线汇入一个综合论点，仓位与风控接在其后，最后报告从所有节点汇入。

最后两个节点负责呈现：渲染图表，并排版出一份**固定格式的研报 PDF**。格式由代码（`scripts/typeset.py`）强制，所以换任何股票跑出来都是同样格式、可引用的研报，只有数字变。

**0. 准备素材。** 一份 `SKILL.md`、一段对话、一份操作手册、脑子里的一套流程：任何讲"某类任务该怎么做"的东西都行。本示例提炼的是 [`us-stock-analysis` skill](https://github.com/tradermonty/claude-trading-skills)（作者 tradermonty），并组合了它的几个姊妹 skill（仓位、敞口、宏观 / 新闻 / 板块、情景综合），补上那个工作流交给它们的那些支线。

**1. 初始化一个空 trace。**

```bash
cd ~/traces
flowtrace init nvda-decision      # 生成 nvda-decision/，含 .git 与一个空的 trace.json
```

**2. 把 skill 提炼成 DAG。** 进到新文件夹，对你的 agent 说："读一下 `docs/trace/CLI.md` 和这份素材，把它提炼成一个 trace，在 `trace.json` 里把每个 step 和它们之间的依赖填好。" 这一步是关键：agent 要把藏在文字里的步骤一个个抽出来，再判断它们之间该怎么连，哪些并行、哪些汇到一处。**箭头就是知识**，连对了 trace 才管用。

```bash
flowtrace validate            # 校验 schema
flowtrace show --fmt mermaid  # 将 DAG 渲染出来审视一遍
```

**3. 核一遍是否忠实（建议做）。** 提炼靠的是判断，而判断常出错：本该有分叉、有汇合的一张图，容易被想当然地拉成一条直线。所以再找**另一个独立的** agent，拿原始 skill 来比对这张 DAG 的连线和覆盖：每一步都在吗？每条依赖都成立吗？有没有漏掉的？把它指出的问题逐条改掉。（这个示例改了两轮，独立审查才给出"忠实且完整"的结论。）

**4. 给每个节点写说明。** 每个 `steps/<id>/STEP.md` 一份：一个简短的 Markdown 文件，顶部可选一段 YAML frontmatter，下面是正文。frontmatter 里的 `reads`/`writes` 会显示在 UI 上，正文讲这一步怎么做。把横切的注意事项就近折进它影响的那几步里。

```markdown
---
name: valuation
description: 估值比率对历史/同行，给出合理价值区间和目标价。
reads:
  - ingest_fundamentals/fundamentals.json
writes:
  - valuation.json
---

# Valuation

从基本面算 P/E（TTM / forward）、PEG、P/B、EV/EBITDA；对比这只票自己的历史和同行；
估一个合理价值区间和目标价。输出 `valuation.json`。估值是判断，不是查比率。
```

**5. 运行它。** 一次 run 的输入就是普通文件：没有 `inputs` 字段，把它们放进 `resources/` 文件夹，让相关 step 的 `reads:` 指向它们。然后照 [怎么用一个 trace](#怎么用一个-trace) 跑一遍；`flowtrace serve` 会让 DAG 随 agent 推进一个个亮起来。

整条路就是这样：一份 skill 进去，一个看得见、可介入的 trace 出来 —— 这里它最后产出一份固定格式的研报 PDF。第 2 到 5 步是 agent 的活；你给素材、给输入，并在你想介入的任何节点上拿主意。

## 试一个参考 trace

在 **[示例画廊](EXAMPLES.zh-CN.md)** 里能一次看到每个例子的 DAG。

`dream-analysis` / `nested-deps` / `iris-analysis` / `tailored-resume` / `nvda-decision` 这几个示例，各自对应 `scripts/examples/<id>/` 下的一个 builder。运行一次 builder，它会在 `~/traces/<id>/` 下生成一个真实的 trace 文件夹（带有独立的 `.git` 与 `runs/`），完整走一遍 CLI 流程，并留下一次现成的 demo run。

```bash
bash scripts/examples/iris-analysis/build.sh   # → ~/traces/iris-analysis/
flowtrace serve                                    # → http://localhost:3000
```

此外还有一个名为 `spring-demo` 的 builder，它刻意放慢构建 trace 时的 commit 节奏，以便在 UI 上呈现 NodeMap 节点逐个入场、连线依次绘制的动画。它仅搭建 DAG 结构，不执行完整的 run 流程：

```bash
bash scripts/examples/spring-demo/build.sh     # → ~/traces/spring-demo/
```

`flowtrace serve` 默认监听 `--scope ~/traces/`，因此该目录下放置的任何 trace 文件夹都会被识别。各 builder 分别演示什么，参见 [docs/trace/REFERENCE-TRACES.md](trace/REFERENCE-TRACES.md)。

## 给用这个 CLI 的 AI agent

需从一份素材**新建** trace？请参照上文 [做你自己的 trace](#做你自己的-trace)。本节讲述的是如何驱动一个**已经存在**的 trace。

若你是一个 AI 助手，正在某个 trace 文件夹中工作，**只需阅读两份文件**即可掌握全部接口：

1. **`docs/trace/CLI.md`**：规范级的系统约定。概念、每条命令、每个 flag、reply 的负载结构、错误码清单。每次会话开始时读一遍即可。
2. **`trace.json`**（或 `flowtrace show --fmt json`）：当前 trace 的方案。step id、依赖、asset 声明、deliverable 的结构。每个 trace 读一遍。

读完这两份即可开始下达命令。若中途忘记某项的结构，二进制自带文档：

| 你想了解 | 运行 |
|---|---|
| 该命令如何调用？ | `flowtrace <cmd> --help` |
| reply 的负载结构？ | `flowtrace explain reply` |
| evidence 有哪些字段？ | `flowtrace explain reply.evidence`，再逐层深入：`flowtrace explain reply.evidence.figure` |
| 可直接修改的最小 JSON 骨架 | `flowtrace explain reply --output example` |
| 正式的 JSON Schema（供校验器使用） | `flowtrace explain reply --output jsonschema` |
| state.json 的结构？ | `flowtrace explain state` |
| trace.json 的结构？ | `flowtrace explain trace` |

CLI.md、`--help`、`explain` 三种查法都由同一份 Rust 类型生成，不会出现不一致。

## 它跟 skill、workflow 的关系

agent 技术栈里的三层，各管一段，又彼此配合：

- **Skill** 是**动作**这一层：一个函数、一个 MCP 工具，或一份 `SKILL.md`。
- **Workflow** 是**控制流**这一层：明确的次序、分支、关卡。
- **Trace** 是**编排**这一层：把各个动作怎么组合起来完成这类任务，画成一张你能看、也能改的图。

trace 的节点可以调用 skill，trace 本身也可由一份 skill 的散文提炼而来，或手动编写。完整的来龙去脉参见 [PHILOSOPHY.md](trace/PHILOSOPHY.md)。

## 工作原理

一份 `trace.json` 声明三项内容：带 DAG 依赖的 **steps**、每一步产出的 **assets**，以及最终的 **deliverable**。每次 run 存放于 trace 文件夹下的 `runs/<run_id>/`：

```
<trace_root>/
├─ .git/                                    标准 git 仓库，审计轨迹
├─ trace.json                              静态方案（DAG + deliverable）
├─ scripts/                                 多个 step 共用的代码
├─ resources/                               多个 step 共用的静态材料（参考资料 / 论文 / 主数据）
├─ steps/<step_id>/
│  ├─ STEP.md                               该步的说明与实现提示
│  ├─ scripts/                              该步专用的代码
│  └─ resources/                            该步专用的材料（图 / PDF / 测试夹具）
└─ runs/<run_id>/
   ├─ state.json                            run 的状态（唯一真相源）
   ├─ replies/NNNN.json                     仅追加的结构化输出流
   └─ <step_id>/                            运行时文件（asset 与草稿）
```

同一套命名约定（`scripts/` 放代码，`resources/` 放静态材料）在两个地方各出现一次：trace 根目录一套，每个 step 里也一套。两个及以上 step 共用的，放 trace 根目录；只有单步用的，留在那一步自己的文件夹里。`STEP.md` 用相对路径引用就行，两处都引得到。

每一次 CLI 写操作都对应一次 git commit，改动范围严格限定在它声明过的路径上：`state.json` 加上 `--asset` 指定的路径，或者新写入的 reply 文件加上它引用的 evidence 路径。草稿文件不进 git。git 历史就是审计轨迹，UI 在它之上做回溯。

**step 之间不靠"输入参数"传数据，而是靠文件。** 每一步把产物写成文件（在 `assets` 里声明），下游要用，直接读这个文件就行。更多细节见 [SCHEMA.md](trace/SCHEMA.md) 与 [PHILOSOPHY.md](trace/PHILOSOPHY.md)。

---

## 社区

- **GitHub Issues**：[反馈 bug / 提出建议](https://github.com/AIScientists-Dev/flowtrace/issues)
- **Discord**：[discord.gg/x9mtbMEx](https://discord.gg/x9mtbMEx)
- **X**：[@morphmind__ai](https://x.com/morphmind__ai?s=11)

---

MIT。详见 [`LICENSE`](../LICENSE)。

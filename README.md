<div align="center">

<img src="docs/assets/hero.png" width="440" alt="Flowtrace">

# Flowtrace

**Run your agent's work as steps you can follow, check, and reuse, instead of a stream of text that buries you and then disappears.**

Works with the agent you already use: Claude Code, Codex, Cursor.

[![GitHub stars](https://img.shields.io/github/stars/AIScientists-Dev/flowtrace?style=flat-square&logo=github)](https://github.com/AIScientists-Dev/flowtrace/stargazers) [![MIT License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE) [![Homepage](https://img.shields.io/badge/Homepage-morphmind.ai-lightgrey?style=flat-square)](https://morphmind.ai) [![Discord](https://img.shields.io/badge/Discord-Join-5865F2?style=flat-square&logo=discord&logoColor=white)](https://discord.gg/x9mtbMEx) [![X](https://img.shields.io/badge/X-Follow-000000?style=flat-square&logo=x&logoColor=white)](https://x.com/morphmind__ai?s=11)

[**What it does**](#what-it-does) · [**Get started**](#get-started) · [**Examples**](#examples) · [**Docs**](docs/trace/README.md)

**English** · [简体中文](docs/README.zh-CN.md)

⭐ **If Flowtrace looks useful, star it** *(button's top-right)* — it's how we decide what to keep building in the open.

</div>

---

Real work with an agent happens as a stream of text. You run a skill and it does the whole task in one pass, or you go back and forth in a chat that keeps growing. Either way it piles up faster than you can follow, and once it is done you are left with a wall of messages.

For a quick question that is fine. For a buy or sell call, a due diligence memo, a security gate, anything you would actually need to verify or run again, it is a problem:

- It is too much to follow. The thread grows longer than you can hold, and you lose track of what was decided and why.
- You cannot check it. A confident wrong answer looks exactly like a right one.
- You cannot steer it. One bad assumption in the middle means redoing the whole thing and hoping the good parts survive.
- It does not last. Every session is a cold start, and the good ones evaporate into scrollback.

Flowtrace runs that same work as a trace: a flow of steps the agent moves through one at a time, each leaving its output on disk. Here is one, a buy or sell decision that ends in a fixed-format, citable PDF:

<div align="center">
<table><tr>
<td align="center" valign="top"><a href="docs/assets/examples/nvda-decision.png"><img src="docs/assets/cap/hero-flow.png" height="280" alt="A trace shown as a clean flow: frame the thesis, research four angles in parallel, fold them into a valuation, deliver a one-page brief"></a><br><sub>The flow · click to enlarge</sub></td>
<td align="center" valign="top"><a href="docs/assets/examples/nvda-decision.pdf"><img src="docs/assets/cap/hero-deliverable.png" height="280" alt="The deliverable, a fixed-format research-note PDF: cover, rating and summary, embedded charts, and numbered references"></a><br><sub>The deliverable · click to open</sub></td>
</tr></table>
</div>

<p align="center"><a href="docs/assets/examples/nvda-decision.pdf"><strong>Read the full research-note PDF</strong></a></p>

## What it does

Same skills, same agent. Running that work as a trace is what changes:

**Transparent.** The work is a flow of steps you take in at a glance, not a thread you scroll. Each step's output is a file you open, so the intermediate work is right there instead of buried in messages.

<div align="center">
<img src="docs/assets/transparent.gif" width="520" alt="A trace running: it moves through its steps one at a time, each leaving a file on disk">
<br><sub>Each step runs in turn and writes its output to a file.</sub>
</div>

**Grounded.** Every result points back to the files it came from, so you verify instead of trust.

<div align="center">
<table><tr>
<td align="center" valign="top"><a href="docs/assets/cap/grounded-finance.png"><img src="docs/assets/cap/grounded-finance.png" height="300" alt="A finance result card titled trend, momentum and risk line up: four research charts (price, RSI, MACD, drawdown), a passing trend-and-momentum check, and the files they came from"></a><br><sub>Finance</sub></td>
<td align="center" valign="top"><a href="docs/assets/cap/grounded-clinical.png"><img src="docs/assets/cap/grounded-clinical.png" height="300" alt="A clinical result card: survival benefit HR 0.62, a Kaplan-Meier curve and passing checks, citing km_curve.png"></a><br><sub>Clinical</sub></td>
</tr></table>
<sub>Two high-stakes decisions, same shape: the finding, its charts, the checks that pass, and the files they came from.</sub>
</div>

**Steerable.** Fix one step and only what depends on it re-runs. The rest stays put.

**Traceable.** The whole run is files and git, so it does not vanish when you close the tab. Stop and resume anytime, hand it to a teammate, read the full history.

<div align="center">
<img src="docs/assets/cap/traceable.png" width="720" alt="Three steps over the same run: pick any step in the node map, open its full version history, then travel back to an older commit to see its state then, v2 gradient-boost AUC 0.84 versus v1 logistic 0.78">
<br><sub>Pick any step, open its history, travel to any past commit.</sub>
</div>

**Reusable.** A finished task becomes a trace you run again on new input. The method is reused, not rebuilt.

**Evolving.** The trace gets better the more it runs. When a step misses its bar, the next version switches to a method that clears it, and the version that passes is the one that sticks.

**Structured reading.** A trace exposes the work as a graph of files, not a linear transcript. The agent reads it by structure and on demand, loading a step's contract, inputs, and outputs only while working on it and following explicit dependencies rather than carrying the whole history. This bounds working context, reduces drift, and yields a representation people and agents can both inspect and extend.

You do not start from scratch. A skill, a long session, a plan, a finished run: run any of them as a trace and you get the same steps you can follow, check, and run again. Open any one to read it full size.

<div align="center">
<table>
<tr>
<td align="center" valign="top"><a href="docs/assets/onramp/convert.png"><img src="docs/assets/onramp/convert.png" width="400" alt="Convert a skill into a trace: its written steps become a flow of real, checkable steps"></a></td>
<td align="center" valign="top"><a href="docs/assets/onramp/distill.png"><img src="docs/assets/onramp/distill.png" width="400" alt="Distill a long session into a trace: the reasoning kept as steps, the back-and-forth dropped"></a></td>
</tr>
<tr>
<td align="center" valign="top"><a href="docs/assets/onramp/plan.png"><img src="docs/assets/onramp/plan.png" width="400" alt="Turn a plan into a trace: the same plan, now something you can steer and run"></a></td>
<td align="center" valign="top"><a href="docs/assets/onramp/handoff.png"><img src="docs/assets/onramp/handoff.png" width="400" alt="Keep a successful run as a trace, not a chat log: re-open the whole job, every step's files and checks, and run it again"></a></td>
</tr>
</table>
</div>

Not every task needs this. A quick one off, just chat. Flowtrace earns its place when the result matters enough to verify, or when you will run the task again.

## Get started

The fast path is to hand the repo to an agent. Point a coding agent (Claude Code, Codex, Cursor) at this folder and say:

> _"Install Flowtrace and run the tailored-resume example."_

It installs the CLI, builds a real trace at `~/traces/tailored-resume/`, and opens the web view at `http://localhost:3000`, where the flow lights up step by step.

Two ways to get a trace:

- **Try a reference.** Each example ships as a builder that creates a real trace folder and walks one full run.

  ```bash
  bash scripts/examples/tailored-resume/build.sh   # → ~/traces/tailored-resume/
  flowtrace serve                                  # → http://localhost:3000
  ```

- **Make your own.** The `make-trace` skill turns any source (a `SKILL.md`, a runbook, a chat log, a finished task) into a trace. Copy `skills/make-trace/` into the agent's skills directory and run `/make-trace`.

A run is steerable: stop at any step, change it, and the steps that depend on it re-run while the rest stay put.

<details>
<summary>Install by hand</summary>

```bash
git clone https://github.com/AIScientists-Dev/flowtrace.git
cd flowtrace
./scripts/install.sh        # builds + symlinks flowtrace to ~/.local/bin/
```

Update with `git pull && ./scripts/install.sh`. Override the symlink target with `INSTALL_DIR=…`. Building from source or contributing? See [CONTRIBUTING.md](./CONTRIBUTING.md).

</details>

## Examples

**Nine examples** built from popular open-source skills, spanning different domains. Open any one for its flow and a one-command demo in the [examples gallery](docs/EXAMPLES.md):

<div align="center">
<table><tr>
<td align="center" valign="top"><a href="docs/EXAMPLES.md#saas-dd"><img src="docs/assets/examples/feat-saas-dd.png" height="240" alt="SaaS acquisition due diligence flow"></a><br><sub><a href="docs/EXAMPLES.md#saas-dd">SaaS due diligence</a></sub></td>
<td align="center" valign="top"><a href="docs/EXAMPLES.md#security-cicd"><img src="docs/assets/examples/feat-security-cicd.png" height="240" alt="security CI/CD pipeline flow"></a><br><sub><a href="docs/EXAMPLES.md#security-cicd">Security CI/CD gate</a></sub></td>
<td align="center" valign="top"><a href="docs/EXAMPLES.md#distill-mind"><img src="docs/assets/examples/feat-distill-mind.png" height="240" alt="distill a mind into a skill flow"></a><br><sub><a href="docs/EXAMPLES.md#distill-mind">Distill a mind into a skill</a></sub></td>
</tr></table>
</div>

Plus seven more:

- Career: [Tailored Résumé Generator](docs/EXAMPLES.md#tailored-resume)
- Investing: [Comprehensive Stock Analysis](docs/EXAMPLES.md#nvda-decision)
- Research / writing: [Industry Deep-Dive Report](docs/EXAMPLES.md#research-writer)
- Software engineering: [Bug-Fix Learning Loop](docs/EXAMPLES.md#swe-bugfix)
- Growth / marketing: [Weekly Paid-Ads Optimization](docs/EXAMPLES.md#paid-ads)
- Design / decks: [Talk → Magazine Slide Deck](docs/EXAMPLES.md#talk-to-deck)
- Product / releases: [Changelog from Commits](docs/EXAMPLES.md#changelog-from-commits)

## Documentation

A trace is one git repository. `trace.json` declares the steps, their dependencies, and the final deliverable. Each run lives under `runs/<run_id>/`:

```
<trace_root>/
├─ .git/                                    standard git repo, the audit trail
├─ trace.json                              the static plan (steps + deliverable)
├─ scripts/                                 shared code used by 2+ steps
├─ resources/                               shared static material (refs, papers, master data)
├─ steps/<step_id>/
│  ├─ STEP.md                               per-step contract + impl hints
│  ├─ scripts/                              step-local code
│  └─ resources/                            step-local material (figures, PDFs, fixtures)
└─ runs/<run_id>/
   ├─ state.json                            run status (sole source of truth)
   ├─ replies/NNNN.json                     append-only structured-output stream
   └─ <step_id>/                            run-time files (assets + scratch)
```

The same two-name convention (`scripts/` for code that runs, `resources/` for static material that doesn't) appears at both the trace root and inside each step. Anything reused across 2+ steps belongs at the trace root; single-step material stays inside the step folder. `STEP.md` references either with relative paths.

Every CLI write makes one git commit, scoped to exactly the paths it declares: `state.json` plus any `--asset` paths, or the new reply file plus its cited evidence paths. Scratch files stay untracked. The git history is the audit trail, and the UI can time-travel through it.

Steps pass data through files, not parameters: each step writes its output, and a downstream step reads it.

| To learn | Read |
|---|---|
| The idea, in depth | [PHILOSOPHY.md](docs/trace/PHILOSOPHY.md) |
| Driving a trace as an agent | [docs/trace/CLI.md](docs/trace/CLI.md) |
| Making a trace | [skills/make-trace/SKILL.md](skills/make-trace/SKILL.md), or run `/make-trace` |
| The format spec | [SCHEMA.md](docs/trace/SCHEMA.md) and [FIELDS.md](docs/trace/FIELDS.md) |
| All examples | [docs/EXAMPLES.md](docs/EXAMPLES.md) |

---

## Community

**If Flowtrace is useful to you, consider starring the repo. It helps others find it.**

- **Contributing**: see [CONTRIBUTING.md](./CONTRIBUTING.md), and look for [good first issues](https://github.com/AIScientists-Dev/Flowtrace/labels/good%20first%20issue).
- **GitHub Issues**: [report bugs / propose changes](https://github.com/AIScientists-Dev/flowtrace/issues)
- **Discord**: [discord.gg/x9mtbMEx](https://discord.gg/x9mtbMEx)
- **X**: [@morphmind__ai](https://x.com/morphmind__ai?s=11)

---

MIT. See [`LICENSE`](./LICENSE).

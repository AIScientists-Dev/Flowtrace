#!/usr/bin/env bash
# changelog-from-commits: lift a SKILL.md into a trace, then run it once.
#
# The source skill is:
#
#   changelog-generator (SKILL.md)
#   by ComposioHQ: https://github.com/ComposioHQ/awesome-claude-skills
#   licensed Apache-2.0
#
# Its six "What This Skill Does" moves become an 8-node DAG:
#
#   history side:  git_log → scan_history → filter_noise → categorize
#   style side:    changelog_style → read_style                (parallel root)
#   fan-out:       categorize → review_breaking   (breaking + security)
#                  categorize → translate         (everything else)
#   joins:         translate        ← categorize + read_style
#                  format_changelog ← translate + review_breaking + read_style
#   fan-out:       format_changelog → release_blurbs
#
# The run writes the v2.5.0 release notes for Tallyboard (a fictional team task
# board) from 25 raw commits. Along the way it exercises:
#   • a noise gate that drops a feature/revert pair and rescues a security bump
#   • blocked → resume: the security review waits on a disclosure decision,
#     while translate (the other branch) keeps going
#   • error → retry: the formatter's own lint fails once, then passes
#   • a raw-log → changelog before/after via `comparison` evidence
#   • a step memory.md written after the run (the gotcha it learned)
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
source "$HERE/../_lib.sh"

FIX="$HERE/fixtures"

# ── Authoring: scaffold the trace one piece at a time ───────────────────
init_empty
scaffold_deliverable
author_file resources/git_log.txt        "add input: git log v2.4.0..v2.5.0"
author_file resources/CHANGELOG_STYLE.md "add input: house changelog style guide"
author_step scan_history
author_step read_style
author_step filter_noise
author_step categorize
author_step review_breaking
author_step translate
author_step format_changelog
author_step release_blurbs

# ── Orientation: read-side CLI surfaces ───────────────────────────────────
cli validate >/dev/null
cli lint >/dev/null 2>&1 || true
cli show --fmt json    >/dev/null
cli show --fmt mermaid >/dev/null
cli show --fmt ascii   >/dev/null

# ── Start the run ─────────────────────────────────────────────────────────
RUN=$(cli run new --name "Tallyboard v2.5.0 release notes" | tail -1)
cli run show --run "$RUN" >/dev/null
place() { mkdir -p "runs/$RUN/$1"; cp "$FIX/$1/$2" "runs/$RUN/$1/$2"; }

# Copy the raw log into the run so the before/after comparison can cite it
# run-relative (evidence paths forbid '..').
cp resources/git_log.txt "runs/$RUN/git_log.txt"

# ── Parallel roots: history side and style side are independent ──────────
cli step scan_history running --message "reading git log v2.4.0..v2.5.0" >/dev/null
place scan_history commits.json
cli step scan_history done --asset commits.json --message "25 commits in v2.4.0..v2.5.0, 1 flagged breaking" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "25 commits in v2.4.0..v2.5.0",
  "status": "complete",
  "support": [
    "22 follow Conventional Commits; 3 don't (a merge, a revert, one free-form subject)",
    "1 breaking change flagged: #420 has the ! marker and a BREAKING CHANGE footer",
    "8 commits carry details in the body that the subject doesn't show"
  ],
  "evidence": [
    { "type": "document", "path": "scan_history/commits.json", "title": "commits.json" },
    { "type": "check", "label": "count matches git", "passed": true, "expected": 25, "actual": 25 },
    { "type": "check", "label": "bodies captured", "passed": true, "expected": "25/25", "actual": "25/25" }
  ],
  "checkpoint": { "step_id": "scan_history" }
}
EOF

cli step read_style running --message "reading CHANGELOG_STYLE.md" >/dev/null
place read_style style_rules.json
cli step read_style done --asset style_rules.json --message "5 sections, voice rules, 6 lint checks" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "style guide → 6 checkable rules",
  "status": "complete",
  "findings": [
    { "title": "header", "detail": "## v2.5.0 · March 13, 2026 (month name, not ISO date)" },
    { "title": "section order", "detail": "Breaking → Features → Improvements → Fixes → Security; omit empty ones" },
    { "title": "voice", "detail": "talk to 'you', lead with the benefit, no internal names, numbers only if measured" },
    { "title": "security", "detail": "impact + fixed, never exploit steps; credit reporters only with consent" }
  ],
  "evidence": [
    { "type": "document", "path": "read_style/style_rules.json", "title": "style_rules.json" },
    { "type": "check", "label": "every mechanical rule is in lint_checks", "passed": true, "expected": 6, "actual": 6 }
  ],
  "checkpoint": { "step_id": "read_style" }
}
EOF

# ── Gate: drop what users will never notice ───────────────────────────────
cli step filter_noise running --message "separating user-facing commits from internal ones" >/dev/null
place filter_noise kept_commits.json
cli step filter_noise done --asset kept_commits.json --message "14 kept, 11 dropped (each with a reason)" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "14 user-facing commits kept, 11 dropped",
  "status": "complete",
  "findings": [
    { "title": "feature + revert cancel out", "detail": "#408 (AI summaries) and its revert #414 are both dropped: users never saw it" },
    { "title": "dependency bump rescued", "detail": "#421 looks like a chore, but its body cites a security advisory: kept for the security review" },
    { "title": "untyped subject kept", "detail": "'improve drag and drop on touch screens' (#409) is user-visible" }
  ],
  "support": [
    "dropped: 2 tests, 1 refactor, 1 CI, 1 docs typo, 1 dev dependency, 1 merge, 2 release chores, 1 feature/revert pair"
  ],
  "evidence": [
    { "type": "document", "path": "filter_noise/kept_commits.json", "title": "kept_commits.json (with the dropped log)" },
    { "type": "check", "label": "scanned = kept + dropped", "passed": true, "expected": 25, "actual": "14 + 11" },
    { "type": "check", "label": "every decision has a reason", "passed": true, "expected": "25/25", "actual": "25/25" }
  ],
  "checkpoint": { "step_id": "filter_noise" }
}
EOF

cli step categorize running --message "grouping the 14 kept commits" >/dev/null
place categorize categorized.json
cli step categorize done --asset categorized.json --message "3 features · 3 improvements · 5 fixes · 1 breaking · 2 security" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "14 changes in 5 groups, 3 judgment calls",
  "status": "complete",
  "support": [
    "3 new features, 3 improvements, 5 fixes, 1 breaking change, 2 security entries"
  ],
  "findings": [
    { "title": "#415 feature → improvement", "detail": "tagged feat, but it extends the existing CSV export" },
    { "title": "#418 fix → security", "detail": "other devices stayed signed in after a password change" },
    { "title": "#421 noise → security", "detail": "patches an advisory about token expiry checks" }
  ],
  "evidence": [
    { "type": "document", "path": "categorize/categorized.json", "title": "categorized.json" },
    { "type": "check", "label": "each kept commit in exactly one group", "passed": true, "expected": 14, "actual": 14 }
  ],
  "checkpoint": { "step_id": "categorize" }
}
EOF

# ── Fan-out, branch 1: breaking + security review blocks on a human call ──
cli step review_breaking running --message "confirming #420 and the 2 security entries" >/dev/null
place review_breaking breaking_security.md
cli step review_breaking blocked --message "needs a maintainer: disclosure wording and reporter credit for #418" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "waiting on a disclosure decision for #418",
  "status": "blocked",
  "note": "#420 (breaking) and #421 (dependency advisory) are written up. #418 was reported by an external researcher, and the style guide says credit needs their consent.",
  "suggestions": [
    "publish impact + fix only, thank the reporter without a name",
    "hold #418 out of the notes until the self-hosted release ships",
    "name the reporter (only if they agreed)"
  ],
  "evidence": [
    { "type": "document", "path": "review_breaking/breaking_security.md", "title": "breaking_security.md (draft)" },
    { "type": "check", "label": "#420 has a What to do line", "passed": true, "actual": "yes" },
    { "type": "check", "label": "#421 advisory applies to our config", "passed": true, "expected": "default clock-skew", "actual": "default clock-skew" },
    { "type": "check", "label": "#418 disclosure decided", "passed": false, "expected": "decision", "actual": "pending" }
  ],
  "checkpoint": { "step_id": "review_breaking" }
}
EOF

# ── Fan-out, branch 2: translate doesn't wait for the review ─────────────
cli step translate running --message "rewriting changes in the reader's words" >/dev/null
place translate entries.json
cli step translate done --asset entries.json --message "13 entries written; #420 left to review_breaking" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "13 entries in plain language",
  "status": "complete",
  "findings": [
    { "title": "perf(sync): batch delta uploads", "detail": "→ Large boards sync about 2x faster (measured on a board with 2,000 cards)" },
    { "title": "fix(upload): images over 10 MB failed silently", "detail": "→ Fixed large images failing to upload without a message; you can now compress them and retry" },
    { "title": "chore(deps): bump jwt-verify", "detail": "→ Expired sign-ins are now always refused" }
  ],
  "evidence": [
    { "type": "document", "path": "translate/entries.json", "title": "entries.json" },
    { "type": "check", "label": "fix lines at most 120 characters", "passed": true, "expected": "≤ 120", "actual": "longest 105" },
    { "type": "check", "label": "no internal names", "passed": true, "expected": "0", "actual": "0" }
  ],
  "takeaway": "the breaking change is left to review_breaking, which owes readers upgrade steps",
  "checkpoint": { "step_id": "translate" }
}
EOF

# ── Resume the review once the maintainer decides ─────────────────────────
cli step review_breaking running --message "decision: impact + fix only, reporter not named" >/dev/null
cli step review_breaking done --asset breaking_security.md --message "1 breaking + 2 security entries written" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "upgrade and security wording ready",
  "status": "complete",
  "support": [
    "#420: v1 webhooks removed; What to do points at /api/v2/webhooks (since v2.2.0)",
    "#418: impact + fix only, the reporter is thanked without a name",
    "#421: user-visible effect only, no library name"
  ],
  "evidence": [
    { "type": "document", "path": "review_breaking/breaking_security.md", "title": "breaking_security.md" },
    { "type": "appendix", "title": "decision log",
      "markdown": "**Blocked:** how much of #418 to disclose, and whether to name the reporter.\n\n**Decision:** state the impact and the fix only. The reporter asked not to be named." },
    { "type": "check", "label": "no reproduction steps", "passed": true, "actual": "none" }
  ],
  "checkpoint": { "step_id": "review_breaking" }
}
EOF

# ── Three-way join: format, lint, retry ───────────────────────────────────
cli step format_changelog running --message "assembling v2.5.0" >/dev/null
cli step format_changelog error --message "lint failed: 2 of 6 checks" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "lint failed on the first draft",
  "status": "error",
  "note": "both failures are formatting, not wording, so they are fixed here rather than upstream",
  "evidence": [
    { "type": "check", "label": "header matches the pattern", "passed": false, "expected": "## v2.5.0 · March 13, 2026", "actual": "## v2.5.0 · 2026-03-13" },
    { "type": "check", "label": "every entry ends with a PR link", "passed": false, "expected": "14/14", "actual": "13/14 (#420 missing)" },
    { "type": "check", "label": "sections in order", "passed": true, "actual": "yes" },
    { "type": "check", "label": "fix lines at most 120 characters", "passed": true, "actual": "longest 105" }
  ],
  "suggestions": ["rewrite the date with the month name", "append the #420 link after its What to do line"],
  "checkpoint": { "step_id": "format_changelog" }
}
EOF
cli step format_changelog running --message "fixed the header date and the #420 link; re-linting" >/dev/null
place format_changelog CHANGELOG.md
cli step format_changelog done --asset CHANGELOG.md --message "6/6 lint checks pass; 14 entries in 5 sections" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "v2.5.0 release notes ready",
  "status": "complete",
  "support": [
    "25 raw commits → 14 entries in 5 sections",
    "breaking change first, with a What to do line",
    "lint clean on retry"
  ],
  "evidence": [
    { "type": "comparison", "title": "raw git log → user-facing changelog",
      "left":  { "label": "git log v2.4.0..v2.5.0", "path": "git_log.txt" },
      "right": { "label": "CHANGELOG.md (v2.5.0)", "path": "format_changelog/CHANGELOG.md" } },
    { "type": "document", "path": "format_changelog/CHANGELOG.md", "title": "CHANGELOG.md" },
    { "type": "check", "label": "lint checks", "passed": true, "expected": "6/6", "actual": "6/6" },
    { "type": "check", "label": "every categorized change appears once", "passed": true, "expected": 14, "actual": 14 }
  ],
  "checkpoint": { "step_id": "format_changelog" }
}
EOF

# ── Fan-out from the finished changelog ───────────────────────────────────
cli step release_blurbs running --message "writing app store, email and social copy" >/dev/null
place release_blurbs blurbs.md
cli step release_blurbs done --asset blurbs.md --message "4 blurbs, all derived from CHANGELOG.md" >/dev/null
cli reply <<'EOF' >/dev/null
{
  "headline": "4 release blurbs",
  "status": "complete",
  "support": [
    "app store, email, social post and GitHub release title",
    "email carries a heads-up about the removed v1 webhooks"
  ],
  "evidence": [
    { "type": "document", "path": "release_blurbs/blurbs.md", "title": "blurbs.md" },
    { "type": "check", "label": "app store text under 500 characters", "passed": true, "expected": "< 500", "actual": 358 },
    { "type": "check", "label": "social post under 280 characters", "passed": true, "expected": "< 280", "actual": 236 },
    { "type": "check", "label": "no claim beyond the changelog", "passed": true, "actual": "yes" }
  ],
  "checkpoint": { "step_id": "release_blurbs" }
}
EOF

# ── Deliverable ───────────────────────────────────────────────────────────
cli deliverable running --message "packaging the release notes" >/dev/null
cli deliverable done \
  --asset format_changelog/CHANGELOG.md \
  --asset release_blurbs/blurbs.md \
  --asset review_breaking/breaking_security.md \
  --asset filter_noise/kept_commits.json \
  --message "Tallyboard v2.5.0 release notes + blurbs + upgrade/security notes + filter log" >/dev/null

# ── Learn: record the gotcha this run surfaced ────────────────────────────
author_file steps/filter_noise/memory.md \
    "memory: filter_noise · read dependency-bump bodies; feature/revert pairs cancel out"

echo
echo "=== final state ==="
"$BIN" run show --run "$RUN"
echo
echo "Done. View: flowtrace serve  →  http://localhost:3000/traces/changelog-from-commits"

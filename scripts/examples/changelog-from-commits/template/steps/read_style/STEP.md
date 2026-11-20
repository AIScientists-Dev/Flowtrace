---
name: read_style
description: Turn the project's changelog style guide into concrete, checkable voice and format rules.
reads:
  - inputs.changelog_style
writes:
  - runs/<run>/read_style/style_rules.json
---

# Read Style Guide

Source skill: **changelog-generator** (Follows Best Practices, plus the "With Custom Guidelines" usage: `CHANGELOG_STYLE.md`).

The style-side ingest. It runs in parallel with the whole history side because it doesn't depend on any commit.

## What this step does

Read the house style guide (`resources/CHANGELOG_STYLE.md` in the demo) and turn its prose into rules two later steps can apply and one can check:

- `translate` uses the **voice** rules to write each entry.
- `format_changelog` uses the **header, section and entry-shape** rules to assemble the release, and the **lint checks** to verify it.

## Output: `style_rules.json`

```json
{
  "header": { "pattern": "## v{version} · {Month D, YYYY}", "example": "..." },
  "sections": ["### ⚠️ Breaking Changes", "### ✨ New Features", "..."],
  "omit_empty_sections": true,
  "voice": ["talk to the reader as 'you'", "..."],
  "entry_shapes": { "features": "- **Name**: one sentence.", "fixes": "- Fixed ... (at most 120 characters)" },
  "pr_link": "([#{n}](https://github.com/<org>/<repo>/pull/{n}))",
  "breaking": "...",
  "security": ["..."],
  "lint_checks": ["header matches the pattern", "fix lines at most 120 characters", "..."]
}
```

## How to approach it

- Every rule that can be checked mechanically (header pattern, section order, line length, banned words, PR links) also goes into `lint_checks`. That list is what makes `format_changelog`'s self-check possible.
- If the project has no style guide, write down the defaults you are choosing (the source skill's example format: emoji section headings, bold feature names) so the choice is visible and can be overruled on the next run.

## Contributes to

`translate` (voice) and `format_changelog` (format + lint).

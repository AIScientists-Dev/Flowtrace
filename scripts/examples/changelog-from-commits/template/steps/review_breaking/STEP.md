---
name: review_breaking
description: Confirm each breaking change and security fix, then write its upgrade steps or disclosure-safe wording.
reads:
  - runs/<run>/categorize/categorized.json
  - runs/<run>/scan_history/commits.json
writes:
  - runs/<run>/review_breaking/breaking_security.md
---

# Breaking & Security Review

Source skill: **changelog-generator** (the "breaking changes" and "security" groups of Categorizes Changes).

The source skill lists these as two more categories. This trace gives them **their own lane** because they carry obligations ordinary entries don't: a breaking change owes readers exact upgrade steps, and a security fix owes them honest impact without a how-to for attackers. Those calls often need a maintainer, so this is the step that is allowed to block.

## What this step does

For each breaking change:

- Confirm it really breaks something (marker, footer, removed or renamed public surface).
- Name who is affected and the exact replacement, with the version it has been available since.
- Write the changelog wording, including a **What to do** line.

For each security entry:

- State the impact and that it is fixed.
- For dependency updates: check that the advisory actually applied to this project's configuration. If it didn't, it goes back to being a routine bump.
- Decide disclosure and credit. **Block** and ask a maintainer when the answer isn't written down (embargo, reporter consent).

## Output: `breaking_security.md`

One section per change with: confirmation, who is affected, replacement or impact, the final changelog wording, and a decision log for anything that was escalated.

## How to approach it

- Never include reproduction steps for a security bug, even if the commit message has them.
- Credit a reporter by name only with their consent; otherwise thank "the external researcher who reported this".
- Integrators read Breaking Changes word by word. Name the exact old and new paths.

## Contributes to

`format_changelog`, which places this wording verbatim.

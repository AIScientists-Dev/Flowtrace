---
name: format_changelog
description: Assemble the release section of CHANGELOG.md in the house format, then lint it against the style rules before calling it done.
reads:
  - runs/<run>/translate/entries.json
  - runs/<run>/review_breaking/breaking_security.md
  - runs/<run>/read_style/style_rules.json
writes:
  - runs/<run>/format_changelog/CHANGELOG.md
---

# Format Changelog

Source skill: **changelog-generator** (Formats Professionally, plus the Tips: "Review and adjust the generated changelog before publishing", "Save output directly to CHANGELOG.md").

The three-way **join**: user entries from `translate`, upgrade and security wording from `review_breaking`, and the format rules from `read_style`.

## What this step does

1. Write the release header in the style's pattern.
2. Add the sections in the style's order, leaving out empty ones. Breaking Changes and Security wording come from `breaking_security.md` as written.
3. End every entry with its PR link.
4. **Lint** the result against every item in `lint_checks`. If any check fails, report `error` with the failing lines, fix them, and run the lint again. Only a clean lint is `done`.

## Output: `CHANGELOG.md`

The new release section, ready to paste at the top of the project's `CHANGELOG.md`:

```markdown
## v2.5.0 · March 13, 2026

### ⚠️ Breaking Changes
- **Legacy webhooks removed**: ...
  **What to do**: ...

### ✨ New Features
- **Recurring tasks**: ... ([#412](...))
```

## How to approach it

- Don't rewrite entries here. If an entry breaks a rule, the fix belongs upstream (`translate` for voice and length, `review_breaking` for upgrade or disclosure wording), and the lint error should say so.
- Check the counts: every change in `categorized.json` appears exactly once.

## Contributes to

`release_blurbs`, and the deliverable.

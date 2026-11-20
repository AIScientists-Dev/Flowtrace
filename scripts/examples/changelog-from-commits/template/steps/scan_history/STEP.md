---
name: scan_history
description: Pull every commit in the release range and normalize it into a list the rest of the trace can filter and group.
reads:
  - inputs.git_log
writes:
  - runs/<run>/scan_history/commits.json
---

# Scan History

Source skill: **changelog-generator** (Scans Git History, plus the "With Specific Date Range" usage).

The history-side ingest. Everything downstream reasons over this list, so it has to be complete: a commit missed here never reaches the changelog.

## What this step does

Pick the release range, then read every commit in it. The range is usually `<previous tag>..<new tag>` (for the demo, `git log v2.4.0..v2.5.0`); a date range works too (`git log --since=2026-03-01 --until=2026-03-15`).

For each commit, record the subject *and* the body. Bodies matter: breaking-change footers and security notes often live there, not in the subject.

## Output: `commits.json`

```json
{
  "range": { "from": "v2.4.0", "to": "v2.5.0", "command": "git log v2.4.0..v2.5.0" },
  "count": 25,
  "commits": [
    { "sha": "cb3ca0d", "date": "2026-03-12", "author": "...", "subject": "feat(api)!: ...",
      "type": "feat", "scope": "api", "breaking": true, "pr": 420,
      "is_merge": false, "body": "BREAKING CHANGE: ..." }
  ]
}
```

## How to approach it

- Parse Conventional Commit subjects (`type(scope)!: summary`) when they are there, but don't depend on them: real histories mix styles. A subject without a type gets `"type": null`, not a guess.
- `breaking` is true for a `!` after the type **or** a `BREAKING CHANGE:` footer in the body.
- Keep merge commits and reverts in the list and mark them (`is_merge`, `type: "revert"` plus `reverts`). Deciding to drop them is `filter_noise`'s job, not this step's.
- `count` must equal the number of commits git reports for the range. Check it.

## Contributes to

`filter_noise`, and through it every other step on the history side.

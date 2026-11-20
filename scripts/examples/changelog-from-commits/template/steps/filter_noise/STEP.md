---
name: filter_noise
description: Drop commits users will never notice (refactors, tests, CI, chores, reverted work) and log why each one was dropped.
reads:
  - runs/<run>/scan_history/commits.json
writes:
  - runs/<run>/filter_noise/kept_commits.json
---

# Filter Noise

Source skill: **changelog-generator** (Filters Noise: "Excludes internal commits (refactoring, tests, etc.)").

This is a **gate**: it comes before categorization on purpose. Grouping 25 commits and then throwing half of them away wastes the categorizer's judgment on work that was never going to ship in the notes.

## What this step does

Split the commit list into `kept` (user-visible) and `dropped` (internal). Every decision gets a one-line reason, so a human can scan the dropped list and pull something back.

## Output: `kept_commits.json`

```json
{
  "summary": { "scanned": 25, "kept": 14, "dropped": 11 },
  "kept":    [ { "sha": "...", "pr": 412, "subject": "...", "reason": "user-facing type (feat)" } ],
  "dropped": [ { "sha": "...", "subject": "...", "reason": "tests only" } ]
}
```

## How to approach it

- Usually dropped: `refactor`, `test`, `ci`, `build`, `style`, contributor `docs`, release housekeeping, merge commits (their changes are already listed one by one), and dev-only dependency bumps.
- **A feature and its revert inside the same range cancel out.** Drop both; users never saw it.
- A subject with no conventional type is not noise by default. Read it: "improve drag and drop on touch screens" is user-visible.
- **Read the body of every dependency bump before dropping it.** A bump that patches a security advisory is kept and passed on for the security review.
- `scanned` must equal `kept + dropped`. Nothing silently disappears.

## Contributes to

`categorize`. The `dropped` list also ships in the deliverable as the audit trail.

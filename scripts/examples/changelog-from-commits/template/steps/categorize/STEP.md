---
name: categorize
description: Group the kept commits into new features, improvements, fixes, breaking changes, and security; record every judgment call.
reads:
  - runs/<run>/filter_noise/kept_commits.json
  - runs/<run>/scan_history/commits.json
writes:
  - runs/<run>/categorize/categorized.json
---

# Categorize Changes

Source skill: **changelog-generator** (Categorizes Changes: "features, improvements, bug fixes, breaking changes, security").

## What this step does

Put every kept commit in exactly one of the five groups. The commit type is a hint, not the answer: the question is what the change means for the person reading the notes.

## Output: `categorized.json`

```json
{
  "breaking":     [ { "pr": 420, "sha": "...", "subject": "...", "why": "..." } ],
  "features":     [ { "pr": 412, "sha": "...", "subject": "..." } ],
  "improvements": [ ... ],
  "fixes":        [ ... ],
  "security":     [ ... ],
  "judgment_calls": ["#415 moved from features to improvements", "..."]
}
```

## How to approach it

- **Feature vs improvement:** a feature is something you could not do before; an improvement makes something you already do better. A `feat` that extends an existing feature (more columns in an export) is an improvement.
- **Fix vs security:** if the bug let someone see, keep or do something they shouldn't (sessions surviving a password change, tokens outliving their expiry), it is security, whatever the commit type says.
- **Breaking** comes from `scan_history`'s `breaking` flag, plus anything that removes or renames something users or integrators rely on.
- Each commit lands in exactly one group. Add a `why` whenever the group differs from what the commit type suggests, and list it in `judgment_calls`.

## Contributes to (fan-out)

- `review_breaking`: takes `breaking` + `security`.
- `translate`: takes the rest.

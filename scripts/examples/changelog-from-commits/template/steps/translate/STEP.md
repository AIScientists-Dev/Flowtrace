---
name: translate
description: Rewrite each categorized change as a short, plain-language entry about what the user can now do or what stopped going wrong.
reads:
  - runs/<run>/categorize/categorized.json
  - runs/<run>/read_style/style_rules.json
  - runs/<run>/scan_history/commits.json
writes:
  - runs/<run>/translate/entries.json
---

# User-Friendly Rewrite

Source skill: **changelog-generator** (Translates Technical → User-Friendly: "Converts developer commits into customer language").

This is a **join**: it needs the categorized changes *and* the voice rules from `read_style`. The same commit reads differently in a consumer app's notes and in a developer tool's.

## What this step does

Write one entry per change in features, improvements, fixes and security. Breaking changes are left to `review_breaking`, which writes them with their upgrade steps.

## Output: `entries.json`

```json
{
  "style": "read_style/style_rules.json",
  "sections": {
    "features":     [ { "pr": 412, "text": "**Recurring tasks**: Set a task to repeat ..." } ],
    "improvements": [ ... ],
    "fixes":        [ { "pr": 399, "text": "Fixed large images (over 10 MB) failing to upload ..." } ],
    "security":     [ ... ]
  },
  "left_to_review_breaking": [420]
}
```

## How to approach it

- Start from the benefit, not the code: "perf(sync): batch delta uploads" becomes "Large boards sync about 2x faster".
- Numbers only when the commit measured them, and say how ("measured on a board with 2,000 cards").
- No internal names: no modules, libraries or protocol words. The reader doesn't know what a "delta upload" is.
- Read the commit body as well as the subject: it often holds the user-visible detail ("offer to compress the image").
- Keep within the style's length limits here, so `format_changelog` doesn't have to cut.

## Contributes to

`format_changelog`.

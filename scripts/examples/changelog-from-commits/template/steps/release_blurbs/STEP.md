---
name: release_blurbs
description: Derive short release copy (app store, email, social post, GitHub release title) from the finished changelog.
reads:
  - runs/<run>/format_changelog/CHANGELOG.md
writes:
  - runs/<run>/release_blurbs/blurbs.md
---

# Release Blurbs

Source skill: **changelog-generator** (Related Use Cases: GitHub release notes, app store update descriptions, email updates, social media posts).

A **fan-out** from the finished changelog. It reads only `CHANGELOG.md`, never the raw commits, so the short copy can't promise anything the reviewed notes don't.

## What this step does

Write one block per channel:

- **App store "What's New"**: under 500 characters, plain text, no links.
- **Email update**: subject line plus the top three changes. If there is a breaking change, add a one-line heads-up with the migration link.
- **Social post**: under 280 characters, with the link to the full notes.
- **GitHub release title**: version plus the two or three headline changes; mention a breaking change in parentheses.

## Output: `blurbs.md`

One Markdown section per channel, in the order above.

## How to approach it

- Pick the headline changes by how many people they reach, not by how much code they took.
- Reuse the changelog's wording and numbers ("about 2x faster"); don't round up.
- Security entries go in the app store text when they change what users see (being signed out of other devices). They never go in social posts as a selling point.

## Contributes to

The deliverable.

# Tallyboard changelog style

How we write the release notes on tallyboard.example/changelog. Tallyboard is a
fictional team task board; this guide is demo material for the
`changelog-from-commits` reference trace.

## Who reads it

People who use Tallyboard every day, not the people who build it. Most of them
skim the headings and read one line per change. Integrators (people who use our
API) read the Breaking Changes section closely.

## Release header

`## v<version> · <Month D, YYYY>`, for example `## v2.4.0 · February 6, 2026`.

## Sections, in this order

1. `### ⚠️ Breaking Changes` (only when there are any)
2. `### ✨ New Features`
3. `### 🔧 Improvements`
4. `### 🐛 Fixes`
5. `### 🔒 Security`

Leave out empty sections.

## Voice

- Talk to the reader as "you". Present tense.
- Lead with what the reader can now do, or what stopped going wrong, not with
  what we changed in the code.
- No internal names: no component, module or library names, no "ops",
  "reducers" or "endpoints" outside the Breaking Changes section.
- Numbers only when we measured them. Round them and say how you measured
  ("about 2x faster on large boards").

## Entry shapes

- **New Features**: `- **Name**: one sentence on what you can do with it.`
- **Improvements**: same shape as features.
- **Fixes**: one plain sentence, starting with "Fixed". At most 120 characters,
  not counting the PR link.
- Every entry ends with its PR link: `([#412](https://github.com/tallyboard/app/pull/412))`.

## Breaking changes

Every breaking change gets a **What to do** line with the exact replacement,
and the version since which the replacement has been available.

## Security

- Say what the impact was and that it is fixed. Never describe how to exploit
  it.
- Credit external reporters only if they agreed to be named.
- A dependency update counts as a security entry only when it patches a
  published advisory that affected Tallyboard.

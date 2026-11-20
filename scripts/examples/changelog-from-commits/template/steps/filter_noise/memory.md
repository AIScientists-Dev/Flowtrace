# memory · filter_noise

## Read the body of every dependency bump

Rule from the v2.5.0 run: a `chore(deps)` subject looks like noise, but
`#421 bump jwt-verify 3.1.0 → 3.1.2` patched an advisory that applied to us
(expired tokens accepted for up to 24 hours). It only surfaced because the
release notes were in the commit body.

- Body mentions "security", "advisory", "CVE" or "GHSA": **keep**, reason
  "kept for the security review". `categorize` and `review_breaking` decide
  whether it really belongs in Security.
- Dev-only bumps (`chore(deps-dev)`, linters, test tools): drop.
- Everything else: drop, but say which package in the reason, so a human
  scanning `dropped` can spot one that matters.

## A feature and its revert inside the range cancel out

`#408 experimental AI task summaries` shipped and was reverted by `#414` in the
same release. Users never saw it. Drop **both**, and say so in both reasons,
so nobody "restores" one half later.

A revert of something from an *earlier* release is different: that is a
user-visible removal. Keep it and let `categorize` decide whether it is
breaking.

## Untyped subjects are not noise by default

About one commit in ten has no conventional type (`improve drag and drop on
touch screens`). Read it; most are user-visible improvements.

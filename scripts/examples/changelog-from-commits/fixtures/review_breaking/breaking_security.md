# Breaking changes and security review · v2.5.0

Three changes need more than a one-line entry. Each one is confirmed below with
the wording the changelog will use.

## ⚠️ Breaking: legacy v1 webhooks endpoint removed (#420)

- **Confirmed breaking.** The commit carries the `!` marker and a
  `BREAKING CHANGE:` footer, and it removes a public API route.
- **Who is affected:** integrations that still call `/api/v1/webhooks`.
- **Replacement:** `/api/v2/webhooks`, available since v2.2.0. v2 payloads are
  signed, so receivers must verify the signature.
- **Changelog wording:**

  > **Legacy webhooks removed**: The v1 webhooks endpoint (`/api/v1/webhooks`)
  > no longer works.
  > **What to do**: Move your integrations to `/api/v2/webhooks`, available
  > since v2.2.0. v2 payloads are signed; see the
  > [migration guide](https://tallyboard.example/docs/webhooks).

## 🔒 Security: other devices stayed signed in after a password change (#418)

- **Impact:** after you changed your password, sessions on your other devices
  stayed signed in.
- **Fixed:** a password change now ends every other session.
- **Disclosure (decided while this step was blocked):** state the impact and
  the fix only, with no reproduction details. The reporter asked not to be
  named, so the entry thanks "the external researcher who reported this".

## 🔒 Security: token library patched (#421)

- **Advisory:** `jwt-verify` 3.1.0 accepted expired tokens for up to 24 hours
  when the clock-skew option was left at its default.
- **Did it affect Tallyboard?** Yes. Tallyboard never sets the clock-skew
  option, so it ran with the vulnerable default. Under the style guide's rule,
  this dependency update is a Security entry, not noise.
- **Changelog wording:** describe the user-visible effect ("expired sign-ins
  are now always refused"), not the library name.

## Decision log

| When | Question | Decision |
|---|---|---|
| blocked | How much of #418 do we disclose, and do we name the reporter? | Impact and fix only; no name, per the reporter's request. |
| resumed | Does #421 belong in Security? | Yes: the advisory applied to our configuration. |

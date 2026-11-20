## v2.5.0 · March 13, 2026

### ⚠️ Breaking Changes

- **Legacy webhooks removed**: The v1 webhooks endpoint (`/api/v1/webhooks`) no longer works.
  **What to do**: Move your integrations to `/api/v2/webhooks`, available since v2.2.0. v2 payloads are signed; see the [migration guide](https://tallyboard.example/docs/webhooks). ([#420](https://github.com/tallyboard/app/pull/420))

### ✨ New Features

- **Recurring tasks**: Set a task to repeat daily, weekly or monthly; the next one appears when you complete it. ([#412](https://github.com/tallyboard/app/pull/412))
- **Search inside attachments**: Search now finds words inside attached PDF and Word files, not just card titles. ([#398](https://github.com/tallyboard/app/pull/398))
- **Keyboard shortcuts**: Press ? to see every shortcut and get around your boards without the mouse. ([#405](https://github.com/tallyboard/app/pull/405))

### 🔧 Improvements

- **Faster sync**: Large boards sync about 2x faster (measured on a board with 2,000 cards). ([#401](https://github.com/tallyboard/app/pull/401))
- **Touch-friendly drag and drop**: Long-press a card to pick it up; the board scrolls for you near the edges. ([#409](https://github.com/tallyboard/app/pull/409))
- **Richer CSV export**: Your exports now include custom fields. ([#415](https://github.com/tallyboard/app/pull/415))

### 🐛 Fixes

- Fixed large images (over 10 MB) failing to upload without a message; you can now compress them and retry. ([#399](https://github.com/tallyboard/app/pull/399))
- Fixed reminders arriving an hour early after a daylight saving time change. ([#403](https://github.com/tallyboard/app/pull/403))
- Fixed the notification badge not clearing after you read your notifications. ([#407](https://github.com/tallyboard/app/pull/407))
- Fixed line breaks disappearing when you paste text from Google Docs. ([#411](https://github.com/tallyboard/app/pull/411))
- Fixed dates showing in US format for people using English (UK). ([#422](https://github.com/tallyboard/app/pull/422))

### 🔒 Security

- **Password changes sign out other devices**: Changing your password now signs you out everywhere else. Thanks to the external researcher who reported this. ([#418](https://github.com/tallyboard/app/pull/418))
- **Expired sign-ins rejected**: Expired sign-ins are now always refused; before, some stayed valid for up to 24 hours. ([#421](https://github.com/tallyboard/app/pull/421))

# Serverless demo — Mini App on Telegram's own backend

A Mini App whose backend runs on [Telegram
Serverless](https://core.telegram.org/bots/serverless): no VPS, no webhooks,
no self-hosted API. The frontend is Rust (`src/main.rs`, built with
[Trunk](https://trunkrs.dev)); the backend is the `tgcloud/` folder — a
JavaScript endpoint deployed to Telegram's infrastructure. One `npm run
deploy` ships both, and the app is served at `https://app<app_id>.tgcloud.ai/`.

The demo endpoint (`tgcloud/endpoints/getVisits.js`) keeps a per-user visit
counter in the platform's built-in database. The frontend calls it through
`telegram_webapp_sdk::api::serverless::call` — the typed wrapper over
`Telegram.WebApp.Serverless.call` — and renders either the counter or the
decoded `EndpointError` with its recovery hint.

## Prerequisites

- Node.js 18+, [Trunk](https://trunkrs.dev) (`cargo install trunk`), Rust.
- A bot registered with [@BotFather](https://t.me/BotFather).

## Setup

1. In @BotFather, open your bot → **Serverless** → turn it on.
2. In the same menu: **Serverless → CLI Access → Access token** (format
   `app<id>:<secret>`).
3. From this directory, log the CLI in (the token is stored in the
   git-ignored `.tgcloud/` folder — never commit it):

   ```bash
   npx tgcloud login
   ```

4. Build the frontend and deploy everything:

   ```bash
   npm run deploy     # trunk build --release && npx tgcloud push
   ```

5. Create the database table (the endpoint stores its counter there):

   ```bash
   npm run migrate    # npx tgcloud migrate — confirms the `visits` table
   ```

6. In @BotFather: **Bot Settings → Menu Button → Configure menu button** (or
   /newwebapp) → set the Mini App URL to the `https://app<app_id>.tgcloud.ai/`
   address `push` printed.

Open the Mini App from the bot in Telegram and press the button: each press
calls `getVisits`, which increments and returns your personal counter.

## Development without Telegram

```bash
trunk serve          # http://localhost:8082
```

Outside Telegram the SDK mock takes over, so the UI boots; the Serverless
call then fails with a typed `EndpointError` whose message explains that the
bridge is unavailable — expected, and a demonstration of the error decoding.

## How it maps to the SDK

| Piece | Where |
| --- | --- |
| Typed endpoint call | `telegram_webapp_sdk::api::serverless::call::<NoInput, Visits>` in `src/main.rs` |
| Error decoding + hints | `EndpointError` (`kind`, `status`, `hint()`) printed into `#status` |
| Verified launch identity | the endpoint trusts `ctx.initData` — the platform validated it before your code ran |
| Platform database | `tgcloud/schema.js` + `db` in `tgcloud/endpoints/getVisits.js` |

## Notes

- Telegram does not publish pricing for Serverless yet (early access since
  October 2026); treat it as free-for-now and re-check the docs before
  production use.
- Only `.js` files under `tgcloud/` and the `dist/` build output are
  deployed; everything else in this folder stays local.
- Do not add your own files under `dist/api/` — that path namespace belongs
  to the endpoints.

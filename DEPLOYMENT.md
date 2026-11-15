# Deploying Merzah to Hostless (or any Docker host)

Merzah is a Leptos SSR app on actix-web. It builds into a single binary
(`merzah`) plus static assets in `target/site/`. Hostless builds the
`Dockerfile` in this repo and runs it as an always-on free app.

## What the audit found

- No Dockerfile, no `.dockerignore`. Added both.
- `Cargo.toml` metadata pins `site-addr = 127.0.0.1:3000` and `env = "DEV"`.
  Fine for local dev; in the container, `LEPTOS_SITE_ADDR` and `LEPTOS_ENV`
  env vars override at runtime (both are read by leptos_config).
- The server binds whatever `LEPTOS_SITE_ADDR` says, so the container must set
  it to `0.0.0.0:$PORT` or Hostless cannot reach the app.
- No health endpoint exists. Not required: Hostless probes your public URL,
  and `/` returns the SSR page with a 200. If you want a dedicated `/healthz`,
  say so and I'll add one.
- The release build failed with "queries overflow the depth limit". `src/lib.rs`
  already had `#![recursion_limit = "256"]` but `src/main.rs` (the SSR bin)
  didn't; added the same attribute there. Required for any release build.
- `reqwest` uses the default native-tls, so OpenSSL is required at build and
  runtime. The Dockerfile installs `libssl-dev` (build) and `libssl3` (runtime).
- `Cargo.lock` was gitignored. It is an executable, not a library; the lock is
  now tracked so builds are reproducible. If you'd rather not commit it, say so.
- Nothing in the app reads `.env` at build time; `dotenv()` is optional at
  runtime. All config comes from environment variables, which you set in the
  Hostless dashboard. No secrets are baked into the image.
- Logging goes to stdout (`tracing_subscriber` fmt), which Hostless captures.
  No file logging, no log rotation needed.
- The app runs in-process cron jobs (session cleanup, event rotation). These
  only fire while the process is alive. Hostless free apps never sleep, so
  they work. Do not migrate this to a sleeping host without moving the cron
  jobs out.

## Hostless free plan: what to expect

- Shared allowance: 0.5 vCPU, 1 GB RAM, 512 MB storage across the account.
- Always-on, no cold starts. Builds from your repo or Dockerfile, free.
  Your Dockerfile path is what Hostless should use.
- Logs kept 30 days. Custom domains and TLS included.
- Above the allowance it bills per vCPU-hour / GB-hour, so don't set
  replicas above 1.

## Environment variables to set in Hostless

Required (database):

- `SURREAL_URL` — your SurrealDB Cloud endpoint, e.g. `wss://<instance>.cloud.surrealdb.com`
- `SURREAL_USER`, `SURREAL_PASS`
- `SURREAL_NS`, `SURREAL_DB`

Optional (OAuth; set all or none per provider):

- `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `GOOGLE_REDIRECT_URI`
- `MICROSOFT_CLIENT_ID`, `MICROSOFT_CLIENT_SECRET`
- `MICROSOFT_REDIRECT_URI`, `MICROSOFT_TENANT_ID` (defaults to `common`)
- `DISCORD_CLIENT_ID`, `DISCORD_CLIENT_SECRET`, `DISCORD_REDIRECT_URI`

Handled by the Dockerfile, no action needed:

- `PORT` — set by Hostless automatically; the CMD reads it.
- `LEPTOS_SITE_ROOT=/app/site`, `LEPTOS_ENV=PROD`, `LEPTOS_SITE_ADDR=0.0.0.0:$PORT`

Set the OAuth redirect URIs to `https://<your-hostless-domain>/...` and make
sure the domains registered with Google/Microsoft/Discord match, or sign-in
will fail with redirect_uri_mismatch.

## Which SurrealDB Cloud plan to pick

You said you'd handle the DB; one strong suggestion before you do. The app
keeps an open WS connection and fires cron queries on a timer. SurrealDB
**Earth Free** meters 100 aggregate database-active-hours/month and pauses
the instance at the limit; an always-on connection can hold that meter open
far longer than a normal web app would. The **Cloud "Start" free instance**
(1 GB storage, dedicated, no card, no request metering) fits this app much
better. Earth is fine for low-traffic with sparse queries, but the always-on
cron is the exact pattern that eats the Earth allowance.

Either way: export a `surreal export` dump before go-live and after any
schema change. Hostless gives you nothing to fall back on if the project is
paused (see "Failure modes" below).

## Deploy steps

1. Push this repo (with the Dockerfile) to GitHub/GitLab/Bitbucket.
2. In Hostless: New App -> your repo -> build from Dockerfile.
3. Set the env vars above. Don't set `PORT` yourself.
4. Deploy, watch the logs for `listening on http://0.0.0.0:...`.
5. Point migrations at the SurrealDB instance from your machine (this repo's
   `scripts/` and `migrations/` are how you seeded it locally).
6. Hit the domain, log in with an OAuth provider, check the mosque map loads.

## Failure modes to keep in mind

- Hostless is a ~3-person company, no SLA, no public status page. ClawCloud
  Run shipped the same product and vanished in May 2026 with 18 days' notice.
  Treat Hostless as a cheap place to run this, not as a home.
- Because the DB lives on SurrealDB Cloud, losing Hostless means redeploy
  elsewhere, not data loss. Keep the deploy reproducible (it is now) and the
  DB backed up.
- If the free allowance is exceeded, Hostless bills overage (or you pause
  apps). Set the account spending limit in Billing so nothing surprises you.

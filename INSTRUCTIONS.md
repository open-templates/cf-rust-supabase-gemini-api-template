# Agent & developer instructions — cf-rust-supabase-gemini-api-template

Use this file when turning this template into a **production API** on Cloudflare Workers. This repository is **self-contained**; pairing with [react-supabase-auth-ai-chat-template](https://github.com/open-templates/react-supabase-auth-ai-chat-template) is the recommended demo for auth + chat.

## What ships out of the box

| Endpoint | Auth | Purpose |
|----------|------|---------|
| `GET /health` | Public | Liveness check |
| `GET /me` | Bearer JWT | Current Supabase user |
| `POST /chat` | Bearer JWT | `{ "message": "..." }` → Gemini reply |
| `GET /chat?message=` | Bearer JWT | Query-string Gemini prompt |

Details: [`index.md`](index.md)

---

## Prerequisites (required before development)

### 1. Supabase project

1. Create a project at [supabase.com](https://supabase.com).
2. From **Settings → API**, copy:
   - **Project URL** → `SUPABASE_URL`
   - **anon public key** → `SUPABASE_ANON_KEY`
   - **service_role key** → `SUPABASE_SERVICE_ROLE_KEY` (server only; never commit or expose to browsers)

No custom database tables are required for the default `/me` endpoint (Supabase Auth only). Add migrations when you extend the API with Postgres data.

### 2. Cloudflare account (for deploy)

- [Cloudflare Workers](https://developers.cloudflare.com/workers/) account
- Rust toolchain (`rustup`) and Wrangler (`npm install -g wrangler` or `npx wrangler`)

Local development does not require Cloudflare login if you use `wrangler dev` with local secrets.

---

## Setup checklist

**Prerequisites:** Rust (`rustup`), Node.js (for Wrangler).

```bash
cp .env.example .dev.vars
# Edit .dev.vars with Supabase credentials and GEMINI_API_KEY
npx wrangler dev   # http://localhost:8787 — builds via worker-build on first run
```

Verify:

```bash
curl http://localhost:8787/health
curl -H "Authorization: Bearer <supabase-access-token>" http://localhost:8787/me
curl -X POST http://localhost:8787/chat \
  -H "Authorization: Bearer <supabase-access-token>" \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello Gemini"}'
```

Full guides: [`QUICKSTART.md`](QUICKSTART.md), [`SETUP.md`](SETUP.md), [`CLOUDFLARE_SETUP.md`](CLOUDFLARE_SETUP.md)

### Environment variables

| Variable | Required | Notes |
|----------|----------|-------|
| `SUPABASE_URL` | Yes | |
| `SUPABASE_ANON_KEY` | Yes | Used with user JWT |
| `SUPABASE_SERVICE_ROLE_KEY` | Yes | Admin operations only |
| `GEMINI_API_KEY` | Yes | [Google AI Studio](https://aistudio.google.com/apikey) — server only |
| `GEMINI_MODEL` | No | Default `gemini-2.5-flash` |
| `ALLOWED_ORIGINS` | Recommended | Comma-separated frontend origins for CORS |
| `ENVIRONMENT` | No | `development` / `staging` / `production` |

Production: set via `wrangler secret put` (see `CLOUDFLARE_SETUP.md`).

---

## Connecting the frontend (missing piece for full stack)

This worker does **not** include a UI. To test auth end-to-end:

1. Clone or fork **[react-supabase-auth-ai-chat-template](https://github.com/open-templates/react-supabase-auth-ai-chat-template)**.
2. Complete **its** `INSTRUCTIONS.md` (Supabase + Google OAuth + redirect URLs).
3. In the frontend `.env.local`:
   ```bash
   VITE_API_BASE_URL=http://localhost:8787   # or your deployed worker URL
   ```
4. Set worker `ALLOWED_ORIGINS` to the frontend origin (e.g. `http://localhost:5173`).
5. Run both: worker `npx wrangler dev`, frontend `bun run dev`.

Test chat from the frontend `/chat` route or via curl (see verify commands above).

---

## Agent workflow (extending to production)

Read in order:

1. **`INSTRUCTIONS.md`** (this file) — prerequisites and pairing
2. **`index.md`** — current API contract
3. **`.agents/skills/index.md`** — OKF module guides
4. **`.agents/skills/README.md`** — agent skill catalog

### Adding a new endpoint

1. **Serde model** (if validating input): `src/models.rs`
2. **Handler**: add async fn and register on `Router` in `src/lib.rs`
3. **Auth**: call `jwt::extract_bearer` + `supabase::get_user` before handler logic
4. **Document** in `index.md` and `specs/features/`
5. **Frontend** (if UI consumes it): add module in react template `src/api/`

### Rules

- Use `success()` / `error_response()` helpers in `src/lib.rs` for JSON envelopes.
- Verify users via Supabase `/auth/v1/user` with anon key + Bearer JWT.
- Never expose `SUPABASE_SERVICE_ROLE_KEY` or `GEMINI_API_KEY` to clients.
- Run `cargo check` before finishing.

---

## Repository map

```
src/lib.rs          Entry + middleware order
src/routes/           health.ts, me.ts (+ your routes)
src/middleware/       Auth, CORS, errors
src/lib/supabase.ts   Client factories
index.md     Feature specification
skills-lock.json        Pinned upstream Supabase agent skills
.agents/skills/         Project agent skills
.cursor/rules/        Cursor IDE rules
```

---

## Troubleshooting

| Issue | Check |
|-------|--------|
| `401` on `/me` | Valid Supabase access token; `SUPABASE_URL` / keys match frontend project |
| CORS errors | `ALLOWED_ORIGINS` includes exact frontend origin |
| Frontend shows API offline | Worker running; correct `VITE_API_BASE_URL` |

---

## License

MIT — see `LICENSE`.

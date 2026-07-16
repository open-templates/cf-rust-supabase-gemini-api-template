# cf-rust-supabase-gemini-api-template — Agent Skills Index

Skills teach agents how this **Rust workers-rs** API works and how to extend it safely.

## Project status (current template)

**Rust Worker + Supabase + Gemini** paired with **react-supabase-auth-ai-chat-template**:

| Endpoint | Auth | Purpose |
|----------|------|---------|
| `GET /health` | Public | Liveness check |
| `GET /me` | Bearer JWT | Current user via Supabase `/auth/v1/user` |
| `POST /chat` | Bearer JWT | Gemini chat completion |
| `GET /chat?message=` | Bearer JWT | Query alternative for quick tests |

Canonical OKF specs: [`index.md`](../../index.md) · OKF modules: [`.agents/skills/index.md`](index.md)

## OKF modules (local)

| Module | Use when |
|--------|----------|
| [auth-middleware](modules/auth-middleware.md) | JWT extraction and Supabase user lookup |
| [chat-route](modules/chat-route.md) | `/chat` handlers and request models |
| [gemini-client](modules/gemini-client.md) | Gemini REST API via `worker::Fetch` |

Shared concepts (synced): [shared/auth/](shared/auth/) · [shared/supabase/](shared/supabase/)

## Project layout

```
src/
├── lib.rs             # Router + route handlers
├── jwt.rs             # Bearer token parsing
├── supabase.rs        # Supabase auth HTTP client
├── gemini.rs          # Gemini generateContent client
└── models.rs          # Serde request/response types
Cargo.toml
wrangler.toml          # worker-build command
```

## Extension order

1. **Serde model** → `src/models.rs`
2. **Handler** → `src/lib.rs` (register on `Router`)
3. **Document** → `specs/features/` + `.agents/skills/modules/` + frontend `src/api/` module

## Commands

```bash
npx wrangler dev
npx wrangler deploy
```

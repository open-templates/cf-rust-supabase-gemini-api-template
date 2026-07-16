# cf-rust-supabase-gemini-api-template

Minimal Cloudflare Worker API built with **Rust** ([workers-rs](https://github.com/cloudflare/workers-rs)), **Supabase Auth**, and **Gemini** from [@open-templates](https://github.com/open-templates). Pairs with [react-supabase-auth-ai-chat-template](https://github.com/open-templates/react-supabase-auth-ai-chat-template).

Alternative to [cf-rust-supabase-gemini-api-template](https://github.com/open-templates/cf-rust-supabase-gemini-api-template) for teams preferring Rust.

## Quick start

**Prerequisites:** Rust toolchain (`rustup`), Node.js (for Wrangler).

```bash
./scripts/init-from-template.sh   # optional personalization
npx wrangler dev
```

Test:

```bash
curl http://localhost:8787/health
```

## Out-of-the-box features

| Endpoint | Auth | Description |
|----------|------|-------------|
| `GET /health` | Public | Liveness check |
| `GET /me` | Bearer JWT | Current Supabase user |
| `POST /chat` | Bearer JWT | Gemini chat (`message`, optional `history`) |
| `GET /chat?message=` | Bearer JWT | Query-string chat for quick tests |

See [`index.md`](index.md) for detailed behavior and extension guidance.

Agent docs: [INSTRUCTIONS.md](INSTRUCTIONS.md) · [.agents/skills/](.agents/skills/)

## Environment variables

| Variable | Required | Purpose |
|----------|----------|---------|
| `SUPABASE_URL` | Yes | Supabase project URL |
| `SUPABASE_ANON_KEY` | Yes | Anon key |
| `SUPABASE_SERVICE_ROLE_KEY` | Yes | Reserved for admin operations |
| `GEMINI_API_KEY` | Yes | Google AI Studio API key |
| `GEMINI_MODEL` | No | Default `gemini-2.5-flash` |
| `ALLOWED_ORIGINS` | No | Comma-separated CORS origins |

## License

MIT

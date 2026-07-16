---
type: Feature
title: Gemini integration
description: Gemini REST client and chat route handlers (Rust).
tags: [gemini, ai, rust]
timestamp: 2026-07-16T00:00:00Z
---

| Module | Role |
|--------|------|
| `src/gemini.rs` | Gemini `generateContent` via `worker::Fetch` |
| `src/lib.rs` | `POST /chat` and `GET /chat` handlers |

Never expose `GEMINI_API_KEY` to the browser.

See [.agents/skills/modules/gemini-client.md](../../.agents/skills/modules/gemini-client.md).

Alternative stack: [cf-hono-supabase-gemini-api-template](https://github.com/open-templates/cf-hono-supabase-gemini-api-template) (TypeScript/Hono + `@google/genai`).

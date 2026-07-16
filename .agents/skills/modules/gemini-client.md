# Module: Gemini REST client (Rust)

Call Google Gemini `generateContent` from a Worker using `worker::Fetch`.

## Pattern

1. Read `GEMINI_API_KEY` and optional `GEMINI_MODEL` from `Env`
2. Build `contents` array from chat history (`user` / `model` roles) + new user message
3. `POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key=...`
4. Parse `candidates[0].content.parts[0].text`

## Files

* `src/gemini.rs`
* `src/models.rs` — `ChatHistoryItem`, `ChatRequest`

## See also

* [chat-route](chat-route.md)
* Hono equivalent: [cf-hono-supabase-gemini-api-template `src/lib/gemini.ts`](https://github.com/open-templates/cf-hono-supabase-gemini-api-template)

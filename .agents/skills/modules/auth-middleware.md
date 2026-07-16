# Module: Rust auth gate

Recreate JWT validation used by `/me` and `/chat`.

## Pattern

1. `jwt::extract_bearer(req.headers().get("Authorization")?)`
2. `supabase::get_user(&ctx.env, &token).await?` — calls `GET /auth/v1/user`
3. Return `error_response("Unauthorized", "UNAUTHORIZED", 401)` when missing/invalid

## Files

* `src/jwt.rs`
* `src/supabase.rs`
* `src/lib.rs` — route handlers call auth before business logic

## See also

* [shared/auth/jwt-api-passthrough](../shared/auth/jwt-api-passthrough.md)
* Hono equivalent: [cf-hono-supabase-gemini-api-template `authentication` skill](https://github.com/open-templates/cf-hono-supabase-gemini-api-template)

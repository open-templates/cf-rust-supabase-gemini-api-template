use serde::Serialize;
use serde_json::{json, Value};
use worker::*;

mod gemini;
mod jwt;
mod models;
mod supabase;

pub fn success<T: Serialize>(data: T) -> Result<Response> {
    Response::from_json(&json!({ "success": true, "data": data }))
}

pub fn error_response(message: &str, code: &str, status: u16) -> Result<Response> {
    let mut response = Response::from_json(&json!({
        "success": false,
        "error": {
            "message": message,
            "code": code,
        }
    }))?;
    *response.status_mut() = status;
    Ok(response)
}

fn apply_cors(origin: Option<String>, mut response: Response, env: &Env) -> Result<Response> {
    let allowed = env
        .var("ALLOWED_ORIGINS")
        .map(|v| v.to_string())
        .unwrap_or_default();

    let allowed_origins: Vec<&str> = allowed
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let origin_value = origin.unwrap_or_default();
    let allow_origin = if allowed_origins.is_empty() {
        if origin_value.is_empty() {
            "*".to_string()
        } else {
            origin_value
        }
    } else if origin_value.is_empty() {
        allowed_origins[0].to_string()
    } else if allowed_origins.iter().any(|o| *o == origin_value) {
        origin_value
    } else {
        allowed_origins[0].to_string()
    };

    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &allow_origin)?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization, X-Requested-With",
    )?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    )?;
    Ok(response)
}

async fn health(_: Request, _: RouteContext<()>) -> Result<Response> {
    success(json!({
        "status": "healthy",
        "timestamp": Date::now().as_millis(),
    }))
}

async fn me(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let token = match jwt::extract_bearer(req.headers().get("Authorization")?) {
        Some(token) => token,
        None => return error_response("Unauthorized", "UNAUTHORIZED", 401),
    };

    let user = match supabase::get_user(&ctx.env, &token).await? {
        Some(user) => user,
        None => return error_response("Unauthorized", "UNAUTHORIZED", 401),
    };

    success(json!({
        "id": user.id,
        "email": user.email,
        "user_metadata": user.user_metadata,
        "app_metadata": user.app_metadata,
        "created_at": user.created_at,
    }))
}

async fn chat_post(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let token = match jwt::extract_bearer(req.headers().get("Authorization")?) {
        Some(token) => token,
        None => return error_response("Unauthorized", "UNAUTHORIZED", 401),
    };

    if supabase::get_user(&ctx.env, &token).await?.is_none() {
        return error_response("Unauthorized", "UNAUTHORIZED", 401);
    }

    let body: models::ChatRequest = match req.json().await {
        Ok(body) => body,
        Err(_) => return error_response("Invalid JSON body", "BAD_REQUEST", 400),
    };

    if body.message.trim().is_empty() {
        return error_response("message is required", "BAD_REQUEST", 400);
    }

    if body.message.len() > 10_000 {
        return error_response("message must be at most 10000 characters", "BAD_REQUEST", 400);
    }

    handle_chat(&ctx.env, body.message, body.history.unwrap_or_default()).await
}

async fn chat_get(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let token = match jwt::extract_bearer(req.headers().get("Authorization")?) {
        Some(token) => token,
        None => return error_response("Unauthorized", "UNAUTHORIZED", 401),
    };

    if supabase::get_user(&ctx.env, &token).await?.is_none() {
        return error_response("Unauthorized", "UNAUTHORIZED", 401);
    }

    let url = req.url()?;
    let message = url
        .query_pairs()
        .find(|(key, _)| key == "message")
        .map(|(_, value)| value.trim().to_string())
        .unwrap_or_default();

    if message.is_empty() {
        return error_response("message query parameter is required", "BAD_REQUEST", 400);
    }

    if message.len() > 10_000 {
        return error_response("message must be at most 10000 characters", "BAD_REQUEST", 400);
    }

    handle_chat(&ctx.env, message, Vec::new()).await
}

async fn handle_chat(
    env: &Env,
    message: String,
    history: Vec<models::ChatHistoryItem>,
) -> Result<Response> {
    let model = env
        .var("GEMINI_MODEL")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "gemini-2.5-flash".to_string());

    match gemini::generate_reply(env, &message, &history, &model).await {
        Ok(reply) => success(json!({
            "message": message,
            "reply": reply,
            "model": model,
        })),
        Err(err) if err.contains("GEMINI_API_KEY") => {
            error_response("AI provider is not configured", "INTERNAL_SERVER_ERROR", 500)
        }
        Err(err) => error_response(&err, "INTERNAL_SERVER_ERROR", 502),
    }
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let origin = req.headers().get("Origin")?;

    if req.method() == Method::Options {
        let response = Response::from_json(&json!({ "success": true, "data": Value::Null }))?;
        return apply_cors(origin, response, &env);
    }

    let router = Router::new()
        .get_async("/health", health)
        .get_async("/me", me)
        .post_async("/chat", chat_post)
        .get_async("/chat", chat_get);

    let response = router.run(req, env.clone()).await?;
    apply_cors(origin, response, &env)
}

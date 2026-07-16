use crate::models::SupabaseUser;
use serde_json::Value;
use worker::*;

pub async fn get_user(env: &Env, token: &str) -> Result<Option<SupabaseUser>> {
    let supabase_url = env.var("SUPABASE_URL")?.to_string();
    let anon_key = env.var("SUPABASE_ANON_KEY")?.to_string();
    let url = format!("{}/auth/v1/user", supabase_url.trim_end_matches('/'));

    let mut headers = Headers::new();
    headers.set("apikey", &anon_key)?;
    headers.set("Authorization", &format!("Bearer {token}"))?;

    let request = Request::new_with_init(
        &url,
        RequestInit::new()
            .with_method(Method::Get)
            .with_headers(headers),
    )?;

    let mut response = Fetch::Request(request).send().await?;
    if response.status_code() != 200 {
        return Ok(None);
    }

    let value: Value = response.json().await?;
    let user: SupabaseUser = serde_json::from_value(value).map_err(|err| Error::RustError(err.to_string()))?;
    Ok(Some(user))
}

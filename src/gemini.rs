use crate::models::ChatHistoryItem;
use serde_json::{json, Value};
use worker::*;

const DEFAULT_MODEL: &str = "gemini-2.5-flash";

pub async fn generate_reply(
    env: &Env,
    message: &str,
    history: &[ChatHistoryItem],
    model: &str,
) -> std::result::Result<String, String> {
    let api_key = env
        .var("GEMINI_API_KEY")
        .map(|v| v.to_string())
        .map_err(|_| "GEMINI_API_KEY is not configured".to_string())?;

    let model_name = if model.is_empty() { DEFAULT_MODEL } else { model };
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, api_key
    );

    let mut contents: Vec<Value> = history
        .iter()
        .map(|item| {
            json!({
                "role": if item.role == "assistant" { "model" } else { "user" },
                "parts": [{ "text": item.content }],
            })
        })
        .collect();

    contents.push(json!({
        "role": "user",
        "parts": [{ "text": message }],
    }));

    let body = json!({ "contents": contents });

    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    let request = Request::new_with_init(
        &url,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(body.to_string().into())),
    )
    .map_err(|err| err.to_string())?;

    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.status_code() >= 400 {
        let text = response.text().await.unwrap_or_default();
        return Err(format!("Gemini request failed: {text}"));
    }

    let payload: Value = response.json().await.map_err(|err| err.to_string())?;
    let text = payload["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(str::to_string)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "Gemini returned an empty response".to_string())?;

    Ok(text)
}

pub fn extract_bearer(header: Option<String>) -> Option<String> {
    let header = header?;
    let mut parts = header.splitn(2, ' ');
    let scheme = parts.next()?;
    let token = parts.next()?.trim();

    if scheme != "Bearer" || token.is_empty() {
        return None;
    }

    Some(token.to_string())
}

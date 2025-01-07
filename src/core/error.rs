use std::collections::HashMap;
use axum::http::StatusCode;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn format_error(key: &str, messages: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();

    errors.insert(key.to_string(), messages);

    errors
}

use axum::{
    Json, 
    http::HeaderMap,
    response::Response,
    body::Body
};
use serde::{Deserialize, Serialize};
use std::fs;
use axum::http::{header, StatusCode};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct ErrorResponse {
    pub error: String
}

pub async fn serve_file(path: String, content_type: &str) -> Response<Body> {
    Response::builder()
        .header("content-type", content_type)
        .body(Body::from(fs::read(path).expect("failed to read file")))
        .unwrap()
}

pub fn camel_case(s: &str) -> String {
    let mut words = s.split_whitespace();

    let first = words.next().unwrap_or("").to_lowercase();

    first + &words
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

pub fn check_auth(
    headers: &HeaderMap,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let expected = "Bearer Kyqc49jIM+5+D0Sed8ZQ671gxkd7W/bBTWjDtZ0Zrgk=";

    if headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        != Some(expected)
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "unauthorized".to_string(),
            }),
        ));
    }

    Ok(())
}
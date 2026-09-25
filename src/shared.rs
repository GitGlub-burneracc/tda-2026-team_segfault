use axum::{
    body::Body,
    response::Response,
};
use std::fs;

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
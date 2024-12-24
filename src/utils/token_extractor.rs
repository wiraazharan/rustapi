use actix_web::HttpRequest;
use serde_json::Value;

pub fn extract_token(req: &HttpRequest, body: Option<&Value>) -> Option<String> {
    // 1. Check Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string()); // Extract token after "Bearer "
            }
        }
    }

    // 2. Check query string
    if let Some(query_token) = req.query_string().split('&').find_map(|pair| {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == "token" {
                return Some(value.to_string());
            }
        }
        None
    }) {
        return Some(query_token);
    }

    // 3. Check request body
    if let Some(body) = body {
        if let Some(token) = body.get("token").and_then(|v| v.as_str()) {
            return Some(token.to_string());
        }
    }

    None // Token not found
}

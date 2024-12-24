use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use bytes::BytesMut; // Correct import
use futures::future::{ok, LocalBoxFuture, Ready};
use futures_util::StreamExt;
use serde_json::Value;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct TokenExtractor;

impl<S, B> Transform<S, ServiceRequest> for TokenExtractor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TokenExtractorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TokenExtractorMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct TokenExtractorMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TokenExtractorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // Extract token
            let token = extract_token(&mut req).await;

            // Store token in request extensions
            if let Some(token_value) = token {
                req.extensions_mut().insert(token_value);
            }

            svc.call(req).await
        })
    }
}

// Async function to extract token
async fn extract_token(req: &mut ServiceRequest) -> Option<String> {
    // 1. Check Authorization header for Bearer token
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str.trim_start_matches("Bearer ").to_string());
            }
        }
    }

    // 2. Check query string for token
    if let Some(query_token) = req.query_string()
        .split('&')
        .find_map(|param| {
            let mut split = param.split('=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                if key == "token" {
                    return Some(value.to_string());
                }
            }
            None
        }) {
        return Some(query_token);
    }

    // 3. Check body parameters for token
    let mut payload = req.take_payload();
    let mut body_bytes = BytesMut::new(); // Use BytesMut correctly

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(bytes) => body_bytes.extend_from_slice(&bytes),
            Err(_) => return None, // Return None if body parsing fails
        }
    }

    if let Ok(json) = serde_json::from_slice::<Value>(&body_bytes) {
        if let Some(body_token) = json.get("token").and_then(|v| v.as_str()) {
            return Some(body_token.to_string());
        }
    }

    None
}

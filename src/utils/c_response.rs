use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct CResponse<T> {
    pub status: Option<String>,
    pub message: Option<String>,
    pub response_code: i32,
    pub description: Option<String>,
    pub app_version: String,
    pub token: Option<String>,
    pub talk_to_server_before: Option<String>,
    pub refresh_token_before: Option<String>,
    pub refresh_token_after: Option<String>,
    pub breakdown_errors: Option<String>, // Placeholder for breakdown errors
    pub preferred_language_code: Option<String>,
    pub data: Option<T>, // Generic type for response data
}

impl<T> CResponse<T>
where
    T: Serialize,
{
    pub fn new(
        status: Option<String>,
        message: Option<String>,
        code: i32,
        description: Option<String>,
        data: Option<T>,
        token: Option<String>,
        talk_to_server_before: Option<String>,
        refresh_token_before: Option<String>,
        refresh_token_after: Option<String>,
        breakdown_errors: Option<String>,
        preferred_language_code: Option<String>,
    ) -> Self {
        CResponse {
            status,
            message,
            response_code: code,
            description,
            app_version: "1.0.0".to_string(), // Replace with actual app version
            token,
            talk_to_server_before,
            refresh_token_before,
            refresh_token_after,
            breakdown_errors,
            preferred_language_code,
            data,
        }
    }

    // Utility function to build an Actix-web response
    pub fn as_http_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

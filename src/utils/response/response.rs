use actix_web::HttpResponse;
use serde::Serialize;
use crate::utils::response::response_code::ResponseCode;

#[derive(Serialize)]
pub struct StandardResponse<T> {
    status: Option<String>,
    message: Option<String>,
    response_code: u16,
    description: String,
    app_version: String,
    data: Option<T>,
    breakdown_errors: Option<serde_json::Value>,
    preferred_language_code: Option<String>,
    token: Option<String>,
}

pub fn create_response<T>(
    status: Option<String>,
    message: Option<String>,
    response_code: ResponseCode,
    data: Option<T>,
    token: Option<String>,
    breakdown_errors: Option<serde_json::Value>,
    preferred_language_code: Option<String>,
) -> HttpResponse
where
    T: Serialize,
{
    let response = StandardResponse {
        status,
        message,
        response_code: response_code.to_code(),
        description: response_code.to_description().to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        data,
        breakdown_errors,
        preferred_language_code,
        token,
    };

    HttpResponse::Ok().json(response)
}

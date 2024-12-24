use std::collections::HashMap;
use serde::Serialize; // Required for serialization of the ApiResponse struct

#[derive(Debug, Clone, Copy)]
pub enum ResponseCode {
    Error = 2111,
    Success = 2100,
    GeneralError = 2222,
    DbQueryError = 3333,
    NotFound = 4444,
    NotAuthorized = 5555,
    JwtTokenInvalid = 6666,
    JwtTokenExpired = 7777,
    JwtTokenNotFound = 8888,
    MaintenanceMode = 9999,
    ValidationError = 10005,
    InvalidLoginCredentials = 10011,
    // Add more response codes as needed...
}

impl ResponseCode {
    pub fn description(&self) -> &'static str {
        match self {
            ResponseCode::Error => "Error",
            ResponseCode::Success => "Success",
            ResponseCode::GeneralError => "General Error",
            ResponseCode::DbQueryError => "Database Query Error",
            ResponseCode::NotFound => "Not Found",
            ResponseCode::NotAuthorized => "Not Authorized",
            ResponseCode::JwtTokenInvalid => "JWT Token Invalid",
            ResponseCode::JwtTokenExpired => "JWT Token Expired",
            ResponseCode::JwtTokenNotFound => "JWT Token Not Found",
            ResponseCode::MaintenanceMode => "Maintenance Mode",
            ResponseCode::ValidationError => "Validation Error",
            ResponseCode::InvalidLoginCredentials => "Invalid Login Credentials",
            // Add more descriptions as needed...
        }
    }

    // Get all constants as a HashMap (if needed)
    pub fn all() -> HashMap<i32, &'static str> {
        let mut map = HashMap::new();
        map.insert(ResponseCode::Error as i32, "Error");
        map.insert(ResponseCode::Success as i32, "Success");
        map.insert(ResponseCode::GeneralError as i32, "General Error");
        map.insert(ResponseCode::DbQueryError as i32, "Database Query Error");
        map.insert(ResponseCode::NotFound as i32, "Not Found");
        map.insert(ResponseCode::NotAuthorized as i32, "Not Authorized");
        map.insert(ResponseCode::JwtTokenInvalid as i32, "JWT Token Invalid");
        map.insert(ResponseCode::JwtTokenExpired as i32, "JWT Token Expired");
        map.insert(ResponseCode::JwtTokenNotFound as i32, "JWT Token Not Found");
        map.insert(ResponseCode::MaintenanceMode as i32, "Maintenance Mode");
        map.insert(ResponseCode::ValidationError as i32, "Validation Error");
        map.insert(ResponseCode::InvalidLoginCredentials as i32, "Invalid Login Credentials");
        // Add more as needed...
        map
    }
}

// Add the ApiResponse struct
#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    pub code: ResponseCode,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: ResponseCode, data: Option<T>) -> Self {
        Self {
            code,
            message: Some(code.description().to_string()),
            data,
        }
    }

    pub fn as_http_response(self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .json(self)
    }
}
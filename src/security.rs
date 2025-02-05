use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    response::{IntoResponse, Response},
    middleware::Next,
};

use crate::CONFIG;

// Middleware function to validate the API key
pub async fn validate_api_key(req: Request<Body>, next: Next) -> Result<Response, Response> {
    // Check for the API key in the 'Authorization' header with the Bearer scheme
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_value) = auth_header.to_str() {
            if let Some(api_key) = auth_value.strip_prefix("ApiKey ") {
                if api_key == CONFIG.api_key {
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    // Return 401 Unauthorized if the key is missing or invalid
    Err((StatusCode::UNAUTHORIZED, "Unauthorized").into_response())
}

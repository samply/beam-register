use crate::beam_app::{BeamApp, BeamAppError};
use axum::{http::StatusCode, Json, Router, response::IntoResponse};

// Function to set up routes
pub fn create_router() -> Router {
    Router::new()
        .route("/beam-app", axum::routing::post(post_beam_app))
        .route("/beam-app", axum::routing::delete(delete_beam_app))
}

// POST /beam-app handler
async fn post_beam_app(Json(payload): Json<BeamApp>) -> Result<String, axum::response::Response> {
    match payload.validate() {
        Ok(_) => Ok(format!("Received POST with beam_id: {}", payload.beam_id.unwrap())),
        Err(e) => {
            let (status, message) = handle_error(e);
            Err(send_error_response(status, message)) // Call the function to send the error response with message
        }
    }
}

// DELETE /beam-app handler
async fn delete_beam_app(Json(payload): Json<BeamApp>) -> Result<String, axum::response::Response> {
    match payload.is_valid_beam_id() {
        Ok(_) => Ok(format!("Received DELETE with beam_id: {}", payload.beam_id.unwrap())),
        Err(e) => {
            let (status, message) = handle_error(e);
            Err(send_error_response(status, message)) // Call the function to send the error response with message
        }
    }
}

// Helper function to map BeamAppError to StatusCode and message
fn handle_error(error: BeamAppError) -> (StatusCode, String) {
    match error {
        BeamAppError::MissingBeamId => (
            StatusCode::BAD_REQUEST,
            "beam_id is missing".to_string(),
        ),
        BeamAppError::MissingBeamSecret => (
            StatusCode::BAD_REQUEST,
            "beam_secret is missing or empty".to_string(),
        ),
        BeamAppError::InvalidBeamIdFormat => (
            StatusCode::BAD_REQUEST,
            "beam_id contains invalid characters (only alphanumeric allowed)".to_string(),
        ),
    }
}

// Helper function to create a response with both status and error message
fn send_error_response(status: StatusCode, message: String) -> axum::response::Response {
    (status, Json(message)).into_response() // Return the response with the status and the message in JSON format
}
